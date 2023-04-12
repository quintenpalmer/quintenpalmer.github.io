use iced;
use iced::widget::{button, Column, Row, Scrollable};

use crate::datastore;

use super::{message, state};

mod album;
mod artistalbums;
mod artists;
mod home;

pub fn view_state<'a>(state: &'a state::State) -> iced::Element<'a, message::Message> {
    let (body, breadcrumbs) = view_page(&state.page, &state.datastore);

    let mut crumb_button_row = Row::new()
        .spacing(10)
        .push(button("Home").on_press(message::Message::Nav(message::Navigate::Home)));

    for crumb_button in breadcrumbs.into_iter() {
        crumb_button_row = crumb_button_row.push(Scrollable::new(crumb_button));
    }


    let mut ret = Column::new();

    ret = ret.push(crumb_button_row);
    ret = ret.push(body);
    ret.into()
}

fn view_page<'a>(
    page: &'a state::Page,
    datastore: &'a datastore::model::Library,
) -> (
    iced::Element<'a, message::Message>,
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    match page {
        state::Page::Home => home::view_home(),
        state::Page::ArtistList => artists::view_artist_list(&datastore),
        state::Page::ArtistAlbumList(ref artist_name) => {
            artistalbums::view_artist_album_list(artist_name.clone(), &datastore)
        }
        state::Page::AlbumTrackList(ref artist_name, ref album_name) => {
            album::view_album_track_list(artist_name.clone(), album_name.clone(), &datastore)
        }
    }
}
