use std::fs;
use std::path;

use crate::{model, util};

pub fn find_audio_files(scan_path: &path::PathBuf) -> Result<Vec<path::PathBuf>, model::Error> {
    let mut audio_files = Vec::new();

    for child_entry in fs::read_dir(scan_path)? {
        let child_entry = child_entry?;
        let child_path = child_entry.path();

        if child_entry.file_type()?.is_dir() {
            audio_files.append(&mut find_audio_files(&child_path)?);
        }
        if child_entry.file_type()?.is_file() {
            let maybe_extension = util::get_maybe_extension_string(&child_path);

            match maybe_extension {
                Some(extension) => match extension.as_str() {
                    "flac" => audio_files.push(child_path),
                    "mp3" => audio_files.push(child_path),
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

    Ok(audio_files)
}
