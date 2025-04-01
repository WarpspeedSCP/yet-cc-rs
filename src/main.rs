use std::path::Path;
use std::path::PathBuf;

mod commands;
mod lz77;
mod opcodescript;
mod scenario_pack;
mod util;

use clap::*;
use commands::*;
use opcodescript::*;
use walkdir::WalkDir;

fn get_final_file_list<T>(input_files: Vec<T>, recurse_dirs: bool, action: &str) -> Vec<PathBuf>
where
  T: AsRef<Path> + Sync + Send,
{
  use rayon::prelude::*;
  if !recurse_dirs {
    input_files
      .par_iter()
      .map(|it| it.as_ref().to_path_buf())
      .filter(|it| {
        if action == "decode" {
          it.extension().unwrap_or_default() == "opcodescript"
        } else {
          true
        }
      })
      .collect()
  } else {
    input_files
      .par_iter()
      .flat_map(|file| {
        WalkDir::new(file.as_ref())
          .contents_first(true)
          .into_iter()
          .filter_map(Result::ok)
          .filter(|it| it.path().is_file())
          .map(|it| it.into_path())
          .filter(|it| {
            if action == "decode" {
              it.extension().unwrap_or_default() == "opcodescript"
            } else {
              false
            }
          })
          .collect::<Vec<_>>()
      })
      .collect()
  }
}

fn main() {
  env_logger::builder().format_timestamp(None).init();

  let mut app = clap::Command::new("yeti")
    .about("A tool to decode and re-encode scenario files for CROSS†CHANNEL final complete (for PC).")
    .arg_required_else_help(true)
    .arg(
      Arg::new("action")
        .num_args(1)
        .action(ArgAction::Set)
        .required(true).value_parser(["encode", "decode", "recomp", "decomp", "fix", "mk_uncompbin"])
        .long_help(r#"The action to take.
* encode:       Create an opcodescript file from the given input yaml file.
                - Can take a single, or multiple yaml files depending on if you pass a file or a directory to --input.
* decode:       Create a script yaml file from the given opcodescript file.
                - Can take a single, or multiple opcodescript files depending on if you pass a file or a directory to --input. 
                - Make sure to add --all if you're decoding a directory.
                - No guarantees that this will work if the script contains any inserted or tip opcodes.
* decomp:       Decompress and decode an sn.bin file into a set of yaml files.
                - Pass the sn.bin file to --input, and an output directory to --output.
* recomp:       Encode and "compress" a directory with yaml files into a new sn.bin file.
                - Pass a directory with script yaml files (and the directory json) to --input, and the output file name to --output.
* mk_uncompbin: Create an uncompressed sn.bin file from the given yaml folder. Useful when debugging crashes sometimes.
                - Pass a directory with script yaml files (and the directory json) to --input, and the output file name to --output.
* fix:          Fix any text spacing issues with a given yaml file.
                - Pass a single yaml file as input, and the program will attempt to fix any text formatting mistakes. Warnings will be raised if any lines are too long to display correctly in textboxes."#),
    )
    .arg(
      Arg::new("input")
        .short('i')
        .long("input")
        .action(ArgAction::Append)
        .required(true)
        .help("The input file/folder to use.")
        .value_name("file")
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("output")
        .short('o')
        .long("output")
        .action(ArgAction::Append)
        .required(true)
        .help("The output file/folder to use.")
        .value_name("file")
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("all")
        .short('a')
        .long("all")
        .help("idk what this does at this point, tbh.")
        .action(ArgAction::SetTrue),
    );

  app.build();

  let help_text = app.render_help();

  let matches = app.get_matches();
  let action = matches
    .get_one::<String>("action")
    .cloned()
    .to_owned()
    .unwrap_or_default();
  let in_dir = matches.get_flag("all");
  let infiles: Vec<String> = matches
    .get_many("input")
    .map(Iterator::cloned)
    .map(Iterator::collect)
    .unwrap_or_default();
  log::debug!("parsing {} file(s)", infiles.len());
  for i in &infiles {
    log::debug!("{i}");
  }
  let outfile: PathBuf = matches
    .get_one("output")
    .map(|it: &String| PathBuf::from(it))
    .or_else(|| Some(std::env::temp_dir()))
    .expect("Expected a valid output file");

  if in_dir && outfile.is_file() {
    eprintln!("Expected a directory for --output, got a file path.");
    return;
  }

  let file_list = get_final_file_list(infiles, in_dir && action != "recomp", &action);

  use rayon::prelude::*;

  match action.as_str() {
    "encode" => {
      file_list.par_iter().for_each(|file| {
        do_reencode_command(&outfile, &file);
      });
    }
    "decode" => {
      file_list.par_iter().for_each(|file| {
        do_decode_command(&outfile, &file);
      });
    }
    "decomp" => {
      let input_file = file_list.first().expect(&help_text.ansi().to_string());

      let data = std::fs::read(input_file).unwrap();

      do_extract_command(data, &outfile);
    }
    "mk_uncompbin" => {
      let top_dir = file_list.first().expect(&help_text.ansi().to_string());
      do_archive_command(top_dir, outfile, false);
    }
    "recomp" => {
      let top_dir = file_list.first().expect(&help_text.ansi().to_string());
      do_archive_command(top_dir, outfile, true);
    }
    "fix" => {
      let input_file = file_list.first().expect(&help_text.ansi().to_string());

      do_fix_command(input_file, outfile);
    }
    _ => panic!("{help_text}"),
  }
}
