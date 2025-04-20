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
  let filter_func = |it: &PathBuf| {
    let value = it.extension().unwrap_or_default();
    if it.file_stem().unwrap_or_default() == "directory" {
      return false;
    }
    if action == "decode" {
      value == "opcodescript" || value == "yaml"
    } else if action == "transform" {
      value == "yaml"
    } else {
      true
    }
  };
  
  use rayon::prelude::*;
  let output = if !recurse_dirs {
    input_files
      .par_iter()
      .map(|it| it.as_ref().to_path_buf())
      .filter(filter_func)
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
          .filter(filter_func)
          .collect::<Vec<_>>()
      })
      .collect()
  };
  
  output
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
        .required(true)
        .value_parser(["encode", "decode", "recomp", "decomp", "fix", "mk_uncompbin", "transform", "untransform", "pack", "unpack"])
        .long_help(r#"The action to take.
* encode:       Create an opcodescript file from the given input yaml file.
                - Can take a single, or multiple yaml files depending on if you pass a file or a directory to --input.
* decode:       Create a script yaml file from the given opcodescript file.
                - Can take a single, or multiple opcodescript files depending on if you pass a file or a directory to --input. 
                - Make sure to add --all if you're decoding a directory.
                - No guarantees that this will work if the script contains any inserted or tip opcodes.
* decomp:       Decompress and decode an sn.bin file into a set of editable text files.
                - Pass the sn.bin file to --input, and an output directory to --output.
* recomp:       Encode and "compress" a directory with yaml files into a new sn.bin file.
                - Pass a directory with script yaml files (and the directory json) to --input, and the output file name to --output.
* mk_uncompbin: Create an uncompressed sn.bin file from the given yaml folder. Useful when debugging crashes sometimes.
                - Pass a directory with script yaml files (and the directory json) to --input, and the output file name to --output.
* transform:    Converts yaml files into easily editable text files.
* untransform:  Converts script text files back to yaml files.
* unpack        Creates a script directory containing all scripts in text form.
* pack          Creates an sn.bin file from the given text script directory (not to be confused with recomp).
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
        .required(false)
        .help("The output file/folder to use.")
        .value_name("file")
        .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("text_dir")
          .short('t')
          .long("textdir")
          .action(ArgAction::Append)
          .required(false)
          .help("The directory where text scripts are kept. Only useful for the transform and untransform commands.")
          .value_name("file")
          .value_hint(ValueHint::AnyPath),
    )
    .arg(
      Arg::new("all")
        .short('a')
        .long("all")
        .help("set this if you want to recurse over a directory containing all script files.")
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("quirks")
        .short('q')
        .long("quirks")
        .help("Specify quirks to apply when running the tool. helps decode games made for xbox and psp sometimes.")
        .required(false),
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
    .unwrap();
  
  let text_script_dir: PathBuf = matches
      .get_one("text_dir")
      .map(|it: &String| PathBuf::from(it))
      .or_else(|| Some(PathBuf::from(infiles.first().unwrap()).join("scripts")))
      .unwrap();

  if in_dir && outfile.is_file() {
    eprintln!("Expected a directory for --output, got a file path.");
    return;
  }
  
  let infiles = infiles.into_iter().map(|it| PathBuf::from(it).canonicalize().unwrap()).collect();

  let mut file_list = get_final_file_list(infiles, in_dir && action != "recomp", &action);
  file_list.sort();
  
  let mut extra_file_list = get_final_file_list(vec![ &text_script_dir ], true, &action);
  
  extra_file_list.sort();
  
  use rayon::prelude::*;

  let quirks_str = matches.get_one::<String>("quirks").map(|it| it.as_str()).unwrap_or_default();
  let mut quirks = Quirks::empty();
  
  if quirks_str.contains("xbox") {
    quirks = quirks.union(Quirks::XBox);
  }
  if quirks_str.contains("xbox-root2") {
    quirks = quirks.union(Quirks::XBoxRoot);
  }
  if quirks_str.contains("psp") {
    quirks = quirks.union(Quirks::PSP);
  }
  if quirks.is_empty() {
    quirks = quirks.union(Quirks::CCFC);
  }

  match action.as_str() {
    "encode" => {
      file_list.par_iter().for_each(|file| {
        do_reencode_command(&outfile, &file);
      });
    }
    "decomp" => {
      let input_file = file_list.first().expect(&help_text.ansi().to_string());

      let data = std::fs::read(input_file).unwrap();

      do_extract_command(data, &outfile, quirks);
    }
    "decode" => {
      file_list.par_iter().for_each(|file| {
        do_decode_command(&outfile, &file, quirks);
      });
    }
    "unpack" => {
      let input_file = file_list.first().expect(&help_text.ansi().to_string());

      let data = std::fs::read(input_file).unwrap();

      do_unpack_command(data, &outfile, &text_script_dir, quirks);
      
    }
    "pack" => {
      let top_dir = file_list.first().expect(&help_text.ansi().to_string());
      do_archive_command(top_dir, &text_script_dir, &outfile, true, true);
      
    }
    "transform" => {
      std::fs::create_dir_all(&outfile).unwrap();
      for input_script in file_list {
        let data = serde_yml::from_str(&std::fs::read_to_string(&input_script).unwrap());
        if let Ok(data) = data {
          let output = tl_transform_script(&data);
          std::fs::write(outfile.join(input_script.file_stem().unwrap()).with_extension("txt"), output).unwrap();
        }
      }
    }
    "untransform" => {
      for (input_script, input_text) in file_list.iter().zip(extra_file_list.iter()) {
        let mut data: Script = serde_yml::from_str(&std::fs::read_to_string(&input_script).unwrap()).unwrap();
        let text_str = std::fs::read_to_string(input_text).unwrap();
        tl_reverse_transform_script(&mut data, &text_str);
        std::fs::write(&input_script, script2yaml(&data)).unwrap();
      }
    }
    "mk_uncompbin" => {
      let top_dir = file_list.first().expect(&help_text.ansi().to_string());
      do_archive_command(top_dir, &text_script_dir, &outfile, false, false);
    }
    "recomp" => {
      let top_dir = file_list.first().expect(&help_text.ansi().to_string());
      do_archive_command(top_dir, &text_script_dir, &outfile, true, false);
    }
    "fix" => {
      let input_file = file_list.first().expect(&help_text.ansi().to_string());

      do_fix_command(input_file, outfile);
    }
    _ => panic!("{help_text}"),
  }
}
