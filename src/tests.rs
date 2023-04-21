#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::path::Path;

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
    #[test]
    fn tokenizer() {
        let tokens = Indexer::new().tokenize("it's a wonderful new world");
        assert!(tokens.contains("new"));
        assert!(tokens.contains("it"));
        assert!(tokens.contains("world"));
        assert!(tokens.contains("a"));
        assert!(tokens.contains("wonderful"));
        assert_eq!(tokens.contains("it's"), false);
    }

    #[test]
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
    fn lep_ocr() {
        let res = ocr::ocr("data/test.png", &mut ocr::get_api()).to_lowercase();
        let res = res.split_whitespace().collect::<HashSet<&str>>();

        let expected = r#"Pure Text"#.to_lowercase();
        let expected = expected.split_whitespace().collect::<HashSet<&str>>();
        assert_eq!(expected, res);
    }
    #[test]
    fn tess_ocr() {
        let res = ocr::tess_ocr("data/test.png", ocr::get_tess_api()).to_lowercase();
        let res = res.split_whitespace().collect::<HashSet<&str>>();

        let expected = r#"Pure Text"#.to_lowercase();
        let expected = expected.split_whitespace().collect::<HashSet<&str>>();
        assert_eq!(expected, res);
    }
    #[test]
    fn rt_ocr() {
        let res = ocr::rt_ocr("data/test.png").to_lowercase();
        let res = res.split_whitespace().collect::<HashSet<&str>>();

        let expected = r#"Pure Text"#.to_lowercase();
        let expected = expected.split_whitespace().collect::<HashSet<&str>>();
        assert_eq!(expected, res);
    }
    #[test]
    fn metadata() {
        let video_path = Path::new("data/patterns.mp4");
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

    #[test]
    fn get_timestamp() {
        let filename = "000513.jpg";
        let fps = 1;
        let num_part = filename.trim_end_matches(".jpg");
        let mut frame_number: u64 = num_part.parse::<u64>().expect("Not a valid u64");
        frame_number /= fps;
        assert_eq!(frame_number, 513);
    }
}
