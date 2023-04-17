use crate::datastore;
use crate::shared;

#[derive(Debug, Clone)]
pub enum Message {
    Nav(Navigate),
    Control(Control),
    SinkCallback(shared::SinkCallbackMessage),
    ErrorResponse(Result<(), String>),
}

#[derive(Debug, Clone)]
pub enum Navigate {
    Home,
    ArtistList,
    ArtistAlbumList(String),
    AlbumTrackList(String, String),
}

#[derive(Debug, Clone)]
pub enum Control {
    Play,
    Pause,
    PlayTrack(datastore::model::AudioFileTrackMetadata),
}
