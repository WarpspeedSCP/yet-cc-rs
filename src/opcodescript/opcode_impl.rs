use crate::opcodescript::*;
use crate::util::*;
use anyhow::{anyhow, Result};

use bitflags::{bitflags, Flags};
use opcodes::*;

impl SingleByteOpcode {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
		}
		.wrap_ok()
	}
}

impl BasicOpcode2 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericBasicOpcode {
			address,
			opcode,
			data,
			..
		} = GenericBasicOpcode::<2>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &data)?,
		}
		.wrap_ok()
	}
}

impl BasicOpcode3 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericBasicOpcode {
			address,
			opcode,
			data,
			..
		} = GenericBasicOpcode::<3>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &data)?,
			padding: data[2],
		}
		.wrap_ok()
	}
}

impl BasicOpcode4 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericBasicOpcode {
			address,
			opcode,
			data,
			..
		} = GenericBasicOpcode::<4>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &data)?,
			arg2: transmute_to_u16(2, &data)?,
		}
		.wrap_ok()
	}
}

impl BasicOpcode6 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericBasicOpcode {
			address,
			opcode,
			data,
			..
		} = GenericBasicOpcode::<6>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &data)?,
			arg2: transmute_to_u16(2, &data)?,
			arg3: transmute_to_u16(4, &data)?,
		}
		.wrap_ok()
	}
}

impl BasicOpcode8 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericBasicOpcode {
			address,
			opcode,
			data,
			..
		} = GenericBasicOpcode::<8>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &data)?,
			arg2: transmute_to_u16(2, &data)?,
			arg3: transmute_to_u16(4, &data)?,
			arg4: transmute_to_u16(6, &data)?,
		}
		.wrap_ok()
	}
}

impl BasicOpcode10 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericBasicOpcode {
			address,
			opcode,
			data,
			..
		} = GenericBasicOpcode::<10>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &data)?,
			arg2: transmute_to_u16(2, &data)?,
			arg3: transmute_to_u16(4, &data)?,
			arg4: transmute_to_u16(6, &data)?,
			arg5: transmute_to_u16(8, &data)?,
		}
		.wrap_ok()
	}
}

impl BasicOpcode12 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericBasicOpcode {
			address,
			opcode,
			data,
			..
		} = GenericBasicOpcode::<12>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &data)?,
			arg2: transmute_to_u16(2, &data)?,
			arg3: transmute_to_u16(4, &data)?,
			arg4: transmute_to_u16(6, &data)?,
			arg5: transmute_to_u16(8, &data)?,
			arg6: transmute_to_u16(10, &data)?,
		}
		.wrap_ok()
	}
}

impl BasicOpcode16 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericBasicOpcode {
			address,
			opcode,
			data,
			..
		} = GenericBasicOpcode::<16>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &data)?,
			arg2: transmute_to_u16(2, &data)?,
			arg3: transmute_to_u16(4, &data)?,
			arg4: transmute_to_u16(6, &data)?,
			arg5: transmute_to_u16(8, &data)?,
			arg6: transmute_to_u16(10, &data)?,
			arg7: transmute_to_u16(12, &data)?,
			arg8: transmute_to_u16(14, &data)?,
		}
		.wrap_ok()
	}
}

impl<const DATA_SIZE: usize> GenericBasicOpcode<DATA_SIZE> {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		Self {
			address,
			opcode: input[address],
			data: transmute_to_array::<DATA_SIZE>(address + 1, input)?,
			size: 1 + DATA_SIZE,
		}
		.wrap_ok()
	}
}

impl Op44Opcode {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let arg2 = transmute_to_u16(address + 3, input)?;
		Op44Opcode {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
			arg1: transmute_to_u16(address + 1, input)?,
			arg2,
			padding_end: if arg2 == 0xFFFF {
				Some(input[address + 5])
			} else {
				None
			},
			size: if arg2 == 0xFFFF { 6 } else { 5 },
		}
		.wrap_ok()
	}
}

impl SwitchArm {
	pub fn new(input: [u8; 6]) -> Result<Self> {
		Self {
			index: transmute_to_u16(0, &input)?,
			jump_address: transmute_to_u32(2, &input)?,
		}
		.wrap_ok()
	}
}

