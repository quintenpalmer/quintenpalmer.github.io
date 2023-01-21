use std::path;

use crate::model;

pub fn find_music_files<P: AsRef<path::Path>>(
    scan_path: P,
) -> Result<Vec<path::PathBuf>, model::Error> {
    todo!("teach me to scan")
}
