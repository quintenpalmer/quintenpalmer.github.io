use super::{message, state};

pub fn handle_message(state: &mut state::State, message: message::Message) {
    match message {
        message::Message::Nav(nav_message) => handle_nav(state, nav_message),
    }
}

fn handle_nav(state: &mut state::State, nav_message: message::Navigate) {
    match nav_message {
        message::Navigate::Home => state.page = state::Page::Home,
        message::Navigate::ArtistList => state.page = state::Page::ArtistList,
        message::Navigate::ArtistAlbumList(artist_name) => {
            state.page = state::Page::ArtistAlbumList(artist_name)
        }
        message::Navigate::AlbumTrackList(artist_name, album_name) => {
            state.page = state::Page::AlbumTrackList(artist_name, album_name)
        }
    }
}
