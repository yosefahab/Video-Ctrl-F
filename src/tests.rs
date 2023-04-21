#[cfg(test)]
mod tests {
    use crate::gec;
    use crate::indexer::Indexer;
    use crate::ocr;
    use crate::trie::Trie;
    use crate::vidsplicer;

    #[test]
    fn trie() {
        let mut trie = Trie::new();
        let entries = [
            "Trie", "Tree", "words", "work", "test", "ocr", "asr", "midnight",
        ];
        entries
            .iter()
            .enumerate()
            .for_each(|(x, i)| trie.insert(i, x as u64));
        entries
            .iter()
            .enumerate()
            .for_each(|(x, i)| assert!(trie.get_timestamps(i).unwrap().contains(&(x as u64))));
    }
    #[test]
    pub fn indexr() {
        let mut indexer = Indexer::new();
        let entries = [
            "Trie", "Tree", "words", "work", "test", "ocr", "asr", "midnight",
        ];
        entries
            .iter()
            .enumerate()
            .for_each(|(x, i)| indexer.update(i, x as u64));
        entries
            .iter()
            .enumerate()
            .for_each(|(x, i)| assert!(indexer.search(i).contains(&(x as u64))));
    }

    // #[test]
    fn gec() {
        assert_eq!(
            gec::correct("She was not been here since Monday."),
            String::from("She was not here since Monday.")
        );
        assert_eq!(
            gec::correct("I was go to the mall today."),
            String::from("I was going to the mall today.")
        );
    }

    #[test]
    pub fn tokenizer() {
        let tokens = Indexer::new().tokenize("it's a wonderful new world");
        assert!(tokens.contains("new"));
        assert!(tokens.contains("it"));
        assert!(tokens.contains("world"));
        assert!(tokens.contains("a"));
        assert!(tokens.contains("wonderful"));
        assert_eq!(tokens.contains("it's"), false);
    }
    // #[test]
    pub fn ocr() {
        let res = ocr::ocr("data/test.png");
        let expected = r#"Pure Text"#;
        assert_eq!(
            expected.to_lowercase(),
            res.to_lowercase().replace("\n", "")
        );
    }
    #[test]
    pub fn metadata() {
        let video_path = "data/patterns.mp4";
        match vidsplicer::ffmpeg_utils::get_video_metadata(video_path) {
            vidsplicer::ffmpeg_utils::FFprobeResult::Success(metadata) => {
                assert_eq!(metadata.width, 854);
                assert_eq!(metadata.height, 480);
                assert_eq!(metadata.fps, 24);
                assert_eq!(metadata.duration, 660.0);
            }
            vidsplicer::ffmpeg_utils::FFprobeResult::Failure(_) => (),
        }
    }
}
