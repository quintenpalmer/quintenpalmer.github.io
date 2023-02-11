use iced;
use iced::widget::{button, text, Column, Row, Scrollable};

use crate::datastore;

use super::super::message;

pub fn view_artist_album_list<'a>(
    artist_name: String,
    datastore: &'a datastore::model::Library,
) -> (
    iced::Element<'a, message::Message>,
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    let breadcrumbs = vec![
        button("Artists").on_press(message::Message::Nav(message::Navigate::ArtistList)),
        button(text(artist_name.clone())).on_press(message::Message::Nav(
            message::Navigate::ArtistAlbumList(artist_name.clone()),
        )),
    ];

    let mut albums_column = Column::new().padding(10);
    for album_name in datastore.artists.get(&artist_name).unwrap().albums.keys() {
        albums_column = albums_column.push(button(text(album_name.clone()).size(26)).on_press(
            message::Message::Nav(message::Navigate::AlbumTrackList(
                artist_name.clone(),
                album_name.clone(),
            )),
        ));
    }

    (
        Column::new()
            .padding(10)
            .push(
                Row::new()
                    .push(text(artist_name).size(46))
                    .push(text("(Artist)").size(26)),
            )
            .push(text("Albums:").size(36))
            .push(Scrollable::new(albums_column))
            .into(),
        breadcrumbs,
    )
}
