use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
  lz77,
  scenario_pack::{parse_scenario, DirEntry},
  util::fix_line,
  Script,
};
use std::{
  io::Read,
  path::{Path, PathBuf},
};

pub fn do_archive_command(top_dir: &PathBuf, outfile: PathBuf, compress: bool) {
  let directory: Vec<DirEntry> =
    serde_yml::from_str(&std::fs::read_to_string(&top_dir.join("directory.yaml")).unwrap())
      .unwrap();

  let mut scripts: Vec<(String, Script)> = vec![];
  for DirEntry { name, .. } in directory {
    let path = top_dir.join(name);
    log::debug!("Opening {}", path.display());
    scripts.push((
      path.display().to_string(),
      serde_yml::from_slice(&std::fs::read(path).unwrap()).unwrap(),
    ))
  }

  let n_scripts = scripts.len();

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
pub fn do_extract_command(data: Vec<u8>, outfile: &PathBuf) {
  let decompressed_data = lz77::lz77_decompress(&data);

  let scripts = parse_scenario(&decompressed_data);

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
        serde_yml::to_string(&script)
          .unwrap()
          .replace("'[", "[")
          .replace("]'", "]")
          .replace(r#"'""#, "")
          .replace(r#""'"#, ""),
      );
      if let Err(e) = res {
        log::error!("Encountered an error when writing {name}: {}", e);
      }
    });
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

pub fn do_decode_command(outfile: &Path, filename: &Path) {
  let script = decode_opcodescript(filename);
  let outfile = if !outfile.ends_with(".yaml") {
    if !outfile.exists() {
      std::fs::create_dir_all(outfile)
        .expect(&format!("Could not create directory {}", outfile.display()));
    }
    outfile.join(filename.with_extension("yaml").file_name().unwrap())
  } else {
    outfile.to_owned()
  };
  log::info!("writing output to {}", outfile.display());
  std::fs::write(outfile, serde_yml::to_string(&script).unwrap()).unwrap();
}

fn decode_opcodescript(filename: &Path) -> Script {
  log::info!("decoding file {}", filename.display());
  let data = std::fs::read(filename).unwrap();

  let (script, error) = Script::new(&data);

  if let Some(error) = error {
    log::error!(
      "Encountered an error ({error}) while decoding file {}",
      filename.display()
    );
  }

  script
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
