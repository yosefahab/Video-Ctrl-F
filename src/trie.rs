use std::collections::HashMap;

pub struct Trie {
    children: HashMap<char, Box<Trie>>,
    timestamps: Option<Vec<u64>>,
}

impl Trie {
    pub fn new() -> Self {
        return Trie {
            children: HashMap::new(),
            timestamps: None,
        };
    }
    /// returns: an ok if insertion was successful or Error
    pub fn insert(&mut self, word: &str, timestamp: u64) {
        let mut current_node = self;
        for c in word.chars() {
            current_node = current_node
                .children
                .entry(c)
                .or_insert(Box::new(Trie::new()));
        }
        current_node
            .timestamps
            .get_or_insert(Vec::new())
            .push(timestamp);
    }
    /// returns: None if word does not exist, list of associated timestamps if it does
    pub fn get_timestamps(&self, word: &str) -> Option<&Vec<u64>> {
        let mut current_node = self;
        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(child) => {
                    current_node = child;
                }
                None => return None,
            }
        }
        return current_node.timestamps.as_ref();
    }
}