impl SwitchOpcode {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let count = transmute_to_u16(address + 1 + 2, input)?; // , = unpack("<H", data[start+3:start+5])
		let arms = input[address + 5..address + 5 + (6 * count as usize)]
			.chunks(6)
			.map(|it| transmute_to_array(0, it))
			.map(|it| SwitchArm::new(it?))
			.collect::<Result<Vec<_>>>()?;

		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
			comparison_value: transmute_to_u16(address + 1, input)?,
			count,
			arms,
			size: 1usize + 2 + 2 + (count as usize * 6),
		}
		.wrap_ok()
	}
}

impl StringOpcode {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let (_, unicode) = get_sjis_bytes(address + 1 + 4, input);
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
			header: transmute_to_array(address + 1, input)?,
			unicode,
			notes: None,
			translation: None,
		}
		.wrap_ok()
	}
}

impl StringOpcode2 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let (_, unicode) = get_sjis_bytes(address + 1 + 2, input);
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
			header: transmute_to_array(address + 1, input)?,
			unicode,
			notes: None,
			translation: None,
		}
		.wrap_ok()
	}
}

impl String47Opcode {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let arg1 = transmute_to_u16(address + 1, input)?;
		let string_offset = if arg1 == 0x000D { 2 } else { 4 };
		let (_, unicode) = get_sjis_bytes(address + 1 + string_offset, input);
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
			arg1,
			opt_arg2: if arg1 == 0x000D {
				None
			} else {
				Some(transmute_to_u16(address + 3, input)?)
			},
			unicode,
			notes: None,
			translation: None,
		}
		.wrap_ok()
	}
}

impl String55Opcode {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let (_, unicode) = get_sjis_bytes(address + 1 + 9, input);
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
			arg1: transmute_to_u16(address + 1, input)?,
			padding_1: transmute_to_array(address + 3, input)?,
			arg2: transmute_to_u16(address + 6, input)?,
			padding_2: transmute_to_array(address + 8, input)?,
			unicode,
			notes: None,
			translation: None,
		}
		.wrap_ok()
	}
}

impl GenericJumpOpcode<4> {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		Self {
			address,
			opcode: input[address],
			header: transmute_to_array(address + 1, input)?,
			jump_address: transmute_to_u32(address + 1 + 4, input)?,
			size: 1 + 4 + 4,
		}
		.wrap_ok()
	}
}

impl GenericJumpOpcode<2> {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let jump_address = transmute_to_u32(address + 1 + 2, input)?;
		Self {
			address,
			opcode: input[address],
			header: transmute_to_array(address + 1, input)?,
			jump_address,
			size: 1 + 4 + 2,
		}
		.wrap_ok()
	}
}

impl JumpOpcode2 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericJumpOpcode {
			address,
			opcode,
			header,
			jump_address,
			..
		} = GenericJumpOpcode::<2>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &header)?,
			jump_address,
		}
		.wrap_ok()
	}
}

impl JumpOpcode4 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericJumpOpcode {
			address,
			opcode,
			header,
			jump_address,
			..
		} = GenericJumpOpcode::<4>::new(address, input)?;
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			arg1: transmute_to_u16(0, &header)?,
			arg2: transmute_to_u16(2, &header)?,
			jump_address,
		}
		.wrap_ok()
	}
}

fn get_header<const SIZE: usize>(
	address: usize,
	input: &[u8],
) -> std::result::Result<[u8; SIZE], YetiError> {
	input[address..address + SIZE]
		.try_into()
		.map_err(|_| YetiError::ParseHeader {
			address,
			header_size: SIZE,
		})
}

impl Choice {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let (_, choice_str) = get_sjis_bytes(address + 10, input);
		Choice {
			address: address as u32,
			header: get_header(address, input)?,
			unicode: choice_str,
			jump_address: transmute_to_u32(address + 6, input)?,
			notes: None,
			translation: None,
		}
		.wrap_ok()
	}
}

impl ChoiceOpcode {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let pre_header = get_header(address + 1, input)?;
		let n_choices = input[address + 3];
		let header = get_header(address + 4, input)?;
		let mut choices = vec![];

		let mut choice_addr = 7;
		for _ in 0..n_choices {
			let choice = Choice::new(address + choice_addr, input)?;
			choice_addr += choice.header.len() + 4 + encode_sjis(&choice.unicode).len() + 1; // +4 for the jump address.
			choices.push(choice);
		}

		ChoiceOpcode {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
			pre_header,
			n_choices,
			header,
			choices,
		}
		.wrap_ok()
	}
}

