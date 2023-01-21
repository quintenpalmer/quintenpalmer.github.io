use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Error {}

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
}
