use crate::datastore;

pub struct State {
    pub page: Page,
    pub datastore: datastore::model::Library,
}

pub enum Page {
    Home,
    ArtistList,
    ArtistAlbumList(String),
    AlbumTrackList(String, String),
}
