use rayon::{
  prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
  vec,
};

use crate::util::{escape_str, unescape_str};
use crate::{
  lz77,
  opcodescript::{
    Choice, ChoiceOpcode, Opcode, Quirks, String47Opcode, StringOpcode, StringOpcode2,
  },
  scenario_pack::{parse_scenario, DirEntry},
  util::fix_line,
  Script,
};
use once_cell::sync::Lazy;
use std::{
  collections::HashMap,
  io::Read,
  path::{Path, PathBuf},
};

pub fn do_archive_command(
  top_dir: &Path,
  text_script_dir: &Path,
  outfile: &Path,
  compress: bool,
  apply_text: bool,
) {
  let directory: Vec<DirEntry> =
    serde_yml::from_str(&std::fs::read_to_string(&top_dir.join("directory.yaml")).unwrap())
      .unwrap();

  let scripts: Vec<(String, Script)> = directory
    .into_par_iter()
    .map(|DirEntry { name, .. }| {
      let path = top_dir.join(&name);
      log::debug!("Opening {}", path.display());
      let mut script: Script = serde_yml::from_slice(&std::fs::read(&path).unwrap()).unwrap();
      if apply_text {
        let text_path = text_script_dir.join(name).with_extension("txt");
        let text = std::fs::read_to_string(text_path).unwrap_or_default();
        tl_reverse_transform_script(&mut script, &text);
      }
      (path.display().to_string(), script)
    })
    .collect();

  let n_scripts = scripts.len();

  let (directory, scripts_concat, scripts) = recompile_scripts(scripts, n_scripts);

  if outfile.display().to_string().ends_with('/') {
    std::fs::create_dir_all(&outfile).unwrap();
    for (path, script) in scripts {
      let newpath = path
        .rsplit_once('/')
        .unwrap()
        .1
        .replace("yaml", "opcodescript");
      let newpath = outfile.join(newpath);
      std::fs::write(newpath, script).unwrap();
    }
  } else {
    let result: Vec<u8> = directory
      .chain(scripts_concat.as_slice())
      .bytes()
      .map(Result::unwrap)
      .collect();

    let compressed = if compress {
      lz77::lz77_compress(&result)
    } else {
      result
    };

    std::fs::write(outfile, compressed).unwrap();
  }
}

fn recompile_scripts(
  scripts: Vec<(String, Script)>,
  n_scripts: usize,
) -> (Vec<u8>, Vec<u8>, Vec<(String, Vec<u8>)>) {
  let (_, directory, scripts_concat, scripts) = scripts
    .into_iter()
    .map(|(path, it)| {
      log::debug!("Serializing {path}.");
      let serialized = it.binary_serialize();
      (serialized.len(), path, serialized)
    })
    .fold(
      (16 * n_scripts, vec![], vec![], vec![]),
      |(script_start, mut directory, mut scripts_concat, mut scripts),
       (this_script_len, path, this_script)| {
        directory.extend((script_start as u32).to_le_bytes());
        directory.extend((this_script_len as u32).to_le_bytes());
        directory.extend([0u8; 8].into_iter());

        scripts.push((path, this_script.clone()));
        scripts_concat.extend(this_script);

        (
          script_start + this_script_len,
          directory,
          scripts_concat,
          scripts,
        )
      },
    );
  (directory, scripts_concat, scripts)
}

pub fn do_extract_command(data: Vec<u8>, outfile: &PathBuf, quirks: Quirks) {
  let decompressed_data = lz77::lz77_decompress(&data);

  let scripts = parse_scenario(&decompressed_data, quirks);

  log::info!("Scenario file is parsed.");
  log::info!("Writing decoded scripts to directory {}", outfile.display());

  std::fs::create_dir_all(outfile).unwrap();

  std::fs::write(
    &outfile.join("directory.yaml"),
    serde_yml::to_string(&scripts.keys().collect::<Vec<_>>()).unwrap(),
  )
  .unwrap();

  scripts
    .par_iter()
    .for_each(|(DirEntry { name, .. }, script)| {
      let res = std::fs::write(
        &outfile.join(&name).with_extension("yaml"),
        script2yaml(script),
      );
      if let Err(e) = res {
        log::error!("Encountered an error when writing {name}: {}", e);
      }
    });
}

