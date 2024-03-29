use std::path;

use crate::{model, util};

pub fn parse_all_audio_files(
    paths: Vec<path::PathBuf>,
) -> Result<Vec<model::AudioFileTrackMetadata>, model::Error> {
    paths
        .into_iter()
        .map(|audio_file_path| parse_single_audio_file(audio_file_path))
        .collect()
}

pub fn parse_single_audio_file(
    audio_file_path: path::PathBuf,
) -> Result<model::AudioFileTrackMetadata, model::Error> {
    let maybe_extension = util::get_maybe_extension_string(&audio_file_path);

    match maybe_extension {
        Some(extension) => match extension.as_str() {
            "flac" => flac::parse_flac_file(audio_file_path),
            "mp3" => id3::parse_mp3_file(audio_file_path),
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
            disc_no: get_u32_optional_result(&tag_map, "discnumber", &path)?,
            disc_total: get_u32_optional_result(&tag_map, "disctotal", &path)?,
            track: get_u32_optional_result(&tag_map, "tracknumber", &path)?,
            track_total: get_u32_optional_result(&tag_map, "tracktotal", &path)?,
            track_title: get_string_result(&tag_map, "title", &path)?,
            genre: get_string_option(&tag_map, "genre"),
            date: get_string_option(&tag_map, "date"),
            full_path: path,
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

    fn get_u32_optional_result(
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

mod id3 {
    use std::path;

    use id3::{self, TagLike};

    use crate::model;

    pub fn parse_mp3_file(
        path: path::PathBuf,
    ) -> Result<model::AudioFileTrackMetadata, model::Error> {
        let tag = id3::Tag::read_from_path(&path)?;

        Ok(model::AudioFileTrackMetadata {
            artist: get_string_result(tag.artist(), "artist", &path)?,
            album_artist: tag.album_artist().map(|x| x.to_string()),
            album: tag.album().map(|x| x.to_string()),
            disc_no: tag.disc(),
            disc_total: tag.total_discs(),
            track: tag.track(),
            track_total: tag.total_tracks(),
            track_title: get_string_result(tag.title(), "title", &path)?,
            genre: tag.genre().map(|x| x.to_string()),
            date: tag.year().map(|x| x.to_string()),
            full_path: path,
        })
    }

    fn get_string_result(
        val: Option<&str>,
        key: &'static str,
        path: &path::PathBuf,
    ) -> Result<String, model::Error> {
        Ok(val
            .ok_or(model::Error::MissingMetadataKey(
                path.to_string_lossy().to_string(),
                key,
            ))?
            .to_string())
    }
}
