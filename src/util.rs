use bimap::BiHashMap;
use once_cell::sync::Lazy;

pub fn transmute_to_array<const SIZE: usize>(address: usize, input: &[u8]) -> [u8; SIZE] {
  input[address..address + SIZE].try_into().unwrap()
}

pub fn transmute_to_u32(address: usize, input: &[u8]) -> u32 {
  u32::from_le_bytes(transmute_to_array(address, input))
}

pub fn transmute_to_u16(address: usize, input: &[u8]) -> u16 {
  u16::from_le_bytes(transmute_to_array(address, input))
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SJISChar {
  SingleByte(u8),
  DoubleByte(u16),
}

impl SJISChar {
  fn new(value: char) -> Self {
    if value.is_ascii() {
      Self::SingleByte(value as u8)
    } else {
      use encoding_rs::SHIFT_JIS;
      let sjis = SHIFT_JIS.encode(&format!("{value}")).0.to_vec();
      let sjis_u16 = transmute_to_u16(0, &sjis);
      Self::DoubleByte(sjis_u16)
    }
  }
  fn from_number(value: u16) -> Self {
    if value <= u8::MAX as u16 {
      Self::SingleByte(value as u8)
    } else {
      Self::DoubleByte(value as u16)
    }
  }

  fn to_vec(&self) -> Vec<u8> {
    match self {
      Self::DoubleByte(value) => value.to_be_bytes().to_vec(),
      Self::SingleByte(value) => value.to_be_bytes().to_vec(),
    }
  }
}

pub static ITALIC_MAPPING_TABLE: Lazy<BiHashMap<char, SJISChar>> = Lazy::new(|| {
  let mut map = BiHashMap::new();

  let file: String;
  #[cfg(debug_assertions)]
  {
    file = std::fs::read_to_string("italic_map.json").unwrap();
  }
  #[cfg(not(debug_assertions))]
  {
    file = include_str!("../italic_map.json").to_owned();
  }

  let data: std::collections::HashMap<String, String> = serde_json::from_str(&file).unwrap();

  for (k, v) in data {
    map.insert(
      k.chars().next().unwrap(),
      SJISChar::from_number(u16::from_str_radix(&v, 16).unwrap()),
    );
  }

  map
});

// Tokenises a string into its constituents.
fn tokens(unicode: &str) -> Vec<String> {
  let mut curr_str = String::new();
  let mut result = vec![];
  let chrs = unicode
    .replace("<dquote/>", "\"")
    .replace("<bslash/>", "\\")
    .chars()
    .collect::<Vec<_>>();
  let mut char_idx = 0usize;
  loop {
    let chr = chrs.get(char_idx);
    if chr.is_none() {
      result.push(std::mem::take(&mut curr_str));
      break;
    }

    let chr = *chr.unwrap();
    char_idx += 1;

    if chr.is_whitespace() {
      if !curr_str.is_empty() {
        result.push(std::mem::take(&mut curr_str));
      }
      curr_str.push(chr);
      let mut cont = false;
      while let Some(&chr) = chrs.get(char_idx) {
        if !chr.is_whitespace() {
          if !curr_str.is_empty() {
            result.push(std::mem::take(&mut curr_str));
          }
          cont = true;
          break;
        }
        char_idx += 1;
        curr_str.push(chr);
      }

      if cont {
        continue;
      }
    } else if chr == '\\' {
      if !curr_str.is_empty() {
        result.push(std::mem::take(&mut curr_str));
      }
      if chrs.get(char_idx) == Some(&'*') {
        char_idx += 1;
        curr_str = "\\*".to_owned();
      } else {
        curr_str.push('\\');
      }
      result.push(std::mem::take(&mut curr_str));
    } else if chr == '*' {
      if !curr_str.is_empty() {
        result.push(std::mem::take(&mut curr_str));
      }
      curr_str.push(chr);
      result.push(std::mem::take(&mut curr_str));
    } else {
      curr_str.push(chr);
    }
  }
  result
}

pub fn encode_sjis(unicode: &str) -> Vec<u8> {
  use encoding_rs::SHIFT_JIS;
  let mut in_italics = false;
  let mut collector = vec![];
  let tokens = tokens(unicode);
  let mut bslash_active = false;

  for substr in tokens {
    let substr = substr;

    if substr == "\\" && !bslash_active {
      bslash_active = true;
      continue;
    }

    if substr == "*" {
      if in_italics {
        in_italics = false;
      } else {
        in_italics = true;
      }
      continue;
    }

    if bslash_active {
      bslash_active = false;
    }

    if substr == "\\*" {
      collector.push(['*' as u8].to_vec());
      continue;
    }

    if in_italics {
      let mut word = vec![];
      for chr in substr.chars() {
        let code = ITALIC_MAPPING_TABLE.get_by_left(&chr);
        if let Some(code) = code {
          word.extend(code.to_vec().into_iter());
        } else {
          word.extend(SHIFT_JIS.encode(&format!("{chr}")).0.iter());
        }
      }
      let output = word;
      collector.push(output);
    } else {
      let output = SHIFT_JIS.encode(&substr).0.to_vec();
      collector.push(output);
    }
  }

  let output: Vec<u8> = collector.into_iter().flatten().collect();

  output
}

pub fn get_sjis_bytes(address: usize, input: &[u8]) -> (Vec<u8>, String) {
  let mut size = 0usize;
  let mut output = vec![];
  while input[address + size] != 0 && size < 1024 {
    output.push(input[address + size]);
    size += 1;
  }
  output.push(0);
  use encoding_rs::SHIFT_JIS;
  let last_idx = output.len() - 1;
  let encoded = SHIFT_JIS.decode(&output[..last_idx]).0.to_string();
  (output, encoded)
}

#[macro_export]
macro_rules! opcode_common_action {
  ($self: ident, $op: ident, $action: block, $array_action: block) => {
    match $self {
      Opcode::OP_RESET($op) => $action,
      Opcode::OP_DIRECT_JUMP($op) => $action,
      Opcode::OP_JUMP_TO_SCRIPT($op) => $action,
      Opcode::OP_03($op) => $action,
      Opcode::OP_04($op) => $action,
      Opcode::OP_SCRIPT_RETURN($op) => $action,
      Opcode::JNE($op) => $action,
      Opcode::JE($op) => $action,
      Opcode::JLE($op) => $action,
      Opcode::JL($op) => $action,
      Opcode::JGE($op) => $action,
      Opcode::JG($op) => $action,
      Opcode::JNZ($op) => $action,
      Opcode::JZ($op) => $action,
      Opcode::Switch($op) => $action,
      Opcode::OP_10($op) => $action,
      Opcode::OP_11($op) => $action,
      Opcode::OP_12($op) => $action,
      Opcode::OP_13($op) => $action,
      Opcode::OP_14($op) => $action,
      Opcode::OP_15($op) => $action,
      Opcode::OP_16($op) => $action,
      Opcode::OP_17($op) => $action,
      Opcode::OP_1A($op) => $action,
      Opcode::OP_1B($op) => $action,
      Opcode::OP_1C($op) => $action,
      Opcode::OP_1D($op) => $action,
      Opcode::OP_1E($op) => $action,
      Opcode::OP_1F($op) => $action,
      Opcode::OP_20($op) => $action,
      Opcode::OP_21($op) => $action,
      Opcode::OP_22($op) => $action,
      Opcode::OP_23($op) => $action,
      Opcode::OP_24($op) => $action,
      Opcode::OP_25($op) => $action,
      Opcode::OP_2D($op) => $action,
      Opcode::OP_2E($op) => $action,
      Opcode::OP_2F($op) => $action,
      Opcode::OP_30($op) => $action,
      Opcode::OP_CHOICE($op) => $action,
      Opcode::OP_MENU_CHOICE($op) => $action,
      Opcode::OP_33($op) => $action,
      Opcode::OP_34($op) => $action,
      Opcode::OP_36($op) => $action,
      Opcode::OP_39($op) => $action,
      Opcode::OP_3A($op) => $action,
      Opcode::OP_3B($op) => $action,
      Opcode::OP_3C($op) => $action,
      Opcode::OP_42($op) => $action,
      Opcode::OP_43($op) => $action,
      Opcode::OP_PLAY_VOICE($op) => $action,
      Opcode::OP_TEXTBOX_DISPLAY($op) => $action,
      Opcode::OP_FREE_TEXT_OR_CHARNAME($op) => $action,
      Opcode::OP_48($op) => $action,
      Opcode::OP_CLEAR_SCREEN($op) => $action,
      Opcode::OP_WAIT($op) => $action,
      Opcode::OP_4B($op) => $action,
      Opcode::OP_4C($op) => $action,
      Opcode::OP_4F($op) => $action,
      Opcode::OP_51($op) => $action,
      Opcode::OP_59($op) => $action,
      Opcode::OP_5A($op) => $action,
      Opcode::OP_5F($op) => $action,
      Opcode::OP_68($op) => $action,
      Opcode::OP_69($op) => $action,
      Opcode::OP_6A($op) => $action,
      Opcode::OP_6C($op) => $action,
      Opcode::OP_6E($op) => $action,
      Opcode::OP_6F($op) => $action,
      Opcode::OP_71($op) => $action,
      Opcode::OP_72($op) => $action,
      Opcode::OP_74($op) => $action,
      Opcode::OP_75($op) => $action,
      Opcode::OP_CUSTOM_TIP_77($op) => $action,
      Opcode::OP_7B($op) => $action,
      Opcode::OP_82($op) => $action,
      Opcode::OP_83($op) => $action,
      Opcode::OP_DEBUG_PRINT($op) => $action,
      Opcode::OP_SPECIAL_TEXT($op) => $action,
      Opcode::OP_Insert($op) => $array_action,
    }
  };
}

pub fn fix_string(input: &str) -> String {
  let words = input.split(" ").collect::<Vec<_>>();
  let mut output = words[0].to_string();

  for word in words.iter().skip(1) {
    let new_len = (output.len() as isize + word.len() as isize) % 60;
    let curr_len = output.len() as isize % 60;

    // If we wrap over after adding the current word, we need to insert a %N
    let spaces = if new_len - curr_len < 0 {
      "%N".to_string()
    } else {
      " ".to_string()
    };

    output.push_str(&spaces);
    output.push_str(word);
  }

  output.trim().to_string()
}

pub fn fix_line(line: &str) -> String {
  let Some(unicode_end) = line.find("unicode: ") else {
    return line.to_string();
  };
  let comment_start = line.find(" #").unwrap_or(line.len());
  if line.ends_with("nofix") || line.contains("%N") {
    return line.to_string();
  }

  let input_str = &line[(unicode_end + 9)..comment_start];

  if input_str.len() >= 180 {
    log::warn!("Line {input_str} contains more than 180 characters, it won't render properly in the game! For best results, split this text across two print opcodes.");
  }

  line.replace(input_str, &fix_string(input_str.trim_matches([' ', '"'])))
}

#[cfg(test)]
pub(crate) mod tests {
  #[test]
  fn test_thing() {
    assert_eq!("Though, as a consolation, you'd find a proper road if you%Nwere to travel towards a different peak in the opposite%Ndirection instead.", &super::fix_string("Though, as a consolation, you'd find a proper road if you were to travel towards a different peak in the opposite direction instead."));
    assert_eq!(
      "The only ways to commute to the city are by train, or via an unpaved mountain road.",
      &super::fix_string(
        "The only ways to commute to the city are by train, or via an unpaved mountain road."
      )
    );
    assert_eq!(
      "If you made the mistake of going on a hike with your waifu%Nand son after looking at a map and making a mole hill of a%Nmountain, it would probably result in a family tragedy.", 
      &super::fix_string("If you made the mistake of going on a hike with your waifu and son after looking at a map and making a mole hill of a mountain, it would probably result in a family tragedy.")
    );
  }

  #[test]
  fn test_thing2() {
    assert_eq!(
      "unicode: Though, as a consolation, you'd find a proper road if you%Nwere to travel towards a different peak in the opposite%Ndirection instead. # comment",
      &super::fix_line(
        "unicode: Though, as a consolation, you'd find a proper road if you were to travel towards a different peak in the opposite direction instead. # comment"
      )
    );

    assert_eq!(
      r#"    unicode: The only ways to commute to the city are by train, or via an unpaved mountain road. #comment"#,
      &super::fix_line(
        r#"    unicode: "The only ways to commute to the city are by train, or via an unpaved mountain road." #comment"#
      )
    );

    assert_eq!(
      r#"    unicode: not to mention rental shops galore, as well as arcades,%Nbookstores, bars, and establishments to deliver one some%N"healing". # ちなみに向こう側にはデパートもあり、時間さえかければ都心にも出られる路線がそのつま先を置き、レンタルショップはおろかゲーセン・本屋・飲み屋・出張（ヘルス）まである。"#,
      &super::fix_line(
        r#"    unicode: not to mention rental shops galore, as well as arcades, bookstores, bars, and establishments to deliver one some "healing". # ちなみに向こう側にはデパートもあり、時間さえかければ都心にも出られる路線がそのつま先を置き、レンタルショップはおろかゲーセン・本屋・飲み屋・出張（ヘルス）まである。"#
      )
    );

    assert_eq!(
      r#"    unicode: Tip - The original uses a term called "Delivery health",%Nwhich is a synonym for a call girl agency."#,
      &super::fix_line(
        r#"    unicode: Tip - The original uses a term called "Delivery health", which is a synonym for a call girl agency."#
      )
    );
  }
}
