use binary_serialize_derive::BinarySerialize;
use opcodelike_derive::Opcodelike;
use serde::Serializer;
use serde_derive::*;
use sizedop_derive::SizedOpcode;

pub trait SizedOpcode {
  fn size(&self) -> usize;
}

pub trait Opcodelike: SizedOpcode + BinarySerialize {
  fn address(&self) -> u32;
  fn opcode(&self) -> u8;
  fn actual_address(&self) -> u32;
  fn set_actual_address(&mut self, new_addr: u32);
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]

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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]

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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
pub struct StringOpcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_slice")]
  pub header: [u8; 4],
  pub unicode: String,
  pub notes: Option<String>,
  pub translation: Option<String>,
}

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
pub struct StringOpcode2 {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_slice")]
  pub header: [u8; 2],
  pub unicode: String,
  pub notes: Option<String>,
  pub translation: Option<String>,
}

impl From<StringOpcode> for Opcode {
  fn from(value: StringOpcode) -> Self {
    match value.opcode {
      0x45 => Opcode::OP_TEXTBOX_DISPLAY(value),
      0x85 => Opcode::OP_DEBUG_PRINT(value),
      0x86 => Opcode::OP_SPECIAL_TEXT(value),
      _ => unreachable!(),
    }
  }
}

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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
  pub unicode: String,
  pub notes: Option<String>,
  pub translation: Option<String>,
}

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
pub struct String55Opcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg1: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_slice")]
  pub padding_1: [u8; 3],
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub arg2: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_slice")]
  pub padding_2: [u8; 2],
  pub unicode: String,
  pub notes: Option<String>,
  pub translation: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GenericJumpOpcode<const HEADER_SIZE: usize> {
  pub address: usize,
  pub opcode: u8,
  pub header: [u8; HEADER_SIZE],
  pub jump_address: u32,
  pub size: usize,
}

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]

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

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]

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

impl From<ChoiceOpcode> for Opcode {
  fn from(value: ChoiceOpcode) -> Self {
    match value.opcode {
      0x31 => Self::OP_CHOICE(value),
      0x32 => Self::OP_MENU_CHOICE(value),
      _ => unreachable!(),
    }
  }
}

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
pub struct LongJumpOpcode {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  pub target_script: u16,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u16")]
  pub jump_address: u16,
}

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, SizedOpcode)]
pub struct Choice {
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_inline_ints_slice")]
  pub header: [u8; 6],
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u32")]
  pub jump_address: u32,
  pub unicode: String,
  pub notes: Option<String>,
  pub translation: Option<String>,
}

#[derive(
  Clone, PartialEq, Debug, SizedOpcode, Serialize, Deserialize, BinarySerialize, Opcodelike,
)]
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

