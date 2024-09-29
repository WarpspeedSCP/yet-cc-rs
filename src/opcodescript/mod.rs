//! Most of this module is adapted from MrChubb's yetireg_tools repo.
//! Changes are mostly to make things more explicit,
//! and to add functionality such as converting from and to opcodescript files easily.

use std::ops::ControlFlow;

use opcodes::{BinarySerialize, InsertOpcode};
use serde::{Deserialize, Serialize};

mod opcode_impl;
mod opcodes;

pub use opcodes::Opcode;

use encoding_rs::SHIFT_JIS;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_vec")]
  pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footer {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_vec")]
  pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
  pub header: Header,
  pub opcodes: Vec<Opcode>,
  pub footer: Footer,
}

impl Script {
  pub fn new(data: &[u8]) -> (Self, Option<String>) {
    let start = crate::util::transmute_to_u32(0, &data) as usize;

    let mut address = start;

    let header = Header {
      bytes: data[0..start].to_owned(),
    };

    let mut opcodes = Vec::new();

    let mut _idx = 0;
    let mut at_end = false;
    let mut encountered_error: Option<String> = None;
    while address < data.len() {
      match Opcode::eat(address, &data) {
        Ok(opcode) => {
          // simple check for end.
          if opcode.opcode() == 0x05 && [0x00, 0x05].contains(&data[address + 1]) {
            at_end = true;
          }

          address += opcode.size();
          opcodes.push(opcode);
          _idx += 1;

          if at_end {
            break;
          }
        }
        Err(e) => {
          encountered_error = Some(e);
          break;
        }
      }
    }

    let last_index = opcodes
      .last()
      .map(|entry| entry.address() + entry.size() as u32)
      .unwrap() as usize;

    let footer = Footer {
      bytes: data[last_index..].to_owned(),
    };

    let script = Script {
      header,
      footer,
      opcodes,
    };

    (script, encountered_error)
  }

  pub fn binary_serialize(&self) -> Vec<u8> {
    let mut output = vec![];

    output.extend(&self.header.bytes);

    let mut jump_correction = 0i64;
    for opcode in self.opcodes.iter().cloned() {
      let opcode = match adjust_single_opcode(opcode, &mut jump_correction) {
        Some(value) => value,
        None => continue, // means we've got a bad opcode.
      };

      let serialized = if let Opcode::OP_Insert(insert) = &opcode {
        let mut contents = Vec::new();
        for (idx, opcode) in insert.contents.iter().enumerate() {
          if let Opcode::OP_CUSTOM_TIP_77(custom) = opcode {
            let mut serialized = custom.binary_serialize();
            let mut offset: u16 = 4;
            for i in 1.. (custom.skip + 1) {
              let curr_opcode = insert.contents.get(idx + i as usize).cloned().unwrap();
              let curr_offset = curr_opcode.size() + match &curr_opcode {
                Opcode::OP_45(op) | Opcode::OP_85(op) | Opcode::OP_86(op) => {
                  SHIFT_JIS.encode(&op.unicode).0.len() + 1
                }
                Opcode::OP_47(str_op) => {
                  SHIFT_JIS.encode(&str_op.unicode).0.len() + 1
                }
                _ => 0
              };
              offset += curr_offset as u16;
            }

            log::info!("Skipping 0x{offset:04X} bytes; equivalent to {} instructions.", custom.skip);
            let offset_bytes=  offset.to_le_bytes();
            serialized[2] = offset_bytes[0];
            serialized[3] = offset_bytes[1];
            log::info!("Generated tip opcode: {:02X} {:02X} {:02X} {:02X}", serialized[0], serialized[1], serialized[2], serialized[3]);
            contents.extend(serialized);
          } else {
            contents.extend(opcode.binary_serialize());
          }
        }
        contents
      } else {
        opcode.binary_serialize()
      };

      output.extend(serialized)
    }
    output.extend(&self.footer.bytes);

    output
  }

  fn adjust_jump_instruction(
    jump_correction: i64,
    address: u32,
    jump_address: u32,
    log_things: bool,
  ) -> Option<i64> {
    let new_address = jump_address as i64 + jump_correction;
    if new_address < 0 || new_address > u32::MAX.into() {
      log::error!(
        "jump address 0x{:08X} became negative due to correction {jump_correction}",
        jump_address
      );
      return None;
    }
    if new_address != jump_address.into() && log_things {
      log::debug!(
        "jump address adjustment at address 0x{:08X}; old: 0x{:08X}, new: 0x{:08X}",
        address,
        jump_address,
        new_address
      );
    }
    Some(new_address)
  }

  fn calculate_jump_adjustment(unicode: &str, jis_len: usize) -> i64 {
    let (sjis_encoded, _, _) = SHIFT_JIS.encode(unicode);
    let diff = (sjis_encoded.len() as i64 + 1) - jis_len as i64;
    diff
  }
}

