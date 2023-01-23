use std::fs;
use std::path;

use crate::{model, util};

pub fn find_music_files(scan_path: &path::PathBuf) -> Result<Vec<path::PathBuf>, model::Error> {
    let mut metadata_map = Vec::new();

    for child_entry in fs::read_dir(scan_path)? {
        let child_entry = child_entry?;
        let child_path = child_entry.path();
        if child_entry.file_type()?.is_dir() {
            metadata_map.append(&mut find_music_files(&child_path)?);
        }
        if child_entry.file_type()?.is_file() {
            let maybe_extension = util::get_maybe_extension_string(&child_path);

            match maybe_extension {
                Some(extension) => match extension.as_str() {
                    "flac" => metadata_map.push(child_path),
                    "mp3" => metadata_map.push(child_path),
                    _ => println!(
                        "DEBUG: Skipping file with unknown extension: {}",
                        child_path.to_string_lossy()
                    ),
                },
                None => println!(
                    "DEBUG: skipping file with no extension: {}",
                    child_path.to_string_lossy()
                ),
            }
        }
    }

    Ok(metadata_map)
}
