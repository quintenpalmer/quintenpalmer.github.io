use std::path;

use crate::{model, organize, parse, scan};

impl model::Library {
    pub fn from_library_directory<P: AsRef<path::Path>>(
        library_directory: P,
    ) -> Result<Self, model::Error> {
        let audio_file_paths = scan::find_music_files(&library_directory.as_ref().to_path_buf())?;

        let audio_file_track_metadata_entries = parse::parse_all_music_files(audio_file_paths)?;

        let library = organize::organize_tracks(audio_file_track_metadata_entries)?;

        Ok(library)
    }
}
