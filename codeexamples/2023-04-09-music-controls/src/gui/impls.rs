use std::cell;

use iced;

use crate::datastore;
use crate::sink;

use super::{message, state, update, view};

impl iced::Sandbox for state::State {
    type Message = message::Message;

    fn new() -> Self {
        let (sink_sender, sink_recv) = sink::create_backend_with_client_and_callback();
        state::State {
            page: state::Page::Home,
            datastore: datastore::model::Library::from_library_directory(".").unwrap(),
            playback: state::PlaybackInfo {
                currently_playing: None,
            },
            sink: state::Sink {
                sink_message_sender: sink_sender,
                sink_callback_recv: cell::RefCell::new(Some(sink_recv)),
            },
        }
    }

    fn title(&self) -> String {
        "Simple Music Viewer".to_string()
    }

    fn update(&mut self, message: message::Message) {
        update::handle_message(self, message)
    }

    fn view(&self) -> iced::Element<message::Message> {
        view::view_state(self)
    }
}
