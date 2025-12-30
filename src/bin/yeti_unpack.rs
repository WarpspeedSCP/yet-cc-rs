use camino::{Utf8Path, Utf8PathBuf};
use yeti::{main_preamble, parse_quirks_arg};
use yeti::opcodescript::Quirks;
use yeti::util::{current_dir, safe_create_dir};

fn main() {

	if std::env::args().any(|it| it == "-h" || it == "--help") {
		println!(r#"Yeti scenario unpacker. Usage: yeti_unpack <sn.bin file> [-q comma,separated,quirks].
This tool will create its output in the current directory under the extracted_scenario subdirectory.
Options:

-q 								Sets quirks to use. The following quirks are available to be used:
									{}.
									Be careful when mixing these, it can cause problems.
									Example: yeti_unpack.exe sn.bin -q psp,ccfc
"#,
						 Quirks::names().join(", "));
		std::process::exit(0);
	}
	
	let (files, quirks) = main_preamble(&"sn.bin");
	let top_out_path = &current_dir().join("extracted_scenario");
	safe_create_dir(&top_out_path).unwrap();

	let quirks = parse_quirks_arg(&quirks);

	for i in files {
		let dirent = i;
		let out_folder_base_name = &top_out_path.join(dirent.file_name().unwrap());
		let out_yaml_folder = &out_folder_base_name.with_extension("bin.yaml");
		let out_script_folder = &out_folder_base_name.with_extension("bin.script");

		safe_create_dir(&out_folder_base_name).unwrap();
		safe_create_dir(&out_yaml_folder).unwrap();
		safe_create_dir(&out_script_folder).unwrap();

		let file_contents = std::fs::read(&dirent).unwrap();

		yeti::commands::do_unpack_command(file_contents, &out_yaml_folder, &out_script_folder, quirks);
	}
}
