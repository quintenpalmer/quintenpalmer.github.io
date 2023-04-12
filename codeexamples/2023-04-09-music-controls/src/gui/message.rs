use crate::shared;

#[derive(Debug, Clone)]
pub enum Message {
    Nav(Navigate),
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
