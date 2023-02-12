use simpleaudioparser as datastore;

mod gui;

use iced::Sandbox;

fn main() {
    gui::state::State::run(iced::Settings::default()).unwrap();
}
