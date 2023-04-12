use iced;

use crate::datastore;

use super::{message, state, update, view};

impl iced::Sandbox for state::State {
    type Message = message::Message;

    fn new() -> Self {
        state::State {
            page: state::Page::Home,
            datastore: datastore::model::Library::from_library_directory(".").unwrap(),
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
