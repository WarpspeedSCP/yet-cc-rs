use crate::{
	opcodescript::{Quirks, Script},
	util::*,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct DirEntry<'a> {
	pub name: String,
	#[serde(skip_serializing)]
	#[serde(default)]
	pub offset: usize,
	#[serde(skip_serializing)]
	#[serde(default)]
	pub size: usize,
	#[serde(skip_serializing)]
	#[serde(default)]
	pub data: Option<&'a [u8]>,
}

pub fn parse_scenario<'a>(input: &'a [u8]) -> Vec<DirEntry<'a>> {
	let max_offset =
		transmute_to_u32(0, input).expect("sn.bin provided is less than 4 bytes long!") as usize;
	let mut offset = 0;
	let mut entry_id = 0;

	let mut direntries = Vec::new();

	while offset < max_offset {
		let entry_offset = transmute_to_u32(offset, input).expect(&format!(
			"Could not process entry offset for entry 0x{entry_id:04X} at offset 0x{offset:08X}"
		)) as usize;
		let entry_size = transmute_to_u32(offset + 4, input).expect(&format!(
			"Could not process entry size for entry 0x{entry_id:04X} at offset 0x{:08X}",
			offset + 4
		)) as usize;
		let entry = DirEntry {
			name: format!("{entry_id:04}.yaml"),
			offset: entry_offset,
			size: entry_size,
			data: Some(&input[entry_offset..entry_offset + entry_size]),
		};

		log::info!(
			"Directory entry {} of size 0x{:08X} at {:08X}",
			entry.name,
			entry.offset,
			entry.size
		);

		direntries.push(entry);
		offset += 16;
		entry_id += 1;
	}

	direntries
}

pub fn parse_script(entry: &DirEntry, quirks: Quirks) -> anyhow::Result<Script> {
	log::debug!("Parsing script {}.", entry.name);

	let data = entry.data.unwrap();

	let (script, error) = Script::new(data, quirks)?;

	if let Some(error) = error {
		let script_id = entry
			.name
			.split('.')
			.next()
			.unwrap()
			.parse::<u32>()
			.unwrap_or(u32::MAX);
		let is_expected = match script_id {
			1 | 382 => quirks.contains(Quirks::LibraryParty),
			352 => true,
			_ => false,
		};

		if is_expected {
			log::info!(
				"Script {} didn't parse correctly; this is expected.",
				entry.name.split('.').next().unwrap()
			);
		} else {
			log::error!(
				"Encountered an error while decoding entry {} of size 0x{:08X}: {}",
				entry.name.split('.').next().unwrap(),
				entry.size,
				error
			);
		}
	}
	Ok(script)
}
