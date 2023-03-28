use std::sync::mpsc;
use std::thread;
use std::time;

use crate::backend;
use crate::shared;

pub fn looping_main() {
    let (sender, rx) = backend::create_backend_with_client_and_callback();
    play_tone_with_pause(&sender, &rx, "flacs/four.flac".to_string());
    play_tone_with_pause(&sender, &rx, "flacs/three.flac".to_string());
    play_tone_with_pause(&sender, &rx, "flacs/two_higher.flac".to_string());
    play_tone_with_pause(&sender, &rx, "flacs/two_lower.flac".to_string());
    play_tone_with_pause(&sender, &rx, "flacs/one.flac".to_string());
}

fn play_tone_with_pause(
    sender: &mpsc::Sender<shared::SinkMessage>,
    rx: &mpsc::Receiver<shared::SinkCallbackMessage>,
    filename: String,
) {
    let mut should_pause_once = true;
    sender
        .send(shared::SinkMessage::LoadSong(filename))
        .unwrap();
    loop {
        match rx.recv_timeout(time::Duration::from_millis(1_000)) {
            Ok(msg) => match msg {
                shared::SinkCallbackMessage::Playing => println!("we heard that we are playing"),
                shared::SinkCallbackMessage::Paused => println!("we heard that we are paused"),
                shared::SinkCallbackMessage::SongEnded => {
                    println!("we learned that the song ended");
                    break;
                }
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {
                println!("waited for a second");
                if should_pause_once {
                    println!("pausing for two second");
                    sender.send(shared::SinkMessage::PauseButton).unwrap();
                    thread::sleep(time::Duration::from_millis(2_000));
                    println!("resuming play after those seconds");
                    sender.send(shared::SinkMessage::PlayButton).unwrap();
                    should_pause_once = false;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("recv sees that all clients have closed");
                break;
            }
        }
    }
}
