use std::collections::BTreeMap;
use std::io;
use std::path;

use claxon;
use id3;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Claxon(claxon::Error),
    ID3(id3::Error),
    MissingMetadataKey(String, &'static str),
    ExpectedU32MetadataValue(String, &'static str),
    ConflictingTrack(String, String, u32, u32, String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<claxon::Error> for Error {
    fn from(e: claxon::Error) -> Self {
        Error::Claxon(e)
    }
}

impl From<id3::Error> for Error {
    fn from(e: id3::Error) -> Self {
        Error::ID3(e)
    }
}

pub struct Library {
    pub artists: BTreeMap<String, Artist>,
}

pub struct Artist {
    pub name: String,
    pub albums: BTreeMap<String, Album>,
}

pub struct Album {
    pub name: String,
    pub discs: BTreeMap<u32, Disc>,
}

pub struct Disc {
    pub number: u32,
    pub tracks: BTreeMap<u32, AudioFileTrackMetadata>,
}

pub struct AudioFileTrackMetadata {
    pub artist: String,
    pub album_artist: Option<String>,
    pub album: Option<String>,
    pub disc_no: Option<u32>,
    pub disc_total: Option<u32>,
    pub track: Option<u32>,
    pub track_total: Option<u32>,
    pub track_title: String,
    pub genre: Option<String>,
    pub date: Option<String>,
    pub full_path: path::PathBuf,
}

impl AudioFileTrackMetadata {
    pub fn resolve_album_artist(&self) -> String {
        // Use the album_artist if specified,
        // otherwise just use the artist
        match self.album_artist {
            Some(ref v) => v.clone(),
            None => self.artist.clone(),
        }
    }

    pub fn resolve_album(&self) -> String {
        // If there is no album specified,
        // assume it is a single and the album
        // can just be the track title
        match self.album {
            Some(ref v) => v.clone(),
            None => self.track_title.clone(),
        }
    }

    pub fn resolve_disc_number(&self) -> u32 {
        // If there is no disc specified,
        // assume that it is a single disc release
        match self.disc_no {
            Some(ref v) => *v,
            None => 1,
        }
    }

    pub fn resolve_track_number(&self) -> u32 {
        // If there is no track number assigned,
        // assume that it's a single with just one track
        // as part of it's "album" of a release
        match self.track {
            Some(ref v) => *v,
            None => 1,
        }
    }
}
