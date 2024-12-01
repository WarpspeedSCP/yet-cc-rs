use crate::{util::*, Script};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct DirEntry {
  pub name: String,
  #[serde(skip_serializing)]
  #[serde(default)]
  pub offset: usize,
  #[serde(skip_serializing)]
  #[serde(default)]
  pub size: usize,
}

pub fn parse_scenario(input: &[u8]) -> BTreeMap<DirEntry, Script> {
  let max_offset = transmute_to_u32(0, input) as usize;
  let mut offset = 0;
  let mut entry_id = 0;

  let mut scripts = BTreeMap::new();

  while offset < max_offset {
    let entry_offset = transmute_to_u32(offset, input) as usize;
    let entry_size = transmute_to_u32(offset + 4, input) as usize;
    log::debug!("Parsing script {entry_id:04}");
    let (script, error) = Script::new(&input[entry_offset..entry_offset + entry_size]);
    
    if let Some(error) = error {
      if entry_id == 352 {
        log::info!("Script 352 didn't parse correctly; this is expected.");
      } else {
        log::error!("Encountered an error ({error}) while decoding entry {entry_id:04}.yaml of size 0x{entry_size:08X}", );
      }
    }
    scripts.insert(
      DirEntry {
        name: format!("{entry_id:04}.yaml"),
        offset: entry_offset,
        size: entry_size,
      },
      script,
    );
    offset += 16;
    entry_id += 1;
  }

  log::info!("Parsed {entry_id} scripts in scenario.");

  scripts
}
