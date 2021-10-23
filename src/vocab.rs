// The structs and helpful methods that need to be used
// TODO: implement these

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabEntry<'a>{
    key: String,
    val: String,
    times: usize, // times tried
    correct: usize, // times correct
    state: VocabState,

    // TODO: sorting?
    // #[serde(skip_serializing_if = "Option::is_none")]
    // sort_number: Option<usize>,

    #[serde(skip_serializing)]
    path: &'a str // the path that it was loaded from
}

#[allow (unused)]
impl<'a> VocabEntry<'a> {
    pub fn increment(&mut self, correct: bool) {
        self.times += 1;
        if correct {
            self.correct += 1;
        }
    }
}

// the state that the piece of vocabulary is in;
// it can have been recently done correctly, it can have been recently failed,
// and it can have been untried in the current session or run
#[derive (Debug, Clone, Serialize, Deserialize)]
pub enum VocabState {
    Passed,
    Failed,
    Untried,
}

// The current set of worked vocabulary; Each individual entry will be saved, but the whole set will not be
#[derive (Debug, Clone)]
pub struct VocabSet<'a> {
    passed: Vec<VocabEntry<'a>>,
    failed: Vec<VocabEntry<'a>>,
    untried: Vec<VocabEntry<'a>>,
    rules: VocabRules
}

// Rules for the set that will be loaded from a file or given via command line
#[derive (Debug, Clone, Serialize, Deserialize)]
pub struct VocabRules {
    shuffle: bool,         // Whether the lists should be shuffled on load/completion
    sorted: bool,          // Whether the lists should be sorted; not available right now
    always_validate: bool, // Whether the lists should always be validated
    debug_validate: bool,  // Whether the lists should only be validated on debug
    inc_passing: bool,     // Whether failed items should be marked as succeeded whenever they are gotten right or only when they are all right in a row
    all_failing: bool,     // Whether all items should be marked as failing by default
    immut: bool,           // Whether or not the items should be modified
                           // TODO: find a better name for this
}

impl std::default::Default for VocabRules {
    fn default() -> Self {
        VocabRules {
            shuffle: true,
            sorted: false,
            always_validate: true,
            debug_validate: true,
            inc_passing: false,
            all_failing: false,
            immut: false,
        }
    }
}

impl<'a> VocabSet<'a> {

    // construct the set from unparsed items
    #[allow(unused)]
    pub fn from_unparsed(items: Vec<VocabEntry<'a>>, rules: Option<VocabRules>) -> VocabSet<'a> {
        let mut passed = Vec::new();
        let mut failed = Vec::new();
        let mut untried = Vec::new();
        for item in items {
            match item.state {
                VocabState::Passed => {
                    passed.push(item);
                },
                VocabState::Failed => {
                    failed.push(item);
                },
                VocabState::Untried => {
                    untried.push(item);
                }
            }
        }
        VocabSet {
            passed,
            failed,
            untried,
            rules: rules.unwrap_or_default(),
        }
    }

    // Converts the set to a format that can be saved
    #[allow(unused)]
    pub fn to_saveable(&self) -> (VocabRules, std::collections::HashMap<&'a str, Vec<VocabEntry<'a>>>) {
        let mut map = std::collections::HashMap::new();
        let total_items = self.passed.iter().chain(self.failed.iter()).chain(self.untried.iter());
        for item in total_items {
            map.entry(item.path).or_insert_with(Vec::new).push(item.clone());
        }
        (self.rules.clone(), map)
    }
}
