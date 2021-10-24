// Reading vocab and configuration from storage
// TODO: implement saving to storage
// TODO: implement configuration

use crate::vocab::{VocabEntry, VocabRules, VocabSet};
use std::io::Write;

//I'm too lazy to type it out that many times
pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

// The directories for saving/reading info
fn data_dir() -> AnyResult<std::path::PathBuf> {
    Ok(
        directories::ProjectDirs::from("", "FeistyKit", "simplecards")
            .ok_or("Could not open project data directory!")?
            .data_dir()
            .to_owned(),
    )
}
fn config_dir() -> AnyResult<std::path::PathBuf> {
    Ok(
        directories::ProjectDirs::from("", "FeistyKit", "simplecards")
            .ok_or("Could not open project config directory!")?
            .config_dir()
            .to_owned(),
    )
}

// Construct a set from specific rules
pub fn construct_set(
    source_files: Vec<String>,
    config_file: Option<String>,
    config_default: bool,
) -> AnyResult<VocabSet> {
    let unsorted_items = make_item_list(source_files)?;
    let config = make_config(config_file, config_default);

    Ok(VocabSet::from_unsorted(unsorted_items, config))
}

// Make the list of VocabEntries by reading from the files
fn make_item_list(source_files: Vec<String>) -> AnyResult<Vec<VocabEntry>> {
    let mut unsorted_items: Vec<VocabEntry> = Vec::new();

    for init_path in source_files {
        // find the total path of the file
        let mut total_path = data_dir()?;
        total_path.push(init_path);

        // extract the vocab entries from the file
        let string = std::fs::read_to_string(total_path)?;
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
        }
        None => {
            // finding the path of the config file
            let mut config_path = config_dir()?;
            config_path.push("config.yml");

            // Getting the item itself
            let string = std::fs::read_to_string(config_path)?;
            // I don't know why it needs to be done this way
            Ok(serde_yaml::from_str(&string)?)
        }
    }
}

// save the total set
pub fn save_set(
    rules: VocabRules,
    map: std::collections::HashMap<String, Vec<VocabEntry>>,
) -> AnyResult<()> {
    // don't overwrite the config if the default rules were chosen
    if rules != VocabRules::default() {
        save_config(rules)?;
    }
    save_data(map)?;
    Ok(())
}

fn save_config(config: VocabRules) -> AnyResult<()> {
    // Finding where to save the info
    let mut config_file = config_dir()?;
    config_file.push("config");

    // Preparing the info to be saved
    let string = serde_yaml::to_string(&config)?;

    // Writing the data
    let mut f = std::fs::File::create(config_file)?;
    f.write(string.as_bytes())?;
    Ok(())
}

fn save_data(data: std::collections::HashMap<String, Vec<VocabEntry>>) -> AnyResult<()> {
    for (rel_path, to_save) in data.into_iter() {
        // Finding where to save the info
        let mut path = data_dir()?;
        path.push(rel_path);

        // Preparing the info to be saved
        let string = serde_json::to_string(&to_save)?;

        // Writing the data
        let mut f = std::fs::File::create(path)?;
        f.write(string.as_bytes());
    }
    Ok(())
}
