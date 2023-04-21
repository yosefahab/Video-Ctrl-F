use std::fs;
use std::path::Path;

use serde_json::{to_string_pretty, Value};

pub fn create_dump(dump_dir: &Path) {
    if dump_dir.exists() {
        fs::remove_dir_all(dump_dir).expect("Failed to delete directories");
    }
    fs::create_dir_all(dump_dir.join("frames")).expect("Failed to create dump directory");
}

pub fn save_as_json(index: Value, path: &Path) -> std::io::Result<()> {
    fs::write(path, to_string_pretty(&index)?)?;
    Ok(())
}
