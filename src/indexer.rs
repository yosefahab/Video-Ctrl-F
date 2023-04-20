use crate::trie::Trie;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[allow(unused)]
pub struct Indexer {
    dict: HashMap<String, Vec<u64>>,
    pub trie: Trie,
}

impl Indexer {
    pub fn new() -> Self {
        return Self {
            dict: HashMap::new(),
            trie: Trie::new(),
        };
    }
    pub fn update(&mut self, text: &str, timestamp: u64) {
        for word in self.tokenize(text) {
            self.trie.insert(&word, timestamp).unwrap();
            // let entry = self.dict.entry(word);
            // entry.or_default().push(timestamp);
        }
    }
    pub fn search(&mut self, text: &str) -> Vec<u64> {
        let mut timestamps = Vec::new();
        for word in self.tokenize(text) {
            if let Some(word_stamps) = self.trie.get_timestamps(&word) {
                if timestamps.is_empty() {
                    timestamps = word_stamps;
                } else {
                    timestamps.retain(|stamp| word_stamps.contains(stamp));
                }
            };
        }
        return timestamps;
    }
    pub fn tokenize(&mut self, text: &str) -> HashSet<String> {
        let re = Regex::new(r"\w+").unwrap();
        let words: HashSet<String> = re
            .find_iter(text.trim().to_lowercase().as_str())
            .map(|m| m.as_str().to_owned())
            .collect();
        return words;
    }
}
