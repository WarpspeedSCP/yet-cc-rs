//! Most of this module is adapted from MrChubb's yetireg_tools repo.
//! Changes are mostly to make things more explicit,
//! and to add functionality such as converting from and to opcodescript files easily.

use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Error, Result};
use opcodes::{BinarySerialize, Custom77, InsertOpcode};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator};
use serde::{Deserialize, Serialize};

mod opcode_impl;
mod opcodes;

use crate::util::OkWrappable;
pub use opcode_impl::Quirks;
pub use opcodes::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Header {
	#[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_vec")]
	pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Footer {
	#[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_vec")]
	pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Script {
	pub header: Header,
	pub opcodes: Vec<Opcode>,
	pub footer: Footer,
}

impl Script {
	pub fn new(data: &[u8], quirks: Quirks) -> Result<(Self, Option<Error>)> {
		let start = crate::util::transmute_to_u32(0, &data)? as usize;

		let mut address = start;

		let header = Header {
			bytes: data[0..start].to_owned(),
		};

		let mut opcodes = Vec::new();

		let mut idx = 0;
		let mut at_end = false;
		let mut encountered_error: Option<anyhow::Error> = None;

		let mut skip_stack: Vec<(Custom77, usize)> = vec![];
		let mut marked_indices = HashSet::new();

		while address < data.len() {
			if address == 0x59B && data[address] == 0x07 {
				log::info!("ldke;wjkr;wejr;w00");
			}
			match Opcode::eat(address, &data, quirks) {
				Ok(opcode) => {
					// simple check for end.
					if opcode.opcode() == 0x05
						&& ([0x00, 0x05].contains(&data[address + 1])
							|| 0x02
								== opcodes
									.last()
									.map(|it: &Opcode| it.opcode())
									.unwrap_or_default()
							|| (data.len() - address) < 0x30)
					{
						at_end = true;
					}

					address += opcode.size();

					if opcode.opcode() == 0x77 {
						let mut op: Custom77 = opcode.clone().try_into().unwrap();
						if let Some(res) = op.skip_bytes.checked_sub(3) {
							op.skip_bytes = res;
						} else {
							encountered_error = Some(anyhow!(
								"invalid skip offset for opcode 77 at address 0x{address:08X}."
							));
							break;
						}
						skip_stack.push((op, idx));
					}

					idx += 1;

					if !skip_stack.is_empty() {
						let size = opcode.size();

						for (op, this_idx) in skip_stack.iter_mut() {
							if op
								.skip_bytes
								.checked_sub(size as u16)
								.map_or(false, |_| true)
							{
								op.skip += 1;
								op.skip_bytes -= size as u16
							} else {
								marked_indices.insert(*this_idx);
							}
						}

						for i in marked_indices.drain() {
							let thing = skip_stack
								.iter()
								.find(|it| it.1 == i)
								.map(|(a, b)| (a.clone(), *b))
								.unwrap();

							match opcodes.get_mut(thing.1).unwrap() {
								Opcode::OP_CUSTOM_TIP_77(opcode) => {
									opcode.skip = thing.0.skip;
								}
								_ => {}
							}

							skip_stack.retain(|it| it.1 != i);
						}
					}

					opcodes.push(opcode);

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

		(script, encountered_error).wrap_ok()
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
		log::debug!("Actual address start is 0x{actual_address:08X}");
		for opcode in opcodes.iter_mut() {
			match opcode {
				Opcode::OP_DIRECT_JUMP(op) | Opcode::OP_03_DIRECT_JUMP_PHANTOM(op) => {
					let mut map = HashMap::new();
					let idx = self
						.opcodes
						.par_iter()
						.position_any(|it| it.address() == op.jump_address)
						.unwrap();
					map.insert(0, idx);
					log::debug!(
						"Direct jump opcode at 0x{:08X} (actual 0x{:08X}) jumps to: 0x{:04X}",
						op.address,
						actual_address,
						self.opcodes[idx].address(),
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
					log::debug!(
						"Conditional jump Opcode at 0x{:08X} (actual {:08X}) jumps to: {:08X}",
						op.address,
						actual_address,
						self.opcodes[thing].address()
					);
				}
				Opcode::JZ(op) | Opcode::JNZ(op) => {
					let mut map = HashMap::new();
					let data = self
						.opcodes
						.par_iter()
						.position_any(|it| it.address() == op.jump_address)
						.unwrap();
					map.insert(0, data);
					jump_map.insert(op.address, map);
					log::debug!(
						"Conditional jump Opcode at 0x{:08X} (actual {:08X}) jumps to: {:08X}",
						op.address,
						actual_address,
						self.opcodes[data].address(),
					);
				}
				Opcode::Switch(op) => {
					let jumps = op
						.arms
						.iter()
						.map(|arm| {
							(
								arm.index,
								self.opcodes
									.par_iter()
									.position_any(|it| it.address() == arm.jump_address)
									.unwrap(),
							)
						})
						.collect();
					jump_map.insert(op.address, jumps);
				}
				Opcode::OP_CHOICE(op) | Opcode::OP_MENU_CHOICE(op) => {
					let jumps = op
						.choices
						.iter()
						.enumerate()
						.filter_map(|(idx, choice)| {
							if choice.jump_address == 0 {
								None
							} else {
								Some((
									idx as u16,
									self.opcodes
										.par_iter()
										.position_any(|it| it.address() == choice.jump_address)
										.unwrap(),
								))
							}
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
								let curr_opcode =
									insert.contents.get(idx + i as usize).cloned().unwrap();
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
		Opcode::OP_DIRECT_JUMP(mut op) => {
			let tbl_entry = &jump_table[&op.address];
			op.jump_address = opcodes[tbl_entry[&0]].actual_address();
			log::debug!(
				"Adjusting direct jump Opcode at 0x{:08X} (actual 0x{:08X}) to jump to: 0x{:08X}",
				op.address,
				op.actual_address,
				op.jump_address,
			);
			Opcode::OP_DIRECT_JUMP(op)
		}
		Opcode::OP_03_DIRECT_JUMP_PHANTOM(mut op) => {
			let tbl_entry = &jump_table[&op.address];
			op.jump_address = opcodes[tbl_entry[&0]].actual_address();
			log::debug!(
        "Adjusting direct jump Opcode (03) at 0x{:08X} (actual 0x{:08X}) to jump to: 0x{:08X}",
        op.address,
        op.actual_address,
        op.jump_address,
      );
			Opcode::OP_03_DIRECT_JUMP_PHANTOM(op)
		}
		Opcode::JNE(mut op)
		| Opcode::JE(mut op)
		| Opcode::JLE(mut op)
		| Opcode::JL(mut op)
		| Opcode::JGE(mut op)
		| Opcode::JG(mut op) => {
			let tbl_entry = &jump_table[&op.address];
			op.jump_address = opcodes[tbl_entry[&0]].actual_address();
			log::debug!(
        "Adjusting conditional jump Opcode ({:02X}) at 0x{:08X} (actual {:08X}) to jump to: {:08X}",
        op.opcode,
        op.address,
        op.actual_address,
        op.jump_address,
      );
			op.into()
		}
		Opcode::JNZ(mut op) | Opcode::JZ(mut op) => {
			let tbl_entry = &jump_table[&op.address];
			op.jump_address = opcodes[tbl_entry[&0]].actual_address();
			log::debug!(
        "Adjusting conditional jump Opcode ({:02X}) at 0x{:08X} (actual {:08X}) to jump to: {:08X}",
        op.opcode,
        op.address,
        op.actual_address,
        op.jump_address,
      );
			op.into()
		}

		Opcode::Switch(mut op) => {
			for branch in op.arms.iter_mut() {
				let tbl_entry = &jump_table[&op.address];
				branch.jump_address = opcodes[tbl_entry[&branch.index]].actual_address()
			}
			Opcode::Switch(op)
		}
		Opcode::OP_CHOICE(mut op) | Opcode::OP_MENU_CHOICE(mut op) => {
			for (idx, branch) in op.choices.iter_mut().enumerate() {
				let tbl_entry = &jump_table[&op.address];
				if tbl_entry.is_empty() {
					continue;
				}
				branch.jump_address = opcodes[tbl_entry[&(idx as u16)]].actual_address()
			}
			op.into()
		}
		Opcode::OP_Insert(ins_opcode) => {
			let mut res: Vec<_> = vec![];
			for opcode in ins_opcode.contents.into_iter() {
				log::debug!("Entering insert adjustment.");
				let adjustment = adjust_single_opcode(opcode, jump_table, opcodes).unwrap();
				log::debug!("Leaving insert adjustment.");
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
    use std::collections::{HashMap, HashSet};

    use crate::opcodescript::Script;

    // #[test]
	// fn test_thing() {
	//   let thing = include_bytes!("../../scenario/0045.yaml");

	//   let res: Script = serde_yml::from_slice(thing).unwrap();
	//   res.binary_serialize();
	// }

	#[test]
	fn things() {
		let all = walkdir::WalkDir::new("/home/wscp/cc_tl/scenario");

		let mut thing = HashSet::new();

		for i in all
			.into_iter()
			.filter_map(|it| it.ok())
			.filter(|file| file.file_name().to_string_lossy().ends_with("yaml"))
		{
			let Ok(res): Result<Script, _> =
				serde_yml::from_slice(&std::fs::read(i.path()).unwrap())
			else {
				continue;
			};

			let all_unique_char_names_and_tls: HashMap<_, _> = res
				.opcodes
				.into_iter()
				.filter_map(|it| {
					if it.opcode() != 0x47 {
						return None;
					}
					if let crate::opcodescript::Opcode::OP_FREE_TEXT_OR_CHARNAME(op) = it {
						return if op.opt_arg2.is_none() {
							Some((op.unicode, op.translation.unwrap_or_default()))
						} else {
							None
						};
					} else {
						None
					}
				})
				.collect();
			// println!("{all_unique_char_names_and_tls:#?}");
			thing.extend(all_unique_char_names_and_tls.into_iter());
		}
		let output: String = thing.iter().map(|(a, b)| format!("{a} : {b}\n")).collect();

		println!("{output}");
	}
}