fn adjust_single_opcode(opcode: Opcode, jump_correction: &mut i64) -> Option<Opcode> {
  let opcode = match opcode {
    // pass it on, we already account for everything here.
    Opcode::OP_CUSTOM_TIP_77(custom) => Opcode::OP_CUSTOM_TIP_77(custom),
    Opcode::OP_31(op) | Opcode::OP_32(op) => {
      log::debug!("Choice opcode at {:08X}", op.address);
      for (idx, choice) in op.choices.iter().enumerate() {
        let diff = Script::calculate_jump_adjustment(&choice.unicode, choice.sjis_bytes.len());
        log::debug!(
          "offset difference for choice {} at address 0x{:08X} is {diff}",
          idx,
          choice.address
        );
        *jump_correction += diff;
      }
      Opcode::OP_31(op)
    }

    Opcode::OP_45(op) | Opcode::OP_85(op) | Opcode::OP_86(op) => {
      let diff = Script::calculate_jump_adjustment(&op.unicode, op.sjis_bytes.len());
      log::debug!(
        "offset difference for string ({:02X}) at address 0x{:08X} is {diff}",
        op.opcode,
        op.address
      );
      *jump_correction += diff;
      op.into()
    }

    Opcode::OP_47(op) => {
      let diff = Script::calculate_jump_adjustment(&op.unicode, op.sjis_bytes.len());
      log::debug!(
        "offset difference for string (47) at address 0x{:08X} is {diff}",
        op.address
      );
      *jump_correction += diff;
      Opcode::OP_47(op)
    }

    Opcode::JNE(mut op)
    | Opcode::JE(mut op)
    | Opcode::JLE(mut op)
    | Opcode::JL(mut op)
    | Opcode::JGE(mut op)
    | Opcode::JG(mut op) => {
      let new_address = match Script::adjust_jump_instruction(
        *jump_correction,
        op.address,
        op.jump_address,
        true,
      ) {
        Some(value) => value,
        None => {
          log::error!(
            "Found a bad conditional jump opcode at 0x{:08X}",
            op.address
          );
          return None;
        }
      };
      op.jump_address = new_address as u32;
      op.into()
    }

    Opcode::JNZ(mut op) | Opcode::JZ(mut op) => {
      if let ControlFlow::Break(_) = do_jump_correction_for_cond_jump(*jump_correction, &mut op) {
        log::error!("Found a bad JNZ/JZ opcode at 0x{:08X}.", op.address);
        return None;
      }
      op.into()
    }

    Opcode::Switch(mut op) => {
      do_jump_correction_for_switch(&mut op, *jump_correction);
      Opcode::Switch(op)
    }
    Opcode::OP_Insert(ins_opcode) => {
      let mut res: Vec<_> = vec![];

      for opcode in ins_opcode.contents {
        let adjustment = adjust_single_opcode(opcode, jump_correction).inspect(|it| {
          log::debug!(
            "Adding offset of 0x{:08X} to jump correction (0x{jump_correction:08X})",
            it.size()
          );
          *jump_correction += it.size() as i64
        });

        res.extend(adjustment.into_iter());
      }

      Opcode::OP_Insert(InsertOpcode { contents: res })
    }
    _ => opcode,
  };
  Some(opcode)
}

fn do_jump_correction_for_cond_jump(
  jump_correction: i64,
  op: &mut opcodes::JumpOpcode2,
) -> ControlFlow<()> {
  let new_address =
    match Script::adjust_jump_instruction(jump_correction, op.address, op.jump_address, true) {
      Some(value) => value,
      None => return ControlFlow::Break(()),
    };
  op.jump_address = new_address as u32;
  ControlFlow::Continue(())
}

fn do_jump_correction_for_switch(op: &mut opcodes::SwitchOpcode, jump_correction: i64) {
  for (idx, arm) in op.arms.iter_mut().enumerate() {
    let new_address =
      match Script::adjust_jump_instruction(jump_correction, op.address, arm.jump_address, false) {
        Some(value) => value,
        None => continue,
      };
    if new_address < 0 || new_address > u32::MAX.into() {
      log::error!(
        "jump address 0x{:08X} for switch arm {} became negative due to correction {jump_correction}",
        arm.jump_address, idx
      );
      continue;
    }
    if new_address != arm.jump_address.into() {
      log::debug!("switch jump address adjustment at address 0x{:08X} (arm {idx:02X}); old: 0x{:08X}, new: 0x{:08X}", op.address, arm.jump_address, new_address);
    }
    arm.jump_address = new_address as u32;
  }
}

#[cfg(test)]
mod tests {
  use crate::opcodescript::Script;

  #[test]
  fn test_thing() {
    let thing = include_str!("../../sn.bin.output/0046.yaml");

    let res: Script = serde_yml::from_str(thing).unwrap();

    res.binary_serialize();
  }
}