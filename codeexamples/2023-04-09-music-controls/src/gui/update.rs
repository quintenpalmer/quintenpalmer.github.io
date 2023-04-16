use std::sync::mpsc;

use crate::shared;

use super::{message, state};

pub fn handle_message(
    state: &mut state::State,
    message: message::Message,
) -> iced::Command<message::Message> {
    println!("handling _a_ message...");
    match message {
        message::Message::Nav(nav_message) => {
            println!("handling nav message");
            handle_nav(state, nav_message);
            iced::Command::none()
        }
        message::Message::Control(control_message) => {
            println!("handling control message");
            handle_control(state, control_message)
        }
        message::Message::SinkCallback(callb) => {
            println!("handling callback message");
            handle_sink_callback(state, callb);
            iced::Command::none()
        }
        message::Message::ErrorResponse(error_message) => {
            println!("handling error message");
            handle_error(state, error_message);
            iced::Command::none()
        }
    }
}

fn handle_nav(state: &mut state::State, nav_message: message::Navigate) {
    match nav_message {
        message::Navigate::Home => state.page = state::Page::Home,
        message::Navigate::ArtistList => state.page = state::Page::ArtistList,
        message::Navigate::ArtistAlbumList(artist_name) => {
            state.page = state::Page::ArtistAlbumList(artist_name)
        }
        message::Navigate::AlbumTrackList(artist_name, album_name) => {
            state.page = state::Page::AlbumTrackList(artist_name, album_name)
        }
    }
}

fn handle_sink_callback(state: &mut state::State, callback_message: shared::SinkCallbackMessage) {
    match callback_message {
        shared::SinkCallbackMessage::Playing => {
            println!("we're now officially playing");
            match state.playback.currently_playing {
                Some((ref _track, ref mut playing)) => *playing = true,
                None => (),
            }
        }
        shared::SinkCallbackMessage::Paused => {
            println!("we're now paused");
            match state.playback.currently_playing {
                Some((ref _track, ref mut playing)) => *playing = false,
                None => (),
            }
        }
        shared::SinkCallbackMessage::SongEnded => {
            println!("the song has officially ended");
            state.playback.currently_playing = None
        }
    }
}

fn handle_error(_state: &mut state::State, error_message: Result<(), String>) {
    match error_message {
        Ok(()) => println!("no error was seen"),
        Err(err_string) => println!("We had seen this error: {}", err_string),
    }
}

fn handle_control(
    state: &mut state::State,
    control_message: message::Control,
) -> iced::Command<message::Message> {
    match control_message {
        message::Control::PlayTrack(track) => {
            state.playback.currently_playing = Some((track.clone(), true));
            iced::Command::perform(
                MessageCommandSender::new(
                    state.sink.sink_message_sender.clone(),
                    shared::SinkMessage::LoadSong(track.full_path.to_string_lossy().to_string()),
                )
                .send_message(),
                message::Message::ErrorResponse,
            )
        }
    }
}

struct MessageCommandSender<T> {
    tx: mpsc::Sender<T>,
    message: T,
}

impl<T: std::fmt::Debug> MessageCommandSender<T> {
    fn new(tx: mpsc::Sender<T>, message: T) -> Self {
        MessageCommandSender {
            tx: tx,
            message: message,
        }
    }

    async fn send_message(self) -> Result<(), String> {
        match self.tx.send(self.message) {
            Ok(a) => {
                println!("GUI:\tresp was {:?}", a);
                Ok(())
            }
            Err(e) => {
                println!("GUI:\terr resp was {:?}", e);
                Err(format!("{:?}", e))
            }
        }
    }
}
