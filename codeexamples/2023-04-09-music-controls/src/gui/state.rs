use crate::datastore;

pub struct State {
    pub page: Page,
    pub datastore: datastore::model::Library,
    pub playback: PlaybackInfo,
}

pub enum Page {
    Home,
    ArtistList,
    ArtistAlbumList(String),
    AlbumTrackList(String, String),
}

pub struct PlaybackInfo {
    pub currently_playing: Option<(datastore::model::AudioFileTrackMetadata, bool)>,
}
