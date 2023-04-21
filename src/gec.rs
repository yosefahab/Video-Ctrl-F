use nlprule::{Rules, Tokenizer};
pub fn correct(text: &str) -> String {
    let mut tokenizer_bytes: &'static [u8] = include_bytes!("../models/en_tokenizer.bin");
    let mut rules_bytes: &'static [u8] = include_bytes!("../models/en_rules.bin");

    let tokenizer =
        Tokenizer::from_reader(&mut tokenizer_bytes).expect("tokenizer binary is invalid");
    let rules = Rules::from_reader(&mut rules_bytes).expect("rules binary is invalid");

    return rules.correct(text, &tokenizer);
}
