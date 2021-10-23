// Reading vocab and configuration from storage
// TODO: implement serde reading from storage
// TODO: implement saving to storage
// TODO: implement configuration

use crate::vocab::{VocabEntry, VocabSet};

// Construct a set from specific rules
pub fn construct_set(
    source_files: Vec<String>,
    config_file: Option<String>,
) -> Result<VocabSet, Box<dyn std::error::Error>> {
    let mut dirs = directories::ProjectDirs::from("", "FeistyKit", "simplecards")
        .ok_or("Could not open project directories!")?;
    let mut unsorted_items: Vec<VocabEntry> = Vec::new();

    // The base path for the files
    let base_data = dirs.data_dir().to_owned();

    for init_path in source_files {
        let mut other_path = base_data.clone();
        other_path.push(init_path);
        let string = std::fs::read_to_string(other_path)?;
        let mut items = serde_json::from_str(&string)?;
        unsorted_items.append(&mut items);
    }

    // TODO: use config_file
    Ok(VocabSet::from_unparsed(unsorted_items, None))
}
