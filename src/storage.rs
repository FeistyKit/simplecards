// Reading vocab and configuration from storage
// TODO: implement serde reading from storage
// TODO: implement saving to storage
// TODO: implement configuration

use crate::vocab::VocabSet;

// Construct a set from specific rules
pub fn construct_set<'a>(source_files: Vec<String>, config_file: Option<String>) -> Result<VocabSet<'a>, Box<dyn std::error::Error>> {
    let mut dirs = directories::ProjectDirs::from("", "FeistyKit", "simplecards").ok_or()?;
    let mut unsorted_items = Vec::new();
    // foo
    // bar/baz
    // jim


    // The base path for the files
    let base_data = dirs.data_dir().to_owned();

    for init_path in source_files {
        let path = base_data.push(init_path);
        let reader = std::io::BufReader::new(std::file::File::open(path)?);
        let items = serde_json::from_reader(reader)?;
        unsorted_items.append(items);
    }

    // TODO: use config_file
    Ok(VocabSet::from_unparsed(unsorted_items, None))
}
