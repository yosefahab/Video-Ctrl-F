#[cfg(test)]
mod tests {
    use crate::gec;
    use crate::ocr;
    use crate::trie::Trie;
    use crate::vidsplicer;
    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        let entries = [
            "Trie", "Tree", "words", "work", "test", "ocr", "asr", "midnight",
        ];
        entries
            .iter()
            .enumerate()
            .for_each(|(x, i)| trie.insert(i, x as u64).unwrap());

        entries
            .iter()
            .enumerate()
            .for_each(|(x, i)| assert_eq!(trie.get_timestamps(i).unwrap(), [x as u64]));
    }

    #[test]
    fn test_gec() {
        let text = "i am happi";
        let corrected_text = "i am happy";
        assert_eq!(gec::correct(text), corrected_text);
    }

    #[test]
    pub fn test_ocr_from_disk() {
        let res = ocr::ocr_from_disk("data/test.png", &mut ocr::get_api());
        let expected = r#" Pure Text"#;
        assert_eq!(expected, res);
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
