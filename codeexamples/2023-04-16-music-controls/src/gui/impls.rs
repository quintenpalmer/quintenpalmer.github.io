use std::cell;

use iced;

use crate::datastore;
use crate::sink;

use super::{message, state, subscription, update, view};

impl iced::Application for state::State {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = message::Message;
    type Theme = iced::Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (sink_sender, sink_recv) = sink::create_backend_with_client_and_callback();
        let state = state::State {
            page: state::Page::Home,
            datastore: datastore::model::Library::from_library_directory(".").unwrap(),
            playback: state::PlaybackInfo {
                currently_playing: None,
            },
            sink: state::Sink {
                sink_message_sender: sink_sender,
                sink_callback_recv: cell::RefCell::new(Some(sink_recv)),
            },
        };
        (state, iced::Command::none())
    }

    fn title(&self) -> String {
        "Simple Music Viewer".to_string()
    }

    fn update(&mut self, message: message::Message) -> iced::Command<Self::Message> {
        update::handle_message(self, message)
    }

    fn view(&self) -> iced::Element<message::Message> {
        view::view_state(self)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::sink_callback(&self)
    }
}
