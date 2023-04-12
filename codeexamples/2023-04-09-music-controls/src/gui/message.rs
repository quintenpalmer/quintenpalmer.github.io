use crate::shared;

#[derive(Debug, Clone)]
pub enum Message {
    Nav(Navigate),
    SinkCallback(shared::SinkCallbackMessage),
}

#[derive(Debug, Clone)]
pub enum Navigate {
    Home,
    ArtistList,
    ArtistAlbumList(String),
    AlbumTrackList(String, String),
}
