const BASE: usize = b'a' as usize;
fn char_to_index(c: char) -> usize {
    return c as usize - BASE;
}
fn is_alphabets_only(word: &str) -> bool {
    for c in word.chars() {
        if !(c.is_ascii_lowercase()) {
            return false;
        }
    }
    return true;
}

// #[derive(Default)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    timestamps: Option<Vec<u64>>,
}
#[derive(Debug)]
pub enum Error {
    NonWord,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NonWord => writeln!(f, "Word isn't valid alphabet (a-zA-Z)"),
        }
    }
}
impl Trie {
    pub fn new() -> Self {
        // return Default::default();
        return Trie {
            children: [(); 26].map(|_| None),
            timestamps: None,
        };
    }
    ///
    /// returns: new trie instance after populating it with the given word and timestamp
    ///
    pub fn from_string(word: &str, timestamp: u64) -> Trie {
        let mut trie = Trie::new();
        trie.insert(word, timestamp);
        return trie;
    }
    ///
    /// returns: an ok if insertion was successful or Error
    ///
    pub fn insert(&mut self, word: &str, timestamp: u64) -> Result<(), Error> {
        if !is_alphabets_only(word) {
            return Err(Error::NonWord);
        }

        let mut current_node = self;
        for c in word.chars().map(char_to_index) {
            current_node = current_node.children[c].get_or_insert(Box::new(Trie::new()));
        }

        current_node
            .timestamps
            .get_or_insert(Vec::new())
            .push(timestamp);

        return Ok(());
    }

    ///
    /// returns: a boolean indicating wether a word exists or not
    ///
    pub fn exists(&self, word: &str) -> Result<bool, Error> {
        if !is_alphabets_only(word) {
            return Err(Error::NonWord);
        }
        let mut current_node = self;
        for c in word.chars().map(char_to_index) {
            match current_node.children[c] {
                Some(ref next_node) => {
                    current_node = next_node;
                }
                None => return Ok(false),
            }
        }
        return Ok(true);
    }

    ///
    /// returns: None if word does not exist, timestamp if it does
    ///
    pub fn get_timestamps(&self, word: &str) -> Option<Vec<u64>> {
        if !is_alphabets_only(word) {
            return None;
        }
        let mut current_node = self;
        for c in word.chars().map(char_to_index) {
            match current_node.children[c] {
                Some(ref next_node) => {
                    current_node = next_node;
                }
                None => return None,
            }
        }
        return current_node.timestamps.clone();
    }
}
