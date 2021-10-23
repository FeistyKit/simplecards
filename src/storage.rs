// Reading vocab and configuration from storage
// TODO: implement saving to storage
// TODO: implement configuration

use crate::vocab::{VocabEntry, VocabRules, VocabSet};

//I'm too lazy to type it out that many times
pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

// Construct a set from specific rules
pub fn construct_set(
    source_files: Vec<String>,
    config_file: Option<String>, config_default: bool
) -> Result<VocabSet, Box<dyn std::error::Error>> {
    let unsorted_items = make_item_list(source_files)?;



    Ok(VocabSet::from_unparsed(unsorted_items, None))
}

// Make the list of VocabEntries by reading from the files
fn make_item_list(source_files: Vec<String>) -> AnyResult<Vec<VocabEntry>> {
    let dirs = directories::ProjectDirs::from("", "FeistyKit", "simplecards")
        .ok_or("Could not open project directories!")?;
    let mut unsorted_items: Vec<VocabEntry> = Vec::new();

    // The base path for the files
    let base_data = dirs.data_dir().to_owned();

    for init_path in source_files {
        // find the total path of the file
        let mut other_path = base_data.clone();
        other_path.push(init_path);

        // extract the vocab entries from the file
        let string = std::fs::read_to_string(other_path)?;
        let mut items = serde_json::from_str(&string)?;

        // add them to the list
        unsorted_items.append(&mut items);
    }

    Ok(unsorted_items)
}

// Get *a* config, although not necesarily the one that the user wanted.
fn make_config(config_file: Option<String>, config_default: bool) -> VocabRules {
    if config_default {
        return Default::default();
    }

    try_make_config(config_file).unwrap_or_default()

}

// Try to get the config from the options
fn try_make_config(config_file: Option<String>) -> AnyResult<VocabRules> {
    match config_file {
        Some(raw_path) => {
            let string = std::fs::read_to_string(raw_path)?;
            Ok(serde_yaml::from_str(&string)?)
        },
        None => {
            // finding the path of the config file
            let dirs = directories::ProjectDirs::from("", "FeistyKit", "simplecards")
                .ok_or("Could not open project directories!")?;
            let mut config_path = dirs.config_dir().to_owned();
            config_path.push("config.yml");

            // Getting the item itself
            let string = std::fs::read_to_string(config_path)?;
            // TODO: fix this
            Ok(serde_yaml::from_str(&string)?)
        }
    }
}
