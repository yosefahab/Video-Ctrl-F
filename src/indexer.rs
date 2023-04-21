// use crate::trie::Trie;
use regex::Regex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
// use stop_words::{get, LANGUAGE};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Indexer {
    index: HashMap<String, Vec<u64>>,
    // trie: Trie,
    // stop_words: Vec<String>,
}

impl Indexer {
    pub fn new() -> Self {
        return Self {
            index: HashMap::new(),
            // trie: Trie::new(),
            // stop_words: get(LANGUAGE::English),
        };
    }
    pub fn update(&mut self, text: &str, timestamp: u64) {
        for word in self.tokenize(text) {
            // self.trie.insert(&word, timestamp);
            let entry = self.index.entry(word);
            entry.or_default().push(timestamp);
        }
    }
    pub fn search(&mut self, text: &str) -> Vec<u64> {
        let mut timestamps = Vec::new();
        for word in self.tokenize(text) {
            // if let Some(word_stamps) = self.trie.get_timestamps(&word) {
            //     if timestamps.is_empty() {
            //         timestamps = word_stamps;
            //     } else {
            //         timestamps.retain(|stamp| word_stamps.contains(stamp));
            //     }
            // };
            if let Some(word_stamps) = self.index.get(&word) {
                if timestamps.is_empty() {
                    timestamps = word_stamps.clone();
                } else {
                    timestamps.retain(|stamp| word_stamps.contains(stamp));
                }
            }
        }
        return timestamps;
    }
    pub fn tokenize(&mut self, text: &str) -> HashSet<String> {
        let re = Regex::new(r"\w+").unwrap();
        let words: HashSet<String> = re
            .find_iter(&text.trim().to_lowercase())
            .map(|m| m.as_str().to_string())
            // .filter(|w| self.stop_words.contains(w) == false)
            .collect();
        return words;
    }

    pub fn serialize(&self) -> Value {
        return serde_json::to_value(&self.index).unwrap();
    }
}
