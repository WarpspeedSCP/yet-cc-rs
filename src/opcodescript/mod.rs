//! Most of this module is adapted from MrChubb's yetireg_tools repo.
//! Changes are mostly to make things more explicit,
//! and to add functionality such as converting from and to opcodescript files easily.

use std::collections::HashMap;

use opcodes::{BinarySerialize, InsertOpcode};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator};
use serde::{Deserialize, Serialize};

mod opcode_impl;
mod opcodes;

pub use opcodes::Opcode;

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
    let mut opcodes = self.opcodes.clone();
    output.extend(&self.header.bytes);

    // get jump addresses for everything first.
    let mut jump_map: HashMap<u32, HashMap<u16, usize>> = HashMap::new();
    let mut actual_address = self
      .opcodes
      .first()
      .map(Opcode::address)
      .unwrap_or_default() as usize;

    for opcode in opcodes.iter_mut() {
      match opcode {
        Opcode::OP_01(op) => {
          let mut map = HashMap::new();
          map.insert(
            0,
            self
              .opcodes
              .par_iter()
              .position_any(|it| it.address() == op.jump_address)
              .unwrap(),
          );
          jump_map.insert(op.address, map);
        }
        Opcode::JNE(op)
        | Opcode::JE(op)
        | Opcode::JLE(op)
        | Opcode::JL(op)
        | Opcode::JGE(op)
        | Opcode::JG(op) => {
          let mut map = HashMap::new();
          let thing = self
            .opcodes
            .par_iter()
            .position_any(|it| it.address() == op.jump_address)
            .unwrap();
          map.insert(0, thing);
          jump_map.insert(op.address, map);
        }
        Opcode::JZ(op) | Opcode::JNZ(op) => {
          let mut map = HashMap::new();
          map.insert(
            0,
            self
              .opcodes
              .par_iter()
              .position_any(|it| it.address() == op.jump_address)
              .unwrap(),
          );
          jump_map.insert(op.address, map);
        }
        Opcode::Switch(op) => {
          let jumps = op
            .arms
            .iter()
            .map(|arm| {
              (
                arm.index,
                self
                  .opcodes
                  .par_iter()
                  .position_any(|it| it.address() == arm.jump_address)
                  .unwrap(),
              )
            })
            .collect();
          jump_map.insert(op.address, jumps);
        }
        _ => {}
      }
      opcode.set_actual_address(actual_address);
      actual_address += opcode.size();
    }

    for opcode in opcodes.iter().cloned() {
      let opcode = match adjust_single_opcode(opcode, &jump_map, &opcodes) {
        Some(value) => value,
        None => continue, // means we've got a bad opcode.
      };

      let serialized = match &opcode {
        Opcode::OP_Insert(insert) => {
          let mut contents = Vec::new();
          for (idx, opcode) in insert.contents.iter().enumerate() {
            if let Opcode::OP_CUSTOM_TIP_77(custom) = opcode {
              let mut serialized = custom.binary_serialize();
              let mut offset: u16 = 4;
              for i in 1..(custom.skip + 1) {
                let curr_opcode = insert.contents.get(idx + i as usize).cloned().unwrap();
                let curr_offset = curr_opcode.size();
                offset += curr_offset as u16;
              }

              log::info!(
                "Encoding skip of 0x{offset:04X} bytes; equivalent to {} instructions.",
                custom.skip
              );

              let offset_bytes = offset.to_le_bytes();
              serialized[2..].copy_from_slice(&offset_bytes);

              log::info!(
                "Generated tip opcode: {:02X} {:02X} {:02X} {:02X}",
                serialized[0],
                serialized[1],
                serialized[2],
                serialized[3]
              );
              contents.extend(serialized);
            } else {
              contents.extend(opcode.binary_serialize());
            }
          }
          contents
        }
        _ => opcode.binary_serialize(),
      };

      actual_address += serialized.len();
      output.extend(serialized);
    }
    output.extend(&self.footer.bytes);

    output
  }
}

fn adjust_single_opcode(
  opcode: Opcode,
  jump_table: &HashMap<u32, HashMap<u16, usize>>,
  opcodes: &[Opcode],
) -> Option<Opcode> {
  let opcode = match opcode {
    Opcode::OP_01(mut op) => {
      let tbl_entry = &jump_table[&op.address];
      op.jump_address = opcodes[tbl_entry[&0]].actual_address();
      Opcode::OP_01(op)
    }

    Opcode::JNE(mut op)
    | Opcode::JE(mut op)
    | Opcode::JLE(mut op)
    | Opcode::JL(mut op)
    | Opcode::JGE(mut op)
    | Opcode::JG(mut op) => {
      let tbl_entry = &jump_table[&op.address];
      op.jump_address = opcodes[tbl_entry[&0]].actual_address();
      op.into()
    }

    Opcode::JNZ(mut op) | Opcode::JZ(mut op) => {
      let tbl_entry = &jump_table[&op.address];
      op.jump_address = opcodes[tbl_entry[&0]].actual_address();
      op.into()
    }

    Opcode::Switch(mut op) => {
      for branch in op.arms.iter_mut() {
        let tbl_entry = &jump_table[&op.address];
        branch.jump_address = opcodes[tbl_entry[&branch.index]].actual_address()
      }
      Opcode::Switch(op)
    }
    Opcode::OP_Insert(ins_opcode) => {
      let mut res: Vec<_> = vec![];
      for opcode in ins_opcode.contents.into_iter() {
        let adjustment = adjust_single_opcode(opcode, jump_table, opcodes).unwrap();

        res.push(adjustment);
      }

      Opcode::OP_Insert(InsertOpcode { contents: res })
    }
    opcode @ _ => opcode,
  };

  Some(opcode)
}

#[cfg(test)]
mod tests {
  use crate::opcodescript::Script;

  #[test]
  fn test_thing() {
    let thing = include_str!("../../sn.bin.output/0000.yaml");

    let res: Script = serde_yml::from_str(thing).unwrap();

    res.binary_serialize();
  }
}
