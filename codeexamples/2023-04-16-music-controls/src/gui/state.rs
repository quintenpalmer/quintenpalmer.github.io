use std::cell;
use std::sync::mpsc;

use crate::datastore;
use crate::shared;

pub struct State {
    pub page: Page,
    pub datastore: datastore::model::Library,
    pub playback: PlaybackInfo,
    pub sink: Sink,
}

pub enum Page {
    Home,
    ArtistList,
    ArtistAlbumList(String),
    AlbumTrackList(String, String),
}

pub struct Sink {
    pub sink_message_sender: mpsc::Sender<shared::SinkMessage>,
    pub sink_callback_recv: cell::RefCell<Option<mpsc::Receiver<shared::SinkCallbackMessage>>>,
}

pub struct PlaybackInfo {
    pub currently_playing: Option<(datastore::model::AudioFileTrackMetadata, bool)>,
}
