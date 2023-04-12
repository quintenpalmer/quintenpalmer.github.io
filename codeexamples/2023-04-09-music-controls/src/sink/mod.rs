use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::shared;

const BLOCKING_TIMEOUT_MILLISECONDS: u64 = 1_000;

pub fn create_backend_with_client_and_callback() -> (
    mpsc::Sender<shared::SinkMessage>,
    mpsc::Receiver<shared::SinkCallbackMessage>,
) {
    let (sender_for_client, recv_for_backend) = mpsc::channel();

    let (callback_from_backend, callback_to_client) = mpsc::channel();

    thread::spawn(move || run_forever(recv_for_backend, callback_from_backend));

    (sender_for_client, callback_to_client)
}

fn run_forever(
    rx: mpsc::Receiver<shared::SinkMessage>,
    callback: mpsc::Sender<shared::SinkCallbackMessage>,
) {
    println!("SINK:\tstarting to listen...");

    let sink = SinkPlayback::new();

    sink.run_forever(rx, callback);

    println!("SINK:\tdone listening");
}

pub struct SinkPlayback {
    _stream: rodio::OutputStream,
    sink: rodio::Sink,
    stream_handle: rodio::OutputStreamHandle,
    // If None there is no loaded song;
    // If Some(x) the inner bool is whether the song is currently playing
    loaded_song_playing: Option<bool>,
}

impl SinkPlayback {
    pub fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        SinkPlayback {
            _stream: stream,
            sink: rodio::Sink::try_new(&stream_handle).unwrap(),
            stream_handle: stream_handle,
            loaded_song_playing: None,
        }
    }

    pub fn run_forever(
        mut self,
        rx: mpsc::Receiver<shared::SinkMessage>,
        callback: mpsc::Sender<shared::SinkCallbackMessage>,
    ) {
        loop {
            match rx.recv_timeout(time::Duration::from_millis(BLOCKING_TIMEOUT_MILLISECONDS)) {
                Ok(msg) => self.handle_msg(msg, &callback),
                Err(mpsc::RecvTimeoutError::Timeout) => self.handle_timeout(&callback),
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    println!("recv sees that all clients have closed");
                    break;
                }
            }
        }
    }

    fn handle_msg(
        &mut self,
        msg: shared::SinkMessage,
        callback: &mpsc::Sender<shared::SinkCallbackMessage>,
    ) {
        println!("SINK:\t handling resp: {:?}", msg);
        match msg {
            shared::SinkMessage::PlayButton => match self.loaded_song_playing {
                Some(ref mut playing) => {
                    *playing = true;
                    self.sink.play();
                    callback.send(shared::SinkCallbackMessage::Playing).unwrap();
                }
                None => (),
            },
            shared::SinkMessage::PauseButton => match self.loaded_song_playing {
                Some(ref mut playing) => {
                    *playing = false;
                    self.sink.pause();
                    callback.send(shared::SinkCallbackMessage::Paused).unwrap();
                }
                None => (),
            },
            shared::SinkMessage::LoadSong(path) => {
                self.loaded_song_playing = Some(true);
                self.sink.stop();
                self.sink = rodio::Sink::try_new(&self.stream_handle).unwrap();

                let file = io::BufReader::new(fs::File::open(path).unwrap());
                self.sink.append(rodio::Decoder::new(file).unwrap());
                self.sink.play();
                callback.send(shared::SinkCallbackMessage::Playing).unwrap();
            }
        }
    }

    fn handle_timeout(&mut self, callback: &mpsc::Sender<shared::SinkCallbackMessage>) {
        match self.loaded_song_playing {
            Some(_playing) => {
                if self.sink.len() == 0 {
                    println!("SINK:\ttimeout on recv poll and we noticed the song was over");
                    self.loaded_song_playing = None;
                    callback
                        .send(shared::SinkCallbackMessage::SongEnded)
                        .unwrap();
                }
            }
            None => (),
        }
    }
}
