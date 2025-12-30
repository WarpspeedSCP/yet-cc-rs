use camino::Utf8PathBuf;
use yeti::{main_preamble};
use yeti::commands::{do_archive_command};
use yeti::opcodescript::Quirks;
use yeti::util::{current_dir};

fn main() {
	if std::env::args().any(|it| it == "-h" || it == "--help") {
		println!(r#"Yeti scenario repacker. Usage: yeti_pack <sn.bin.script directory>.
This tool will put the output sn.bin file in the direcory where it is run from. 
"#);
		std::process::exit(0);
	}

	let (files, _) = main_preamble(&"txt");

	let files = if files.is_empty() {
		let args = std::env::args().collect::<Vec<_>>();
		args.iter().skip(1).map(|it| Utf8PathBuf::from(it).join("nonexistant")).collect::<Vec<_>>()
	} else {
		files
	};

	let current_folder = files
		.first()
		.expect("Expected the folder to contain files!")
		.parent()
		.unwrap();

	let yaml_folder = current_folder
		.parent().unwrap()
		.join(current_folder.file_name().expect("Expected folder to have a file name!").replace(".script", ".yaml"));

	do_archive_command(&yaml_folder, current_folder, &current_dir().join("sn.bin"), true, true)
}
