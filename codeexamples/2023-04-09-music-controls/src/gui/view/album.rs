use iced;
use iced::widget::{button, text, Column, Row, Scrollable, Space};

use crate::datastore;

use super::message;

pub fn view_album_track_list<'a>(
    artist_name: String,
    album_name: String,
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
        button(text(album_name.clone())).on_press(message::Message::Nav(
            message::Navigate::AlbumTrackList(artist_name.clone(), album_name.clone()),
        )),
    ];

    let mut discs_column = Column::new().padding(10);
    for disc in datastore
        .artists
        .get(&artist_name)
        .unwrap()
        .albums
        .get(&album_name)
        .unwrap()
        .discs
        .values()
    {
        let mut tracks_column = Column::new().padding(10);
        for track in disc.tracks.values() {
            let track_row = Row::new()
                .spacing(10)
                .push(text(format!("{:>3}", track.track.unwrap_or(1))).size(26))
                .push(text(track.track_title.clone()).size(26));
            tracks_column = tracks_column.push(track_row);
        }
        discs_column = discs_column
            .push(text(format!("Disc: {}", disc.number)))
            .push(tracks_column);
    }

    (
        Column::new()
            .padding(10)
            .push(
                Row::new()
                    .push(text(album_name).size(46))
                    .push(text("(Album)").size(26)),
            )
            .push(
                Row::new()
                    .push(Space::with_width(iced::Length::Units(50)))
                    .push(text(artist_name).size(36))
                    .push(text("(Artist)").size(26)),
            )
            .push(text("Tracks:").size(36))
            .push(Scrollable::new(discs_column))
            .into(),
        breadcrumbs,
    )
}
