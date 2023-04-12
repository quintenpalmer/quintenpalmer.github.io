use iced;
use iced::widget::{button, text, Column};

use super::super::message;

pub fn view_home<'a>() -> (
    iced::Element<'a, message::Message>,
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    (
        Column::new()
            .padding(10)
            .push(text("Welcome").size(46))
            .push(button("Artists").on_press(message::Message::Nav(message::Navigate::ArtistList)))
            .into(),
        Vec::new(),
    )
}
