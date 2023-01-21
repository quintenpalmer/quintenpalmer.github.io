use std::path;

use crate::model;

pub fn parse_all_music_files(
    paths: Vec<path::PathBuf>,
) -> Result<Vec<model::AudioFileTrackMetadata>, model::Error> {
    todo!("teach me to parse all files")
}

pub fn parse_single_music_file(
    path: path::PathBuf,
) -> Result<model::AudioFileTrackMetadata, model::Error> {
    todo!("teach me to parse a single file")
}
