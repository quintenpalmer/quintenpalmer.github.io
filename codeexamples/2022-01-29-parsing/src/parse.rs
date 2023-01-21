use std::collections::BTreeMap;
use std::path;

use crate::model;

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
    let reader = claxon::FlacReader::open(&path)?;

    let tag_map = reader
        .tags()
        .map(|(k, v)| (k.to_string().to_lowercase(), v.to_string()))
        .collect::<BTreeMap<String, String>>();

    // Note: "artist" and "title" are the only keys we require, the rest may or may not be set
    // If one of the disc or track values are not numbers, then we will error out
    Ok(model::AudioFileTrackMetadata {
        artist: tag_map
            .get("artist")
            .ok_or(model::Error::MissingMetadataKey(
                path.to_string_lossy().to_string(),
                "artist",
            ))?
            .clone(),
        album_artist: tag_map.get("albumartist").map(|x| x.clone()),
        album: tag_map.get("album").map(|x| x.clone()),
        disc_no: match tag_map.get("discnumber") {
            Some(v) => Some(v.parse::<u32>().map_err(|_| {
                model::Error::ExpectedU32MetadataValue(
                    path.to_string_lossy().to_string(),
                    "discnumber",
                )
            })?),
            None => None,
        },
        disc_total: match tag_map.get("disctotal") {
            Some(v) => Some(v.parse::<u32>().map_err(|_| {
                model::Error::ExpectedU32MetadataValue(
                    path.to_string_lossy().to_string(),
                    "disctotal",
                )
            })?),
            None => None,
        },
        track: match tag_map.get("tracknumber") {
            Some(v) => Some(v.parse::<u32>().map_err(|_| {
                model::Error::ExpectedU32MetadataValue(
                    path.to_string_lossy().to_string(),
                    "tracknumber",
                )
            })?),
            None => None,
        },
        track_total: match tag_map.get("tracktotal") {
            Some(v) => Some(v.parse::<u32>().map_err(|_| {
                model::Error::ExpectedU32MetadataValue(
                    path.to_string_lossy().to_string(),
                    "tracktotal",
                )
            })?),
            None => None,
        },
        track_title: tag_map
            .get("title")
            .ok_or(model::Error::MissingMetadataKey(
                path.to_string_lossy().to_string(),
                "title",
            ))?
            .clone(),
        genre: tag_map.get("genre").map(|x| x.clone()),
        date: tag_map.get("date").map(|x| x.clone()),
    })
}
