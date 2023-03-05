use nlprule::{Rules, Tokenizer};
pub fn correct(text: &str) -> String {
    let mut tokenizer_bytes: &'static [u8] = include_bytes!("../en_tokenizer.bin");
    let mut rules_bytes: &'static [u8] = include_bytes!("../en_rules.bin");

    let tokenizer =
        Tokenizer::from_reader(&mut tokenizer_bytes).expect("tokenizer binary is valid");
    let rules = Rules::from_reader(&mut rules_bytes).expect("rules binary is valid");

    return rules.correct(text, &tokenizer);
}