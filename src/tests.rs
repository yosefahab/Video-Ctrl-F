#![allow(unused)]
use crate::indexer::tokenizer;
use crate::ocr;
use crate::trie::Trie;
use crate::{disk, gec};

use crate::vidsplicer::ffmpeg_utils;
use std::collections::HashSet;
const TEST_IMG: &str = "data/test.png";

#[test]
fn test_validation() {
    let word = "test123";
    for c in word.chars() {
        print!("{c}");
        print!("{}, {}", c.is_ascii_lowercase(), c.is_alphabetic());
    }
}
#[test]
fn test_trie() {
    let mut trie = Trie::new();
    let word1 = "trie";
    let word2 = "tree";

    let word3 = "ever wondered";
    let word4 = "how large scale enterprises";

    for word in tokenizer::get_words(word1) {
        trie.insert(&word, 1);
    }
    for word in tokenizer::get_words(word3) {
        trie.insert(&word, 2);
    }
    for word in tokenizer::get_words(word4) {
        trie.insert(&word, 3);
    }

    assert_eq!(trie.exists(word1).unwrap(), true);
    assert_eq!(trie.exists(word2).unwrap(), false);

    assert_eq!(trie.get_timestamps("ever").unwrap(), [2]);
    assert_eq!(trie.get_timestamps("wondered").unwrap(), [2]);

    assert_eq!(trie.get_timestamps("how").unwrap(), [3]);
    assert_eq!(trie.get_timestamps("large").unwrap(), [3]);
    assert_eq!(trie.get_timestamps("scale").unwrap(), [3]);
}
#[test]
#[should_panic]
fn test_trie_panic() {
    let mut trie = Trie::new();
    trie.insert("a122", 0).unwrap();
}

// #[test]
fn test_gec() {
    let text = "She likes playing in the park and come here every week.";
    let corrected_text = "She likes playing in the park and comes here every week.";
    assert_eq!(gec::correct(text), corrected_text);
}
// #[test]
fn test_save() {
    let img = disk::load_img("../test.png").unwrap();
    disk::save_as_jpg(img, 1, "dump");
}
// #[test]
fn test_get_words() {
    let sentence = "The Quick Brown Fox".to_string();
    let expected: HashSet<String> = HashSet::from_iter(
        [
            "the".to_string(),
            "quick".to_string(),
            "brown".to_string(),
            "fox".to_string(),
        ]
        .iter()
        .cloned(),
    );
    assert_eq!(expected, tokenizer::get_words(&sentence));
}
// #[test]
pub fn test_ocr_from_disk() {
    let mut api = ocr::get_api();
    let res = ocr::ocr_from_disk(TEST_IMG, &mut api);
    let expected = "Don't abbreviate names \
        Don't put types in variable names \
        Add units to variables unless the type tells you \
        Don't put types in your types (e.g. Abstract, BaseX) \
        Refactor if you find yourself naming code \"Utils\"";
    println!("{:?}", res);
    assert_eq!(expected, res);
}

// #[test]
// fn test_operations() {
//     let img = disk::load_img(TEST_IMG).unwrap();
//     let img2 = operations::blur_img(img.clone());
//     disk::save_as_jpg(img2, 2);
//     let img3 = operations::gray_img(img.clone());
//     disk::save_as_jpg(img3, 3);
//     let img4 = img.filter3x3(&[0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]);
//     disk::save_as_jpg(img4, 4);
// }

// #[test]
// fn test_lep_ocr_from_mem() {
//     let mut api = ocr::get_api();
//     let img = disk::load_img(TEST_IMG).unwrap();
//     let res = ocr::ocr_from_mem(&img, &mut api);
//     println!("{:?}", res);
// }
