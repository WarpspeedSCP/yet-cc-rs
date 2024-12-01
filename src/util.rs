pub fn transmute_to_array<const SIZE: usize>(address: usize, input: &[u8]) -> [u8; SIZE] {
  input[address..address + SIZE].try_into().unwrap()
}

pub fn transmute_to_u32(address: usize, input: &[u8]) -> u32 {
  u32::from_le_bytes(transmute_to_array(address, input))
}

pub fn transmute_to_u16(address: usize, input: &[u8]) -> u16 {
  u16::from_le_bytes(transmute_to_array(address, input))
}

pub fn get_sjis_bytes(address: usize, input: &[u8]) -> (Vec<u8>, String) {
  let mut size = 0usize;
  let mut output = vec![];
  while input[address + size] != 0 && size < 500 {
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
      Opcode::OP_TEXTBOX_CHARNAME($op) => $action,
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
