use iced;
use iced::widget::{button, text, Column, Row, Scrollable};

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

    let playback_info = view_playback_info(&state.playback);

    let mut ret = Column::new();

    ret = ret.push(crumb_button_row);
    ret = ret.push(body);
    match playback_info {
        Some(actual_playback_info) => ret = ret.push(actual_playback_info),
        None => (),
    };
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

fn view_playback_info<'a>(
    playback: &'a state::PlaybackInfo,
) -> Option<iced::Element<'a, message::Message>> {
    match playback.currently_playing {
        Some((ref track, playing)) => {
            let mut row = Row::new().spacing(10);
            if playing {
                row = row.push(text(" >"));
            } else {
                row = row.push(text("||"));
            }
            row = row.push(text(track.track_title.clone()));
            Some(row.into())
        }
        None => None,
    }
}
