pub mod lookup {
    use super::tokenizer;

    use crate::trie::Trie;
    use std::collections::{HashMap, HashSet};
    pub static mut INDEX: Option<HashMap<String, Vec<u64>>> = None;

    pub static mut LOOKUP: Option<Trie> = None;

    fn get_index() -> &'static mut HashMap<String, Vec<u64>> {
        unsafe {
            return INDEX.get_or_insert(HashMap::new());
        }
    }

    fn get_lookup() -> &'static mut Trie {
        unsafe {
            return LOOKUP.get_or_insert(Trie::new());
        }
    }

    pub fn update(text: &str, timestamp: u64) {
        for word in tokenizer::get_words(text) {
            get_lookup().insert(&word, timestamp);
            // let entry = get_index().entry(word);
            // entry.or_default().push(timestamp);
        }
    }
    pub fn search(text: &str) -> Vec<u64> {
        let mut timestamps = Vec::new();
        for word in tokenizer::get_words(text) {
            if let Some(mut word_stamps) = get_lookup().get_timestamps(&word) {
                if timestamps.is_empty() {
                    timestamps = word_stamps;
                } else {
                    timestamps.retain(|stamp| word_stamps.contains(stamp));
                }
            };
        }
        return timestamps;
    }
}
pub mod tokenizer {
    use regex::Regex;
    use std::collections::HashSet;

    pub fn get_words(text: &str) -> HashSet<String> {
        let mut set: HashSet<String> = HashSet::new();
        let re = Regex::new(r"\W+").unwrap();
        for word in re.replace_all(&text.to_lowercase(), " ").split(' ') {
            if !word.is_empty() {
                set.insert(word.to_string());
            }
        }
        return set;
    }
}