impl DirectJumpOpcode {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
			jump_address: transmute_to_u32(address + 1, input)?,
		}
		.wrap_ok()
	}
}

impl LongJumpOpcode {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode: input[address],
			target_script: transmute_to_u16(address + 1, input)?,
			jump_address: transmute_to_u16(address + 3, input)?,
		}
		.wrap_ok()
	}
}

impl Custom77 {
	pub fn new(address: usize, input: &[u8]) -> Result<Self> {
		let GenericBasicOpcode {
			address,
			opcode,
			data,
			..
		} = GenericBasicOpcode::<3>::new(address, input)?;

		Self {
			address: address as u32,
			actual_address: address as u32,
			opcode,
			condition: data[0],
			skip: 0,
			skip_bytes: transmute_to_u16(1, &data)?,
		}
		.wrap_ok()
	}
}

impl BinarySerialize for Custom77 {
	fn binary_serialize(&self) -> Vec<u8> {
		return vec![self.opcode, self.condition, 0, 0];
	}
}

impl SizedOpcode for Custom77 {
	fn size(&self) -> usize {
		4
	}
}

bitflags! {
  #[derive(Copy, Clone, Debug)]
  pub struct Quirks: u8 {
	const CCFC      = 1;
	const PSP       = 2;
	const XBox      = 4;
	const XBoxRoot  = 8;
	const SG        = 16;
	const SG2       = 32;
	const Phantom   = 64;
  }
}

impl Quirks {
	pub fn names() -> Vec<String> {
		["CCFC", "PSP", "XBox", "XBoxRoot", "SG", "SG2", "Phantom"]
			.into_iter()
			.map(|it| it.to_lowercase())
			.collect()
	}
}

