use image::DynamicImage;

use crate::indexer::lookup;
use std::fs;
use std::path::Path;

pub fn create_dump(filename: &str) {
    let dump_dir = Path::new("dump").join(filename);
    if dump_dir.exists() {
        fs::remove_dir_all(&dump_dir).expect("Failed to delete directories");
    }
    fs::create_dir_all(dump_dir.join("frames")).expect("Failed to create dump directory");
}
pub fn save_as_jpg(image: DynamicImage, index: u64, path: &str) {
    let path = Path::new(path).join(format!("frame{index}.jpg"));
    image.save(path.to_str().unwrap());
}
pub fn save_as_txt(text: &str, index: u64, path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path).join(format!("frame{index}.txt"));
    fs::write(path, text)?;
    return Ok(());
}
pub fn save_as_json(path: &str) -> std::result::Result<(), std::io::Error> {
    let path = Path::new(path).join("index.json");
    let file = fs::File::create(path).expect("Unable to create file");
    let writer = &mut std::io::BufWriter::new(file);
    unsafe {
        serde_json::to_writer_pretty(writer, &lookup::INDEX).unwrap();
    }
    return Ok(());
}
pub fn load_img(path: &str) -> Result<DynamicImage, image::error::ImageError> {
    return image::open(path);
}
