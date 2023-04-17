use iced;
use iced::widget::{button, text, Column, Scrollable};

use crate::datastore;

use super::super::message;

pub fn view_artist_list<'a>(
    datastore: &'a datastore::model::Library,
) -> (
    iced::Element<'a, message::Message>,
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    let breadcrumbs =
        vec![button("Artists").on_press(message::Message::Nav(message::Navigate::ArtistList))];

    let mut artist_list_column = Column::new();
    for artist_name in datastore.artists.keys() {
        artist_list_column = artist_list_column.push(button(text(artist_name.clone())).on_press(
            message::Message::Nav(message::Navigate::ArtistAlbumList(artist_name.clone())),
        ))
    }

    (
        Column::new()
            .padding(10)
            .push(text("Artists:").size(46))
            .push(Scrollable::new(artist_list_column))
            .into(),
        breadcrumbs,
    )
}
