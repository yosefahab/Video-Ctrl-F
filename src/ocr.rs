#![allow(unused)]
use std::collections::HashMap;

use leptess::LepTess;
use rusty_tesseract::{tesseract as rt, Args};
use tesseract::Tesseract;

pub fn ocr(path: &str, api: &mut LepTess) -> String {
    return ocr_from_disk(path, api);
}

pub fn get_api() -> LepTess {
    let mut api = leptess::LepTess::new(Some("models/traineddata/tessdata_best"), "eng").unwrap();
    api.set_variable(leptess::Variable::TesseditPagesegMode, "3")
        .expect("Failed to set Leptess variable");
    api.set_variable(leptess::Variable::TesseditOcrEngineMode, "2")
        .expect("Failed to set Leptess variable");
    api.set_variable(leptess::Variable::UserDefinedDpi, "100")
        .expect("Failed to set Leptess variable");
    return api;
}
pub fn get_tess_api() -> Tesseract {
    let api = Tesseract::new(None, Some("eng"))
        .unwrap()
        .set_variable("tessedit_pageseg_mode", "12")
        .expect("Failed to set Tesseract variable")
        .set_variable("tessedit_ocr_engine_mode", "2")
        .expect("Failed to set Tesseract variable");
    // // .set_variable(leptess::Variable::UserDefinedDpi, "100")
    // // .expect("Failed to set Tesseract variable");
    return api;
}

fn ocr_from_disk(path: &str, api: &mut LepTess) -> String {
    api.set_image(path).unwrap();
    return api.get_utf8_text().expect("OCR failed");
}

pub fn threaded_ocr(path: &str, api: &mut LepTess) -> String {
    api.set_image(path).unwrap();
    return api.get_utf8_text().expect("Threaded OCR failed");
}

pub fn threaded_tess_ocr(path: &str, api: Tesseract) -> String {
    return api
        .set_image(path)
        .unwrap()
        .get_text()
        .expect("Threaded OCR failed");
}

pub fn tess_ocr(path: &str, api: Tesseract) -> String {
    return api.set_image(path).unwrap().get_text().expect("OCR failed");
}

pub fn rt_ocr(path: &str) -> String {
    let image = &rusty_tesseract::Image::from_path(path).unwrap();
    let args = Args {
        lang: "eng".to_string(),
        config_variables: HashMap::from([(
            "TESSDATA_PREFIX".into(),
            "models/traineddata/tessdata_fast/eng.traineddata".into(),
        )]),
        psm: 12,
        oem: 2,
        dpi: 100,
    };
    return rt::image_to_string(image, &args).unwrap();
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