pub fn do_unpack_command(data: Vec<u8>, outfolder: &Path, scriptfolder: &Path, quirks: Quirks) {
  let decompressed_data = lz77::lz77_decompress(&data);

  let scripts = parse_scenario(&decompressed_data, quirks);

  log::info!("Scenario file is parsed.");
  log::info!(
    "Writing decoded scripts to directory {}",
    outfolder.display()
  );

  std::fs::create_dir_all(outfolder).unwrap();
  std::fs::create_dir_all(&scriptfolder).unwrap();

  std::fs::write(
    &outfolder.join("directory.yaml"),
    serde_yml::to_string(&scripts.keys().collect::<Vec<_>>()).unwrap(),
  )
  .unwrap();

  scripts
    .par_iter()
    .for_each(|(DirEntry { name, .. }, script)| {
      let res = std::fs::write(
        &outfolder.join(&name).with_extension("yaml"),
        script2yaml(script),
      )
      .and_then(|_| {
        std::fs::write(
          &scriptfolder.join(&name).with_extension("txt"),
          tl_transform_script(script),
        )
      });

      if let Err(e) = res {
        log::error!("Encountered an error when writing {name}: {}", e);
      }
    });
}

pub(crate) fn script2yaml(script: &Script) -> String {
  serde_yml::to_string(&script)
    .unwrap()
    .replace("'[", "[")
    .replace("]'", "]")
    .replace(r#"'""#, "")
    .replace(r#""'"#, "")
}

pub fn do_reencode_command(outfile: &Path, filename: &Path) {
  let outfile = if outfile.is_dir() {
    outfile.join(filename.with_extension("opcodescript").file_name().unwrap())
  } else {
    outfile.to_owned()
  };

  let script = serde_yml::from_str::<Script>(&std::fs::read_to_string(&filename).unwrap()).unwrap();
  log::info!("Serializing {}.", outfile.display());
  std::fs::write(outfile, script.binary_serialize()).unwrap();
}

pub fn do_decode_command(outfile: &Path, filename: &Path, quirks: Quirks) {
  let script = decode_opcodescript(filename, quirks);
  let outfile = if outfile.extension().unwrap().to_string_lossy() != "yaml" {
    if !outfile.exists() {
      std::fs::create_dir_all(outfile)
        .expect(&format!("Could not create directory {}", outfile.display()));
    }
    outfile.join(filename.with_extension("yaml").file_name().unwrap())
  } else {
    outfile.to_owned()
  };
  log::info!("writing output to {}", outfile.display());
  std::fs::write(outfile, script2yaml(&script)).unwrap();
}

fn decode_opcodescript(filename: &Path, quirks: Quirks) -> Script {
  log::info!("decoding file {}", filename.display());
  let data = std::fs::read(filename).unwrap();

  let (script, error) = Script::new(&data, quirks);

  if let Some(error) = error {
    log::error!(
      "Encountered an error ({error}) while decoding file {}",
      filename.display()
    );
  }

  script
}
#[derive(PartialOrd, PartialEq)]
enum LineState {
  Nothing,
  TL,
  Notes,
  ChoiceTL,
  ChoiceNotes,
}

struct DocLine {
  speaker_address: u32,
  speaker_translation: String,
  address: u32,
  translation: String,
  notes: String,
  choices: Vec<(String, String)>,
}

impl DocLine {
  fn new() -> Self {
    DocLine {
      speaker_address: 0,
      speaker_translation: String::default(),
      address: 0,
      translation: String::default(),
      notes: String::default(),
      choices: vec![],
    }
  }
}

pub fn tl_reverse_transform_script(script: &mut Script, tl_doc: &str) {
  let mut text2addr: HashMap<u32, &mut Opcode> = HashMap::new();
  for opcode in script.opcodes.iter_mut() {
    if ![0x47, 0x45, 0x86, 0x31, 0x32].contains(&opcode.opcode()) {
      continue;
    }

    match opcode {
      Opcode::OP_FREE_TEXT_OR_CHARNAME(_)
      | Opcode::OP_TEXTBOX_DISPLAY(_)
      | Opcode::OP_SPECIAL_TEXT(_)
      | Opcode::OP_47_TEXT(_)
      | Opcode::OP_CHOICE(_)
      | Opcode::OP_MENU_CHOICE(_) => {
        text2addr.insert(opcode.address(), opcode);
      }
      _ => {}
    }
  }

  let mut doclines = vec![];
  let mut curr_line = DocLine::new();

  let mut line_state = LineState::Nothing;
  for line in tl_doc.lines() {
    if line.starts_with("[speaker @ 0x") {
      let (speaker_address, speaker_text) = parse_tl_doc_line(&line, 13, true);

      curr_line.speaker_address = speaker_address;
      curr_line.speaker_translation = speaker_text;
    } else if line.starts_with("[original text @ 0x") {
      let (address, _) = parse_tl_doc_line(&line, 19, false);

      curr_line.address = address;
    } else if line.starts_with("[choices @ 0x") {
      let (address, _) = parse_tl_doc_line(&line, 13, false);

      curr_line.address = address;
    } else if line.starts_with("[choice translation]:") {
      let text = line[21..].trim().to_string();
      curr_line.choices.push((text, String::default()));
      line_state = LineState::ChoiceTL;
    } else if line.starts_with("[choice notes]:") {
      let text = line[15..].trim().to_string();
      curr_line.choices.last_mut().unwrap().1.push_str(&text);
      line_state = LineState::ChoiceNotes;
    } else if line.starts_with("[translation]:") {
      let text = line[14..].trim().to_string();
      curr_line.translation = text;
      line_state = LineState::TL;
    } else if line.starts_with("[notes]:") {
      if line_state == LineState::TL {
        line_state = LineState::Notes;
      }

      let text = line[8..].trim().to_string();
      curr_line.notes = text;
    } else if line == TL_LINE_END.as_str() {
      line_state = LineState::Nothing;
      doclines.push(curr_line);
      curr_line = DocLine::new();
    } else if line == TL_CHOICE_END.as_str() {
      line_state = LineState::Nothing;
    } else {
      match line_state {
        LineState::Nothing => continue,
        LineState::TL => curr_line.translation.push_str(&("\n".to_string() + line)),
        LineState::Notes => curr_line.notes.push_str(&("\n".to_string() + line)),
        LineState::ChoiceTL => curr_line
          .choices
          .last_mut()
          .unwrap()
          .0
          .push_str(&("\n".to_string() + line)), // Add to choice tl.
        LineState::ChoiceNotes => curr_line
          .choices
          .last_mut()
          .unwrap()
          .1
          .push_str(&("\n".to_string() + line)), // Add to choice notes.
      }
    }
  }

  for line in doclines {
    if line.speaker_address != 0 {
      let speaker_op = text2addr.get_mut(&line.speaker_address);
      if let Some(Opcode::OP_FREE_TEXT_OR_CHARNAME(op)) = speaker_op {
        op.translation = Some(line.speaker_translation);
      }
    }

    let op = text2addr.get_mut(&line.address);
    match op {
      Some(Opcode::OP_FREE_TEXT_OR_CHARNAME(ref mut op)) => {
        op.translation = if !line.translation.trim().is_empty() {
          Some(escape_str(&line.translation))
        } else {
          None
        };
        op.notes = if !line.notes.trim().is_empty() {
          Some(escape_str(&line.notes))
        } else {
          None
        };
      }
      Some(Opcode::OP_TEXTBOX_DISPLAY(op)) => {
        op.translation = if !line.translation.trim().is_empty() {
          Some(escape_str(&line.translation))
        } else {
          None
        };
        op.notes = if !line.notes.trim().is_empty() {
          Some(escape_str(&line.notes))
        } else {
          None
        };
      }
      Some(Opcode::OP_SPECIAL_TEXT(op)) => {
        op.translation = if !line.translation.trim().is_empty() {
          Some(escape_str(&line.translation))
        } else {
          None
        };
        op.notes = if !line.notes.trim().is_empty() {
          Some(escape_str(&line.notes))
        } else {
          None
        };
      }
      Some(Opcode::OP_47_TEXT(op)) => {
        op.translation = if !line.translation.trim().is_empty() {
          Some(escape_str(&line.translation))
        } else {
          None
        };
        op.notes = if !line.notes.trim().is_empty() {
          Some(escape_str(&line.notes))
        } else {
          None
        };
      }
      Some(Opcode::OP_CHOICE(op)) | Some(Opcode::OP_MENU_CHOICE(op)) => {
        for (choice, c_tl, c_note) in op
          .choices
          .iter_mut()
          .zip(line.choices)
          .map(|(a, (b, c))| (a, b, c))
        {
          choice.translation = if !c_tl.trim().is_empty() {
            Some(escape_str(&c_tl))
          } else {
            None
          };
          choice.notes = if !c_note.trim().is_empty() {
            Some(escape_str(&c_note))
          } else {
            None
          };
        }
      }
      _ => {}
    }
  }
}

fn parse_tl_doc_line(line: &str, prefix_size: usize, is_speaker: bool) -> (u32, String) {
  let chars = line.chars().collect::<Vec<_>>();
  let mut curr = prefix_size;
  while chars[curr] != ']' {
    curr += 1;
  }
  let data = &line[prefix_size..curr];
  let address = u32::from_str_radix(data.trim(), 16).unwrap();

  curr += 1; // Skip the ']:'
  if curr == line.len() {
    return (address, String::default());
  }
  if chars[curr] == ':' {
    curr += 1;
  }
  if is_speaker {
    let mut new_curr = curr;
    while chars[new_curr] != '(' {
      new_curr += 1;
    }

    let speaker_text = (&line[(curr)..new_curr].trim()).to_string();
    (address, speaker_text)
  } else {
    (address, line[curr..].to_string())
  }
}

const TL_CHOICE_END: Lazy<String> = Lazy::new(|| "---~~~---".to_string());
const TL_LINE_END: Lazy<String> = Lazy::new(|| "---===---".to_string());

pub fn tl_transform_script(input: &Script) -> String {
  let mut lines = vec![];

  let mut curr_speaker = ("", String::default(), &0);
  for opcode in input.opcodes.iter() {
    if ![0x47, 0x45, 0x86, 0x31, 0x32].contains(&opcode.opcode()) {
      continue;
    }

    match opcode.opcode() {
      0x47 => {
        if let Opcode::OP_FREE_TEXT_OR_CHARNAME(String47Opcode {
          address,
          opt_arg2,
          unicode,
          translation,
          notes,
          ..
        }) = opcode
        {
          let tl_text = translation
            .as_ref()
            .map(|it| unescape_str(it.as_str()))
            .unwrap_or_default();
          let note_text = notes
            .as_ref()
            .map(|it| unescape_str(it.as_str()))
            .unwrap_or_default();
          if opt_arg2.is_none() {
            curr_speaker = (unicode.as_str(), tl_text, address);
            continue;
          } else {
            // lines.push(format!("index {}", i + 1));
            if !curr_speaker.0.is_empty() {
              lines.push(format!(
                "[speaker @ 0x{:08X}]: {} ({})",
                curr_speaker.2, curr_speaker.1, curr_speaker.0
              ));
              curr_speaker.0 = "";
            }

            lines.push(format!("[original text @ 0x{address:08X}]: {unicode}"));
            lines.push(format!("[translation]: {tl_text}"));
            lines.push(format!("[notes]: {note_text}"));
          }
        } else if let Opcode::OP_47_TEXT(StringOpcode2 {
          address,
          unicode,
          notes,
          translation,
          ..
        }) = opcode
        {
          let tl_text = translation
            .as_ref()
            .map(|it| unescape_str(it.as_str()))
            .unwrap_or_default();
          let note_text = notes
            .as_ref()
            .map(|it| unescape_str(it.as_str()))
            .unwrap_or_default();

          // lines.push(format!("index {}", i + 1));
          if !curr_speaker.0.is_empty() {
            lines.push(format!(
              "[speaker @ 0x{:08X}]: {} ({})",
              curr_speaker.2, curr_speaker.1, curr_speaker.0
            ));
            curr_speaker.0 = "";
          }

          lines.push(format!("[original text @ 0x{address:08X}]: {unicode}"));
          lines.push(format!("[translation]: {tl_text}"));
          lines.push(format!("[notes]: {note_text}"));
        }
      }
      0x45 | 0x86 => {
        if let Opcode::OP_TEXTBOX_DISPLAY(StringOpcode {
          address,
          unicode,
          notes,
          translation,
          ..
        }) = opcode
        {
          let tl_text = translation
            .as_ref()
            .map(|it| unescape_str(it.as_str()))
            .unwrap_or_default();
          let note_text = notes
            .as_ref()
            .map(|it| unescape_str(it.as_str()))
            .unwrap_or_default();

          if !curr_speaker.0.is_empty() {
            lines.push(format!(
              "[speaker @ 0x{:08X}]: {} ({})",
              curr_speaker.2, curr_speaker.1, curr_speaker.0
            ));
            curr_speaker.0 = "";
          }

          lines.push(format!("[original text @ 0x{address:08X}]: {unicode}"));
          lines.push(format!("[translation]: {tl_text}"));
          lines.push(format!("[notes]: {note_text}"));
        } else if let Opcode::OP_SPECIAL_TEXT(StringOpcode {
          address,
          unicode,
          notes,
          translation,
          ..
        }) = opcode
        {
          let tl_text = translation
            .as_ref()
            .map(|it| unescape_str(it.as_str()))
            .unwrap_or_default();
          let note_text = notes
            .as_ref()
            .map(|it| unescape_str(it.as_str()))
            .unwrap_or_default();

          if !curr_speaker.0.is_empty() {
            lines.push(format!(
              "[speaker @ 0x{:08X}]: {} ({})",
              curr_speaker.2, curr_speaker.1, curr_speaker.0
            ));
            curr_speaker.0 = "";
          }

          lines.push(format!("[original text @ 0x{address:08X}]: {unicode}"));
          lines.push(format!("[translation]: {tl_text}"));
          lines.push(format!("[notes]: {note_text}"));
        }
      }
      0x31 | 0x32 => {
        if let Opcode::OP_CHOICE(ChoiceOpcode {
          address, choices, ..
        })
        | Opcode::OP_MENU_CHOICE(ChoiceOpcode {
          address, choices, ..
        }) = opcode
        {
          // lines.push(format!("index {}", i + 1));
          lines.push(format!("[choices @ 0x{address:08X}]"));
          for (
            j,
            Choice {
              unicode,
              notes,
              translation,
              ..
            },
          ) in choices.iter().enumerate()
          {
            let tl_text = translation
              .as_ref()
              .map(|it| unescape_str(it.as_str()))
              .unwrap_or_default();
            let note_text = notes
              .as_ref()
              .map(|it| unescape_str(it.as_str()))
              .unwrap_or_default();
            lines.push(format!("[choice original text]: {unicode}"));
            lines.push(format!("[choice translation]: {tl_text}"));
            lines.push(format!("[choice notes]: {note_text}"));
            lines.push(TL_CHOICE_END.clone());
          }
        }
      }
      _ => continue,
    }
    lines.push(TL_LINE_END.clone());
    lines.push("\n".to_string());
  }

  lines.join("\n")
}

pub fn do_fix_command(input_file: &PathBuf, outfile: PathBuf) {
  let data = std::fs::read_to_string(input_file).unwrap();
  let mut output = vec![];

  for line in data.lines() {
    output.push(fix_line(line));
  }

  let output = output.join("\n") + "\n";

  std::fs::write(&outfile, output).unwrap();
}

#[cfg(test)]
mod test {
  use crate::opcodescript::Script;

  use super::{tl_reverse_transform_script, tl_transform_script};

  #[test]
  fn test_transform() {
    let input = include_str!("/home/wscp/cc_tl/scenario/0045.yaml");

    let script: Script = serde_yml::from_str(input).unwrap();

    let translated_str = tl_transform_script(&script);
    assert!(!translated_str.is_empty());
    println!("{translated_str}");

    let mut new_script = script.clone();
    tl_reverse_transform_script(&mut new_script, &translated_str);
    assert_eq!(new_script, script);
  }
}
