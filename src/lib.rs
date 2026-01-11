use crate::opcodescript::Quirks;
use camino::Utf8PathBuf;

pub mod commands;
pub mod logging;
pub mod lz77;
pub mod opcodescript;
pub mod scenario_pack;
pub mod util;

pub fn main_preamble(typ: &str) -> (Vec<Utf8PathBuf>, String) {
	use walkdir;
	logging::init().unwrap();

	let mut args = std::env::args().skip(1).collect::<Vec<_>>();

	let mut quirks_string = String::new();
	if let Some(idx) = args.iter().position(|it| it == "-q") {
		if idx == args.len() - 1 {
			log::error!(
	  	    		"No quirks specified! The following quirks modes are available.\n{}\nSome will cause incorrect decompilation if used incorrectly, so be careful.",
	  	    		Quirks::names().join(", ")
	  	    );
			std::process::exit(1);
		}
		quirks_string = args[idx + 1].to_string();
		args.remove(idx + 1);
		args.remove(idx);
	} else if let Some(idx) = args.iter().position(|it| it.starts_with("-q=")) {
		quirks_string = args[idx].trim_start_matches("-q=").trim_matches('"').into();
		args.remove(idx);
	}

	let mut out_files: Vec<Utf8PathBuf> = vec![];
	for arg in args {
		let walker = walkdir::WalkDir::new(&arg);
		for entry in walker.contents_first(true).into_iter() {
			let entry = entry.unwrap();
			log::trace!("walking {}", entry.path().display());
			let file_name = entry.file_name().to_string_lossy();
			let c1 = entry.file_type().is_file();
			let c2 = str::is_empty(typ);
			let c3 = util::ends_with_ignore_case(&file_name, &typ);
			if c1 && (c2 || c3) {
				out_files.push(Utf8PathBuf::from_path_buf(entry.into_path()).unwrap());
			}
		}
	}

	(out_files, quirks_string)
}

pub fn parse_quirks_arg(quirks_arg: &str) -> Quirks {
	let quirks_list = quirks_arg.split(",").collect::<Vec<_>>();

	let mut quirks = Quirks::empty();

	if quirks_list.contains(&"xbox") {
		quirks = quirks.union(Quirks::XBox);
	}
	if quirks_list.contains(&"xbox-root2") {
		quirks = quirks.union(Quirks::XBoxRoot);
	}
	if quirks_list.contains(&"psp") {
		quirks = quirks.union(Quirks::PSP);
	}
	if quirks_list.contains(&"phantom") {
		quirks = quirks.union(Quirks::Phantom);
	}
	if quirks_list.contains(&"sg2") {
		quirks = quirks.union(Quirks::SG2);
	}
	if quirks_list.contains(&"sg") {
		quirks = quirks.union(Quirks::SG);
	}
	if quirks_list.contains(&"lp") || quirks_list.contains(&"library-party") {
		quirks = quirks.union(Quirks::LibraryParty);
	}
	if quirks.is_empty() {
		quirks = quirks.union(Quirks::CCFC);
	}

	quirks
}
