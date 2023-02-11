#[derive(Debug, Clone)]
pub enum Message {
    Nav(Navigate),
}

#[derive(Debug, Clone)]
pub enum Navigate {
    Home,
    ArtistList,
    ArtistAlbumList(String),
    AlbumTrackList(String, String),
}