/// This opcode is essentially a hardcoded conditional that only checks for whether a particular
/// flag has values below a certain level.
///
/// If the condition evaluates to false, the first `skip` opcodes that occur after this opcode will
/// be skipped, and execution will resume from the `pc + [skip] + 1`'th opcode.
///
/// Otherwise, execution will continue from the first opcode immediately after this one.
///
/// Here's how you use it:
/// 1. Find where you want to add new opcodes.
/// 2. Insert the opcodes you need to add using the OP_Insert structure.
/// 3. add this opcode in front, specifying how many opcodes after this one to skip, as well as the condition level.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Opcodelike)]
pub struct Custom77 {
  pub address: u32,
  #[serde(skip)]
  pub actual_address: u32,
  #[serde(serialize_with = "crate::opcodescript::opcodes::serialize_hex_u8")]
  pub opcode: u8,
  /// The tip level. 0 = The tip will always be on, 1 = Tips for obscure details, 2 = All tips.
  /// To disable this tip, set to a value greater than 2.
  pub condition: u8,
  /// The number of succeeding opcodes to skip if tips are disabled.
  pub skip: u16,
  #[serde(skip)]
  pub skip_bytes: u16,
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
  OP_RESET(S),          // reset? 0x00
  OP_DIRECT_JUMP(D),    // unconditional jump 0x01
  OP_JUMP_TO_SCRIPT(L), // jump to script in index 0x02
  OP_03(B4),
  OP_04(B4),
  OP_SCRIPT_RETURN(S), // Seems like 1 byte is the only valid size; script end/return? 0x05
  JE(J4),              // je 0x06
  JNE(J4),             // jne 0x07
  JG(J4),              // 0x08
  JGE(J4),             // 0x09
  JL(J4),              // 0x0A
  JLE(J4),             // 0x0B
  JZ(J2),              // 0x0C
  JNZ(J2),             // 0x0D
  Switch(SS),          // 0x0E
  OP_0F_SG(S),
  OP_0F_XBOX(B8),
  OP_10(B4),
  OP_11(B4),
  OP_12(B4),
  OP_13(B4),
  OP_14(B4),
  OP_15(B4),
  OP_16(B4),
  OP_17(B4),
  OP_19(B8),
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
  OP_23_PSP(B6),
  OP_24(B6), // : 7,
  OP_25(B4), // : 5,  // 26 to 2C are absent in final complete for desktop.
  // An opportunity to insert new code?
  OP_2A(S),
  OP_2B(S),
  OP_2C(B2),
  OP_2D(B4),         // : 5,
  OP_2E(S),          // : 1,
  OP_2F(B2),         // : 3,
  OP_30(B10),        // : 11,
  OP_CHOICE(C),      // : getlen_opcodes_31_32, # choice 0x31
  OP_MENU_CHOICE(C), // : getlen_opcodes_31_32, 0x32
  OP_33(S),          // : 1,
  OP_34(B10),        // : 11,
  OP_36(B3),         // : 4,
  OP_37(S),
  OP_39(B4), // : 5,
  OP_3A(B4), // : 5,
  OP_3B(B2), // : 3,
  OP_3C(B2), // : 3,
  OP_42(B8), // : 9,
  OP_43(B4), // : 5, x360 as well.
  OP_43_OLDPSP(B2),
  OP_PLAY_VOICE(Op44Opcode),     // : getlen_opcode44 - voice? 0x44
  OP_TEXTBOX_DISPLAY(ST),        // : getlen_opcode_4_plus_sz, # text 0x45
  OP_FREE_TEXT_OR_CHARNAME(S47), // : getlen_opcode_4_plus_sz, # charname 0x47
  OP_47_TEXT(StringOpcode2),
  OP_48(B2),           // : 3,
  OP_CLEAR_SCREEN(B4), // : 5, - clear screen 0x49
  OP_WAIT(B2),         // : 3, - Wait for user input 0x4A
  OP_4B(B4),           // : 5,
  OP_4C(B6),           // : 7,
  OP_4F(B4),           // : 5,
  OP_51(B6),           // : 7,
  OP_55(String55Opcode),
  OP_56_SG2(B4),
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
  OP_CUSTOM_TIP_77(Custom77), // Custom Tip opcode, see readme for details. 0x77
  OP_79(String55Opcode),
  OP_7A_SG2(B6),
  OP_7A_ROOT_XBOX(B10),
  OP_7B(B4), // : 5,
  OP_7B_ROOT_XBOX(ST),
  OP_81_SG2(B6),
  OP_82(B2), // : 3, -
  OP_83(B4), // : 5,
  OP_84_SG(B2),
  OP_DEBUG_PRINT(ST),  // : getlen_opcode_4_plus_sz, # ? Debug string ? 0x85
  OP_SPECIAL_TEXT(ST), // : getlen_opcode_4_plus_sz, # Special text  0x86
  OP_86_PSP(B4),
  OP_87_ROOT_XBOX(S),
  OP_8A_ROOT_XBOX(B2),
  OP_8B_XBOX(B4),
  OP_8C_XBOX(B12),
  OP_8D_XBOX(S),
  OP_8E_ROOT_XBOX(B10),
  OP_8F_ROOT_XBOX(B6),
  OP_Insert(InsertOpcode), // Use this to insert new opcodes into a script. 0xFF (not retained after compilation)
}
