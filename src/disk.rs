use std::fs;
use std::path::Path;

pub fn create_dump(dump_dir: &Path) {
    if dump_dir.exists() {
        fs::remove_dir_all(&dump_dir).expect("Failed to delete directories");
    }
    fs::create_dir_all(dump_dir.join("frames")).expect("Failed to create dump directory");
}
