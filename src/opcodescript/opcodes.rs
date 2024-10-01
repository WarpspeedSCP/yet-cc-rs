use binary_serialize_derive::BinarySerialize;
use serde::Serializer;
use serde_derive::*;
use sizedop_derive::SizedOpcode;

pub trait SizedOpcode {
  fn size(&self) -> usize;
}

pub fn serialize_inline_ints_slice<S>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let string = format!(
    "[ {} ]",
    data
      .iter()
      .map(|int| format!("0x{int:02X}"))
      .collect::<Vec<_>>()
      .join(", ")
  );

  serializer.serialize_str(&string)
}

#[allow(dead_code)]
pub fn serialize_hex_usize<S>(data: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(&format!(r#""0x{data:08X}""#))
}

pub fn serialize_hex_u32<S>(data: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(&format!(r#""0x{data:08X}""#))
}

pub fn serialize_hex_u16<S>(data: &u16, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(&format!(r#""0x{data:04X}""#))
}

pub fn serialize_opt_hex_u16<S>(data: &Option<u16>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  match data {
    Some(inner) => serializer.serialize_str(&format!(r#""0x{inner:04X}""#)),
    None => serializer.serialize_none(),
  }
}

pub fn serialize_hex_u8<S>(data: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(&format!(r#""0x{data:02X}""#))
}

pub fn serialize_inline_ints_vec<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let string = format!(
    "[ {} ]",
    data
      .iter()
      .map(|int| format!("0x{int:02X}"))
      .collect::<Vec<_>>()
      .join(", ")
  );

  serializer.serialize_str(&string)
}

pub trait BinarySerialize {
  fn binary_serialize(&self) -> Vec<u8>;
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]

pub struct SingleByteOpcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GenericBasicOpcode<const DATA_SIZE: usize> {
  pub address: usize,
  pub opcode: u8,
  pub data: [u8; DATA_SIZE],
  pub size: usize,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]

pub struct BasicOpcode2 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]

pub struct BasicOpcode3 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub padding: u8,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct BasicOpcode4 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg2: u16,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct BasicOpcode6 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg2: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg3: u16,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct BasicOpcode8 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg2: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg3: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg4: u16,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct BasicOpcode10 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg2: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg3: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg4: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg5: u16,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct BasicOpcode12 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg2: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg3: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg4: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg5: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg6: u16,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct BasicOpcode16 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg2: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg3: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg4: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg5: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg6: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg7: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg8: u16,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct Op44Opcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  // Voice type.
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  /// The voice ID
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg2: u16,
  pub padding_end: Option<u8>,
  pub size: usize,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct SwitchOpcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub comparison_value: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub count: u16,
  pub arms: Vec<SwitchArm>,
  pub size: usize,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SwitchArm {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub index: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub jump_address: u32,
}

impl SizedOpcode for SwitchArm {
  fn size(&self) -> usize {
    6
  }
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct StringOpcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_slice")]
  pub header: [u8; 4],
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_vec")]
  pub sjis_bytes: Vec<u8>,
  pub size: usize,
  pub unicode: String,
}

impl From<StringOpcode> for Opcode {
  fn from(value: StringOpcode) -> Self {
    match value.opcode {
      0x45 => Opcode::OP_45(value),
      0x85 => Opcode::OP_85(value),
      0x86 => Opcode::OP_86(value),
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct String47Opcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_opt_hex_u16")]
  pub opt_arg2: Option<u16>,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_vec")]
  pub sjis_bytes: Vec<u8>,
  pub size: usize,
  pub unicode: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GenericJumpOpcode<const HEADER_SIZE: usize> {
  pub address: usize,
  pub opcode: u8,
  pub header: [u8; HEADER_SIZE],
  pub jump_address: u32,
  pub size: usize,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]

pub struct JumpOpcode2 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub jump_address: u32,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]

pub struct JumpOpcode4 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg2: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub jump_address: u32,
}

impl From<JumpOpcode2> for Opcode {
  fn from(value: JumpOpcode2) -> Self {
    match value.opcode {
      0x0C => Self::JNZ(value), // (J<6>), // jnz
      0x0D => Self::JZ(value),
      _ => unreachable!(), // (J<6>), // jz
    }
  }
}

impl From<JumpOpcode4> for Opcode {
  fn from(value: JumpOpcode4) -> Self {
    match value.opcode {
      0x06 => Self::JNE(value), // (J<4>), // jne
      0x07 => Self::JE(value),  // (J<4>), // je
      0x08 => Self::JLE(value), // (J<4>), // jle
      0x09 => Self::JL(value),  // (J<4>), // jl
      0x0A => Self::JGE(value), // (J<4>), // jge
      0x0B => Self::JG(value),  // (J<4>), // jg
      _ => unreachable!(),      // (J<6>), // jz
    }
  }
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct LongJumpOpcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub target_script: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub jump_address: u16,
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct DirectJumpOpcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub jump_address: u32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Choice {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_slice")]
  pub header: [u8; 10],
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_vec")]
  pub sjis_bytes: Vec<u8>,
  pub unicode: String,
}

impl SizedOpcode for Choice {
  fn size(&self) -> usize {
    use encoding_rs::SHIFT_JIS;
    self.header.len() + SHIFT_JIS.encode(&self.unicode).0.len() + 1
  }
}

#[derive(Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize)]
pub struct ChoiceOpcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_slice")]
  pub pre_header: [u8; 0x02],
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub n_choices: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_slice")]
  pub header: [u8; 0x03],
  pub choices: Vec<Choice>,
  pub size: usize,
}

/// Avoid serializing this opcode, it is only meant to be used when converting to an opcodescript file.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct InsertOpcode {
  pub contents: Vec<Opcode>,
}

impl SizedOpcode for InsertOpcode {
  fn size(&self) -> usize {
    self.contents.iter().map(Opcode::size).sum()
  }
}

/// Avoid serializing this opcode.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Custom77 {
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  pub opcode: u8,
  /// The tip level. 0 = No tips, 1 = Tips for obscure details, 2 = All tips.
  pub condition: u8,
  /// The number of succeeding opcodes to skip if tips are disabled.
  pub skip: u16,
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

pub type B2 = BasicOpcode2;
pub type B3 = BasicOpcode3;
pub type B4 = BasicOpcode4;
pub type B6 = BasicOpcode6;
pub type B8 = BasicOpcode8;
pub type B10 = BasicOpcode10;
pub type B12 = BasicOpcode12;
pub type B16 = BasicOpcode16;

pub type S = SingleByteOpcode;
pub type SS = SwitchOpcode;
pub type ST = StringOpcode;
pub type S47 = String47Opcode;
pub type J2 = JumpOpcode2;
pub type J4 = JumpOpcode4;
pub type D = DirectJumpOpcode;
pub type L = LongJumpOpcode;
pub type C = ChoiceOpcode;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Opcode {
  OP_00(S), // reset?
  OP_01(D), // unconditional jump
  OP_02(L), // jump to script in index
  OP_03(B4),
  OP_04(B4),
  OP_05(S), // Seems like 1 byte is the only valid size; script end/return?
  JNE(J4),  // jne
  JE(J4),   // je
  JLE(J4),
  JL(J4),
  JGE(J4),
  JG(J4),
  JNZ(J2),
  JZ(J2),
  Switch(SS),
  OP_10(B4),
  OP_11(B4),
  OP_12(B4),
  OP_13(B4),
  OP_14(B4),
  OP_15(B4),
  OP_16(B4),
  OP_17(B4),
  OP_1A(B4),  //: 5,
  OP_1B(S),   // 1,
  OP_1C(S),   //: 1,
  OP_1D(B6),  //: 7,
  OP_1E(B10), // : 11,
  OP_1F(B12), // : 13,
  OP_20(B6),  // : 7,
  OP_21(B6),  // : 7,
  OP_22(B4),  // : 5,
  OP_23(B8),  // : 9,
  OP_24(B6),  // : 7,
  OP_25(B4),  // : 5,  // 26 to 2C are absent in final complete for desktop.
  // An opportunity to insert new code?
  OP_2D(B4),                  // : 5,
  OP_2E(S),                   // : 1,
  OP_2F(B2),                  // : 3,
  OP_30(B10),                 // : 11,
  OP_31(C),                   // : getlen_opcodes_31_32, # choice
  OP_32(C),                   // : getlen_opcodes_31_32,
  OP_33(S),                   // : 1,
  OP_34(B10),                 // : 11,
  OP_36(B3),                  // : 4,
  OP_39(B4),                  // : 5,
  OP_3A(B4),                  // : 5,
  OP_3B(B2),                  // : 3,
  OP_3C(B2),                  // : 3,
  OP_42(B8),                  // : 9,
  OP_43(B4),                  // : 5,
  OP_44(Op44Opcode),          // : getlen_opcode44 - voice?
  OP_45(ST),                  // : getlen_opcode_4_plus_sz, # text
  OP_47(S47),                 // : getlen_opcode_4_plus_sz, # charname
  OP_48(B2),                  // : 3,
  OP_49(B4),                  // : 5, - clear screen
  OP_4A(B2),                  // : 3, - Wait for user input
  OP_4B(B4),                  // : 5,
  OP_4C(B6),                  // : 7,
  OP_4F(B4),                  // : 5,
  OP_51(B6),                  // : 7,
  OP_59(S),                   // : 1,
  OP_5A(S),                   // : 1,
  OP_5F(S),                   // : 1,
  OP_68(B10),                 // : 11, // always comes in pairs. Start and end of something?
  OP_69(B2),                  // : 3,
  OP_6A(B4),                  // : 5, debug...
  OP_6C(B16),                 // : 17,
  OP_6E(B4),                  // : 5,
  OP_6F(B6),                  // : 7, unused
  OP_71(B6),                  // : 7. Skips the succeeding 5A if op 2 is not 0xFFFF.
  OP_72(B4),                  // : 5,
  OP_74(B6),                  // : 7,
  OP_75(B4),                  // : 5,
  OP_CUSTOM_TIP_77(Custom77), // Custom Tip opcode, see docs/opcodes for details.
  OP_7B(B4),                  // : 5,
  OP_82(B2),                  // : 3, -
  OP_83(B4),                  // : 5,
  OP_85(ST),                  // : getlen_opcode_4_plus_sz, # ? Debug string ?
  OP_86(ST),                  // : getlen_opcode_4_plus_sz, # Special text
  OP_Insert(InsertOpcode),
}