impl Opcode {
	pub fn eat(address: usize, input: &[u8], quirks: Quirks) -> Result<Self> {
		let opcode = input[address];
		log::debug!("Got opcode 0x{opcode:02X} at address 0x{address:08X}.");
		match opcode {
			// Complete each switch case with the correct variant of Opcode based on the members of opcode.
			0x00 => Ok(Self::OP_RESET(S::new(address, input)?)), // (S),    // reset?

			0x01 => Ok(Self::OP_DIRECT_JUMP(D::new(address, input)?)), // (B<4>), // unconditional jump
			0x02 => Ok(Self::OP_JUMP_TO_SCRIPT(L::new(address, input)?)), // (B<4>), // jump to script in index
			0x03 => Ok(Self::OP_03_DIRECT_JUMP_PHANTOM(D::new(address, input)?)), // (B<4>),
			0x04 => Ok(Self::OP_04_JUMP_TO_SCRIPT_WITH_OFFSET_PHANTOM(L::new(
				address, input,
			)?)), // (B<4>),

			0x05 => Ok(Self::OP_SCRIPT_RETURN(S::new(address, input)?)), // Seems like 1 byte is the only valid size; script end/return?

			0x06 => Ok(Self::JE(J4::new(address, input)?)), // (J<4>), // je
			0x07 => Ok(Self::JNE(J4::new(address, input)?)), // (J<4>), // jne
			0x08 => Ok(Self::JG(J4::new(address, input)?)), // (J<4>), // jg
			0x09 => Ok(Self::JGE(J4::new(address, input)?)), // (J<4>), // jge
			0x0A => Ok(Self::JL(J4::new(address, input)?)), // (J<4>), // jl
			0x0B => Ok(Self::JLE(J4::new(address, input)?)), // (J<4>), // jle

			0x0C => Ok(Self::JZ(J2::new(address, input)?)), // (J<6>), // jz
			0x0D => Ok(Self::JNZ(J2::new(address, input)?)), // (J<6>), // jnz
			0x0E => Ok(Self::Switch(SS::new(address, input)?)), // (SS),   // switch

			0x0F => {
				if quirks.contains(Quirks::SG) {
					Ok(Self::OP_0F_SG(S::new(address, input)?))
				} else if quirks.contains(Quirks::XBox) {
					Ok(Self::OP_0F_XBOX(B8::new(address, input)?))
				} else {
					anyhow!("Bad use of 0F without quirks at address 0x{address:08X}. This script might be meant for Secret Garden (add the \"sg\" quirk) or an xbox 360 game (add the \"x360\" quirk).").wrap_err()
				}
			}

			0x10 => Ok(Self::OP_10(B4::new(address, input)?)), // (B<4>),
			0x11 => Ok(Self::OP_11(B4::new(address, input)?)), // (B<4>),
			0x12 => Ok(Self::OP_12(B4::new(address, input)?)), // (B<4>),
			0x13 => Ok(Self::OP_13(B4::new(address, input)?)), // (B<4>),
			0x14 => Ok(Self::OP_14(B4::new(address, input)?)), // (B<4>),
			0x15 => Ok(Self::OP_15(B4::new(address, input)?)), // (B<4>),
			0x16 => Ok(Self::OP_16(B4::new(address, input)?)), // (B<4>),
			0x17 => Ok(Self::OP_17(B4::new(address, input)?)), // (B<4>),
			0x18 => Ok(Self::OP_18_PHANTOM(B4::new(address, input)?)),
			0x19 => Ok(Self::OP_19(B8::new(address, input)?)),
			0x1A => Ok(Self::OP_1A(B4::new(address, input)?)), // (B<4>),  //: 5,

			0x1B => Ok(Self::OP_1B(S::new(address, input)?)),
			0x1C => Ok(Self::OP_1C(S::new(address, input)?)),

			0x1D => Ok(Self::OP_1D(B6::new(address, input)?)), // (B<6>),  //: 7,

			0x1E => Ok(Self::OP_1E(B10::new(address, input)?)), // (B<10>), // : 11,

			0x1F => Ok(Self::OP_1F(B12::new(address, input)?)), // (B<12>), // : 13,

			0x20 => Ok(Self::OP_20(B6::new(address, input)?)), // (B<6>), // : 7,
			0x21 => Ok(Self::OP_21(B6::new(address, input)?)), // (B<6>), // : 7,

			0x22 => Ok(Self::OP_22(B4::new(address, input)?)), // (B<4>), // : 5,4

			0x23 => {
				if quirks.contains(Quirks::CCFC) || quirks.contains(Quirks::Phantom) {
					Ok(Self::OP_23(B8::new(address, input)?)) // (B<8>), // : 9,
				} else {
					Ok(Self::OP_23_PSP(B6::new(address, input)?))
				}
			}

			0x24 => Ok(Self::OP_24(B6::new(address, input)?)), // (B<6>), // : 7,

			0x25 => Ok(Self::OP_25(B4::new(address, input)?)), // (B<4>), // : 5,  // 26 to 2C are absent in final complete for desktop.
			0x2A => Ok(Self::OP_2A(S::new(address, input)?)),
			0x2B => Ok(Self::OP_2B(S::new(address, input)?)),
			0x2C => Ok(Self::OP_2C(B2::new(address, input)?)),
			0x2D => Ok(Self::OP_2D(B4::new(address, input)?)), // (B<4>), // : 5,

			0x2E => Ok(Self::OP_2E(S::new(address, input)?)),

			0x2F => Ok(Self::OP_2F(B2::new(address, input)?)), // (B<2>), // : 3,

			0x30 => Ok(Self::OP_30(B10::new(address, input)?)), // (B<10>), // : 11,

			0x31 => Ok(Self::OP_CHOICE(C::new(address, input)?)), // (C),     // : getlen_opcodes_31_32, # choice
			0x32 => Ok(Self::OP_MENU_CHOICE(C::new(address, input)?)), // (C),     // : getlen_opcodes_31_32,

			0x33 => Ok(Self::OP_33(S::new(address, input)?)),

			0x34 => Ok(Self::OP_34(B10::new(address, input)?)), // (B<10>), // : 11,

			0x36 => Ok(Self::OP_36(B3::new(address, input)?)), // (B<3>),  // : 4,

			0x37 => Ok(Self::OP_37(S::new(address, input)?)), // : 1

			0x39 => Ok(Self::OP_39(B4::new(address, input)?)), // (B<4>),  // : 5,
			0x3A => Ok(Self::OP_3A(B4::new(address, input)?)), // (B<4>),  // : 5,

			0x3B => Ok(Self::OP_3B(B2::new(address, input)?)), // (B<2>),  // : 3,
			0x3C => Ok(Self::OP_3C(B2::new(address, input)?)), // (B<2>),  // : 3,

			0x42 => Ok(Self::OP_42(B8::new(address, input)?)), // (B<8>), // : 9,

			0x43 => {
				if quirks.intersects(Quirks::CCFC | Quirks::XBox | Quirks::XBoxRoot | Quirks::SG2) {
					Ok(Self::OP_43(B4::new(address, input)?))
				} else {
					Ok(Self::OP_43_OLDPSP(B2::new(address, input)?))
				}
			} // (B<4>), // : 5,

			0x44 => Ok(Self::OP_PLAY_VOICE(Op44Opcode::new(address, input)?)), // (Op44Opcode),     // : getlen_opcode44,

			0x45 => Ok(Self::OP_TEXTBOX_DISPLAY(ST::new(address, input)?)), // (ST),     // : getlen_opcode_4_plus_sz, # text

			0x47 => {
				if quirks.intersects(Quirks::CCFC | Quirks::XBox | Quirks::XBoxRoot | Quirks::SG2) {
					Ok(Self::OP_FREE_TEXT_OR_CHARNAME(S47::new(address, input)?))
				// (ST),     // : getlen_opcode_4_plus_sz, # charname
				} else {
					Ok(Self::OP_47_TEXT(StringOpcode2::new(address, input)?))
				}
			}
			0x48 => Ok(Self::OP_48(B2::new(address, input)?)), // (B<2>),     // : 3,
			0x49 => Ok(Self::OP_CLEAR_SCREEN(B4::new(address, input)?)), // (B<4>),     // : 5,
			0x4A => Ok(Self::OP_WAIT(B2::new(address, input)?)), // (B<2>),     // : 3,
			0x4B => Ok(Self::OP_4B(B4::new(address, input)?)), // (B<4>),     // : 5,
			0x4C => Ok(Self::OP_4C(B6::new(address, input)?)), // (B<6>),     // : 7,
			0x4F => Ok(Self::OP_4F(B4::new(address, input)?)), // (B<4>),     // : 5,
			0x51 => Ok(Self::OP_51(B6::new(address, input)?)), // (B<6>), // : 7,
			0x53 => Ok(Self::OP_53_PHANTOM(B2::new(address, input)?)),
			0x55 => Ok(Self::OP_55(String55Opcode::new(address, input)?)),
			0x56 => {
				if quirks.contains(Quirks::Phantom) {
					Ok(Self::OP_56_PHANTOM(B2::new(address, input)?))
				} else {
					Ok(Self::OP_56_SG2(B4::new(address, input)?))
				}
			}
			0x59 => Ok(Self::OP_59(S::new(address, input)?)),
			0x5A => Ok(Self::OP_5A(S::new(address, input)?)),
			0x5F => Ok(Self::OP_5F(S::new(address, input)?)),

			0x66 => Ok(Self::OP_66_PHANTOM(B2::new(address, input)?)),
			0x68 => Ok(Self::OP_68(B10::new(address, input)?)), // (B<10>), // : 11, // always comes in pairs. Start and end of something?
			0x69 => Ok(Self::OP_69(B2::new(address, input)?)),  // (B<2>), // : 3,
			0x6A => Ok(Self::OP_6A(B4::new(address, input)?)),  // (B<4>), // : 5, debug...
			0x6B => Ok(Self::OP_6B_PHANTOM(B2::new(address, input)?)), // (B<2>), // : 3,
			0x6C => Ok(Self::OP_6C(B16::new(address, input)?)), // (B<16>), // : 17,
			0x6E => Ok(Self::OP_6E(B4::new(address, input)?)),  // (B<4>), // : 5,
			0x6F => Ok(Self::OP_6F(B6::new(address, input)?)),  // (B<6>), // : 7, unused
			0x70 => Ok(Self::OP_70(S::new(address, input)?)),
			0x71 => Ok(Self::OP_71(B6::new(address, input)?)), // (B<6>), // : 7. Skips the succeeding 5A if op 2 is not 0xFFFF.
			0x72 => Ok(Self::OP_72(B4::new(address, input)?)), // (B<4>), // : 5,
			0x74 => Ok(Self::OP_74(B6::new(address, input)?)), // (B<6>), // : 7,
			0x75 => Ok(Self::OP_75(B4::new(address, input)?)), // (B<4>), // : 5,
			0x77 => Ok(Self::OP_CUSTOM_TIP_77(Custom77::new(address, input)?)),
			0x7A => {
				if quirks.contains(Quirks::SG2) {
					Ok(Self::OP_7A_SG2(B6::new(address, input)?))
				} else if quirks.contains(Quirks::XBoxRoot) {
					Ok(Self::OP_7A_ROOT_XBOX(B10::new(address, input)?))
				} else {
					Err(anyhow!("Bad use of 7A without quirks at address 0x{address:08X}. This script might be meant for Secret Garden 2 (add the \"sg2\" quirk) or root double (add the \"rootx360\" quirk)."))
				}
			}
			0x7B => {
				if quirks.contains(Quirks::XBoxRoot) {
					Ok(Self::OP_7B_ROOT_XBOX(ST::new(address, input)?))
				} else {
					Ok(Self::OP_7B(B4::new(address, input)?))
				}
			} // (B<4>), // : 5,
			0x80 => Ok(Self::OP_80_PHANTOM(B4::new(address, input)?)),
			0x81 => Ok(Self::OP_81_SG2(B6::new(address, input)?)),
			0x82 => Ok(Self::OP_82(B2::new(address, input)?)), // (B<2>), // : 3, -
			0x83 => Ok(Self::OP_83(B4::new(address, input)?)), // (B<4>), // : 5,
			0x84 => Ok(Self::OP_84_SG(B2::new(address, input)?)),
			0x85 => Ok(Self::OP_DEBUG_PRINT(ST::new(address, input)?)), // (ST), // : getlen_opcode_4_plus_sz, # ? Debug string ?
			0x86 => Ok(Self::OP_SPECIAL_TEXT(ST::new(address, input)?)), // (ST), // : getlen_opcode_4_plus_sz, # Special text
			0x87 => {
				log::warn!("got opcode 87 at 0x{:08X}", address);
				Ok(Self::OP_87_ROOT_XBOX(S::new(address, input)?))
			}
			0x8A => Ok(Self::OP_8A_ROOT_XBOX(B2::new(address, input)?)),
			0x8B => Ok(Self::OP_8B_XBOX(B4::new(address, input)?)),
			0x8C => {
				if quirks.contains(Quirks::Phantom) {
					Ok(Self::OP_8C_PHANTOM(B4::new(address, input)?))
				} else {
					Ok(Self::OP_8C_XBOX(B12::new(address, input)?))
				}
			}
			0x8D => Ok(Self::OP_8D_XBOX(S::new(address, input)?)),
			0x8E => Ok(Self::OP_8E_ROOT_XBOX(B10::new(address, input)?)),
			0x8F => Ok(Self::OP_8F_ROOT_XBOX(B6::new(address, input)?)),
			0x90 => Ok(Self::OP_90_PHANTOM_CHARNAME(StringOpcode2::new(
				address, input,
			)?)),
			0xFF => Ok(Self::OP_FF(S::new(address, input)?)),
			_ => Err(anyhow::Error::new(YetiError::ParseOpcode {
				opcode: input[address],
				address,
			})),
		}
	}

