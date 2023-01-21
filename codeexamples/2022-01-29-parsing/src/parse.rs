use std::path;

use crate::{model, util};

pub fn parse_all_music_files(
    paths: Vec<path::PathBuf>,
) -> Result<Vec<model::AudioFileTrackMetadata>, model::Error> {
    let mut all_metadata = Vec::new();

    for music_file_path in paths.into_iter() {
        let single_file_metadata = parse_single_music_file(music_file_path)?;
        all_metadata.push(single_file_metadata);
    }

    Ok(all_metadata)
}

pub fn parse_single_music_file(
    path: path::PathBuf,
) -> Result<model::AudioFileTrackMetadata, model::Error> {
    let maybe_extension = util::get_maybe_extension_string(&path);

    match maybe_extension {
        Some(extension) => match extension.as_str() {
            "flac" => flac::parse_flac_file(path),
            _ => panic!("unknown audio file extension"),
        },
        None => panic!("file without extension"),
    }
}

mod flac {
    use std::collections::BTreeMap;
    use std::path;

    use crate::model;

    pub fn parse_flac_file(
        path: path::PathBuf,
    ) -> Result<model::AudioFileTrackMetadata, model::Error> {
        let reader = claxon::FlacReader::open(&path)?;

        let tag_map = reader
            .tags()
            .map(|(k, v)| (k.to_string().to_lowercase(), v.to_string()))
            .collect::<BTreeMap<String, String>>();

        // Note: "artist" and "title" are the only keys we require, the rest may or may not be set
        // If one of the disc or track values are not numbers, then we will error out
        Ok(model::AudioFileTrackMetadata {
            artist: get_string_result(&tag_map, "artist", &path)?,
            album_artist: get_string_option(&tag_map, "albumartist"),
            album: get_string_option(&tag_map, "album"),
            disc_no: get_u32_result(&tag_map, "discnumber", &path)?,
            disc_total: get_u32_result(&tag_map, "disctotal", &path)?,
            track: get_u32_result(&tag_map, "tracknumber", &path)?,
            track_total: get_u32_result(&tag_map, "tracktotal", &path)?,
            track_title: get_string_result(&tag_map, "title", &path)?,
            genre: get_string_option(&tag_map, "genre"),
            date: get_string_option(&tag_map, "date"),
        })
    }

    fn get_string_option(tag_map: &BTreeMap<String, String>, key: &'static str) -> Option<String> {
        tag_map.get(key).map(|x| x.clone())
    }

    fn get_string_result(
        tag_map: &BTreeMap<String, String>,
        key: &'static str,
        path: &path::PathBuf,
    ) -> Result<String, model::Error> {
        Ok(tag_map
            .get(key)
            .ok_or(model::Error::MissingMetadataKey(
                path.to_string_lossy().to_string(),
                key,
            ))?
            .clone())
    }

    fn get_u32_result(
        tag_map: &BTreeMap<String, String>,
        key: &'static str,
        path: &path::PathBuf,
    ) -> Result<Option<u32>, model::Error> {
        Ok(match tag_map.get(key) {
            Some(v) => Some(v.parse::<u32>().map_err(|_| {
                model::Error::ExpectedU32MetadataValue(path.to_string_lossy().to_string(), key)
            })?),
            None => None,
        })
    }
}
