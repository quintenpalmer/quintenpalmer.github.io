use simpleaudioparser as datastore;
use simplemusicplayback::shared as shared;
use simplemusicplayback::backend as sink;

mod gui;

use iced::Application;

fn main() {
    gui::state::State::run(iced::Settings::default()).unwrap();
}