	pub fn address(&self) -> u32 {
		crate::opcode_common_action!(self, op, { op.address() }, {
			op.contents.first().map(Opcode::address).unwrap_or(u32::MAX)
		})
	}

	pub fn size(&self) -> usize {
		crate::opcode_common_action!(self, op, { op.size() }, { op.size() })
	}

	pub fn opcode(&self) -> u8 {
		crate::opcode_common_action!(self, op, { op.opcode() }, {
			log::warn!(
				"Attempting to do something with an insert opcode of size {}.",
				op.size()
			);
			0
		})
	}

	pub fn binary_serialize(&self) -> Vec<u8> {
		crate::opcode_common_action!(self, op, { op.binary_serialize() }, {
			op.contents
				.iter()
				.flat_map(|op| op.binary_serialize())
				.collect()
		})
	}

	pub fn actual_address(&self) -> u32 {
		crate::opcode_common_action!(self, op, { op.actual_address() }, {
			op.contents
				.first()
				.map(Opcode::actual_address)
				.unwrap_or_default()
		})
	}

	pub fn set_actual_address(&mut self, actual_address: usize) {
		crate::opcode_common_action!(
			self,
			op,
			{
				op.set_actual_address(actual_address as u32);
			},
			{ op.contents[0].set_actual_address(actual_address) }
		)
	}
}

impl TryFrom<Opcode> for Custom77 {
	type Error = &'static str;

	fn try_from(value: Opcode) -> Result<Self, Self::Error> {
		match value {
			Opcode::OP_CUSTOM_TIP_77(value) => Ok(value),
			_ => Err("Opcode was not a tip."),
		}
	}
}
