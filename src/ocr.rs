#![allow(unused)]

use leptess::LepTess;
use std::collections::HashSet;
use std::io::Cursor;
use std::sync::MutexGuard;

pub fn get_api() -> LepTess {
    let mut api = leptess::LepTess::new(
        Some("../Video-Ctrl-F/models/traineddata/tessdata_fast"),
        "eng",
    )
    .unwrap();
    api.set_variable(leptess::Variable::TesseditPagesegMode, "3")
        .expect("Failed to set tesseract variable");
    api.set_variable(leptess::Variable::TesseditOcrEngineMode, "2")
        .expect("Failed to set tesseract variable");
    api.set_variable(leptess::Variable::UserDefinedDpi, "100")
        .expect("Failed to set tesseract variable");
    return api;
}

pub fn ocr_from_disk(path: &str, api: &mut LepTess) -> String {
    api.set_image(path).unwrap();
    return api.get_utf8_text().expect("OCR failed");
}

// pub fn ocr_from_mem(image: &DynamicImage, api: &mut LepTess) -> HashSet<String> {
//     let mut jpg_buffer = Vec::new();
//     image
//         .write_to(
//             &mut Cursor::new(&mut jpg_buffer),
//             image::ImageOutputFormat::Jpeg(80),
//         )
//         .expect("failed to convert buffer to jpeg");
//     api.set_image_from_mem(&jpg_buffer);
//     let text = api.get_utf8_text().unwrap();
//     return tokenizer::get_words(text);
// }

pub fn threaded_ocr_from_disk(path: &str, mut api: MutexGuard<LepTess>) -> String {
    api.set_image(path).unwrap();
    let text = api.get_utf8_text().unwrap();
    return text;
}

// enum OcrEngineMode {
//     OEM_TESSERACT_ONLY = 0
//     OEM_LSTM_ONLY = 1
//     OEM_TESSERACT_LSTM_COMBINED = 2
//     OEM_DEFAULT = 3
//     OEM_COUNT = 4
// }
// enum PageSegMode {
//     PSM_OSD_ONLY = 0,
//     PSM_AUTO_OSD = 1,
//     PSM_AUTO_ONLY = 2,
//     PSM_AUTO = 3,
//     PSM_SINGLE_COLUMN = 4,
//     PSM_SINGLE_BLOCK_VERT_TEXT = 5,
//     PSM_SINGLE_BLOCK = 6,
//     PSM_SINGLE_LINE = 7,
//     PSM_SINGLE_WORD = 8,
//     PSM_CIRCLE_WORD = 9,
//     PSM_SINGLE_CHAR = 10,
//     PSM_SPARSE_TEXT,
//     PSM_SPARSE_TEXT_OSD = 12,
//     PSM_RAW_LINE = 13,
//     PSM_COUNT,
// }
