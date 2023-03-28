#[derive(Clone, Debug)]
pub enum SinkMessage {
    PlayButton,
    PauseButton,
    LoadSong(String),
}

#[derive(Clone, Debug)]
pub enum SinkCallbackMessage {
    Playing,
    Paused,
    SongEnded,
}
