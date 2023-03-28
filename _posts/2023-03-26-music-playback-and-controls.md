---
title: Music Playback and Controls
date: 2023-03-26
---

# Introduction

Welcome back to another installment of this music-tech blog! This time we're getting back into some technical topics, so put on your hard hats and let's dig in! Our topic this time will be playing back music files and some rudimentary controls we can provide to interface with the playback "sink" as we'll be calling it today. Here we go!

# Welcome to the Rodio

[Rodio](https://docs.rs/rodio/latest/rodio/) is a Rust crate that provides a very simple interface for interacting with audio files and the physical hardware on your machine. It is built on top of another crate called [`cpal`](https://docs.rs/cpal/latest/cpal/) which stands for "Cross-Platform Audio Library", which is what does a lot of the under hood work to resolve what how to talk to the operating system and the physical hardware. As always with this technology, we're going to let Rodio and CPAL do their heavy lifting and not dig into their inner workings here, but if that interests you, it's all still open source! Let's start by learning the definitions of some of the terms that these projects use (and expose).

## Terms

### Source

A [`Source`](https://docs.rs/rodio/latest/rodio/source/trait.Source.html) in Rodio's world is something that represents a sound to play, be it a raw sine wave, a flac file, or some other data that could be decoded. As long as it satisfies this `Source` trait, Rodio will be happy to help you play it back. And conveniently enough, Rodio does provide implementations for `Source` that decode from common audio file formats, which we will definitely be leveraging!

### Output Stream

An [`Output Stream`](https://docs.rs/rodio/latest/rodio/struct.OutputStream.html) and [`OutputStreamHandle`] both represent the controls over the devices that you can stream `Source` data into. They handle all of the cross-platform logic and do conveniently offer a simple constructor [`OutputStream::try_default`](https://docs.rs/rodio/latest/rodio/struct.OutputStream.html#method.try_default) which will try to get a handle on what the OS provides as the default device.

### Sink

Rodio also provides a nice struct called a [`Sink`](https://docs.rs/rodio/latest/rodio/struct.Sink.html) that offers more control over the audio playback, like playing, pausing, appending sources, and more. We will be using this in our demo today.

## A (Very) Very Simple Playback Example

Let's start with something that just plays back a `.flac` file that we've generated with a simple sine wave.

### **`Cargo.toml`**
```toml
[package]
name = "simplestmusicplayback"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rodio = "0.16"
```

Almost as bog-standard as a `Cargo.toml` can get, we just have the aforementioned `rodio` crate as a dependency.

### **`src/main.rs`**
```rust
use std::fs;
use std::io;
use std::path;

pub fn main() {
    println!("starting...");
    // We get the default device as provided by the host/operating system
    // We don't actually need the `_stream` value that we're ignoring,
    // but the `stream_handle` is how we will build our Sink in the near future
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

	// And we pass the `stream_handle` to build a new `Sink`
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    println!("we have a sink");

	// We will open the file at `flacs/sample.flac`
    let flac_path = "flacs/sample.flac";
    let path = {
        let mut inner = path::PathBuf::new();
        inner.push(flac_path);
        inner
    };
    println!("we have a path to the \"{}\" file", flac_path);

	// We buffer the file, honestly for this demo it's more for fun,
	// but it is advised in general to buffer
    let file = io::BufReader::new(fs::File::open(path).unwrap());
    println!("we have an open and buffered .flac file");

	// Rodio is very generous and provides a decoder for flac files,
	// which we leverage and add to our sink's queue
    sink.append(rodio::Decoder::new(file).unwrap());
    println!("the sink now has the (buffered) file");

	// Actually play back the file!
    sink.play();
    println!("the sink is playing");

	// If the main thread were to exit then we wouldn't get to listen
	// to this beautiful sine wave!
	// The sink will gladly sleep until playback is done if asked to
    println!("the sink is sleeping until playback is done");
    sink.sleep_until_end();
    println!("the sink is done");

    println!("...exiting");
}
```

Hopefully those in-line comments demonstrate what is happening at each step along the way. While not exciting, here is what this _looks_ like when you run it, you can copy paste this into a new project if you want to experience this demo.

```
 cargo run -q
starting...
we have a sink
we have a path to the "flacs/sample.flac" file
we have an open and buffered .flac file
the sink now has the (buffered) file
the sink is playing
the sink is sleeping until playback is done
the sink is done
...exiting
```

I don't have much more to add with this small demo, let's try something a bit more complex.

## A Very Simple Playback Example

This will still be relatively simple, but we will use threads and channels to communicate. No actual `async` though, don't worry!

### **`Cargo.toml`**
```toml
[package]
name = "simplemusicplayback"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rodio = "0.16"
```

Same old `rodio` dependency here.

### **`src/main.rs`**
```rust
mod backend;
mod client;
mod shared;

fn main() {
    client::looping_main()
}
```

Also a pretty simple `main.rs`, I've deferred all of the exciting stuff off into these modules, let's take a look at them.

### **`src/shared.rs`**
```rust
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
```

These are just simple messages that we'll pass back and forth from our backend sink and our basic client.

Let's take a look at the backend sink in two parts. This first part is our Sink-wrapper; it's a lot, so hopefully some context before diving in will help. It's main purpose is to allow asynchronous playback of music with two-way communication between this sink wrapper (the backend) and a client on the other end. The communication is what the `shared` module above exposes, playing, pausing, loading songs, and communicating the end of songs. Let's try it:

### **`src/backend.rs`**
```rust
use std::fs;
use std::io;
// multi-producer, single-consumer channels
// are how we will communicate with this backend sink
use std::sync::mpsc;
use std::time;

use crate::shared;

const BLOCKING_TIMEOUT_MILLISECONDS: u64 = 1_000;

pub struct SinkPlayback {
	// We need to keep a handle on the `OutputStream`
	// otherwise Rust will drop it and then we can't play anything back
    _stream: rodio::OutputStream,
	// This is the same `Sink` as the simpler demo!
    sink: rodio::Sink,
    // And we hold on to the `OutputStreamHandle` when we want
    // to create a new `Sink` from the same `OutputStream` we create initially
    stream_handle: rodio::OutputStreamHandle,
    // If this is None there is no loaded song;
    // If this has Some(boolean) the inner bool is whether the song is currently playing
    loaded_song_playing: Option<bool>,
}

impl SinkPlayback {
    pub fn new() -> Self {
	    // Same default-device functionality we're using again here
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        SinkPlayback {
	        // As stated in the struct definition,
	        // hold on to the `stream` just so it doesn't get dropped
            _stream: stream,
            sink: rodio::Sink::try_new(&stream_handle).unwrap(),
            stream_handle: stream_handle,
            // We start with no playing audio, so this is `None`
            loaded_song_playing: None,
        }
    }

	// This is the entry-point into this backend-sink's functionality
    pub fn run_forever(
        mut self,
        // It accepts a receiver for new messages to control playback
        rx: mpsc::Receiver<shared::SinkMessage>,
        // And this sender is where it can report things happening
        // Especially when a song ends, which the client wouldn't be able
        // to know on its own
        callback: mpsc::Sender<shared::SinkCallbackMessage>,
    ) {
	    // We just run forever
        loop {
	        // We block for 1 second (1,000 milliseconds) and...
            match rx.recv_timeout(time::Duration::from_millis(BLOCKING_TIMEOUT_MILLISECONDS)) {
	            // If we get a message, we handle it
                Ok(msg) => self.handle_msg(msg, &callback),
                // And if we timed out, then we handle that case as well
                Err(mpsc::RecvTimeoutError::Timeout) => self.handle_timeout(&callback),
                // But if we disconnected, then we just abort
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    println!("recv sees that all clients have closed");
                    break;
                }
            }
        }
    }

    fn handle_msg(
        &mut self,
        // This is the message that we received from the receiver
        msg: shared::SinkMessage,
        // This is the callback sender to report back up to the client
        callback: &mpsc::Sender<shared::SinkCallbackMessage>,
    ) {
        println!("SINK:\t handling resp: {:?}", msg);
        match msg {
	        // If the client requested to resume playback:
            shared::SinkMessage::PlayButton => match self.loaded_song_playing {
		        // Only do anything if we have a song loaded
                Some(ref mut playing) => {
	                // Set playing to true (we don't care if it was already true)
                    *playing = true;
                    // And actually have the sink resume playback
                    self.sink.play();
		            // Communicate up to the client that we are currently playing
                    callback.send(shared::SinkCallbackMessage::Playing).unwrap();
                }
                None => (),
            },
            // If the client requested to pause playback:
            shared::SinkMessage::PauseButton => match self.loaded_song_playing {
                Some(ref mut playing) => {
	                // Set playing to false (we don't care if it was already false)
                    *playing = false;
                    // And actually have the sink pause playback
                    self.sink.pause();
		            // Communicate up to the client that we are currently paused
                    callback.send(shared::SinkCallbackMessage::Paused).unwrap();
                }
                None => (),
            },
            // If the client requested to load a new song (by path):
            shared::SinkMessage::LoadSong(path) => {
	            // Set that we will have a song and will be playing it
                self.loaded_song_playing = Some(true);
                // Stop the old sink
                self.sink.stop();
                // Build a new sink from our handle
                self.sink = rodio::Sink::try_new(&self.stream_handle).unwrap();

				// Load the file into a buffered reader and append it to the
				// Sink's queue
                let file = io::BufReader::new(fs::File::open(path).unwrap());
                self.sink.append(rodio::Decoder::new(file).unwrap());
                // Begin playback on the loaded file
                self.sink.play();
                // Communicate up to the client that we are currently playing
                callback.send(shared::SinkCallbackMessage::Playing).unwrap();
            }
        }
    }

	// When we handle a timeout, we may still need to communicate up the client with the sender
    fn handle_timeout(&mut self, callback: &mpsc::Sender<shared::SinkCallbackMessage>) {
        match self.loaded_song_playing {
            Some(_playing) => {
	            // If we have a loaded song and it's length is 0, then we are done with playback of this track
                if self.sink.len() == 0 {
                    println!("SINK:\ttimeout on recv poll and we noticed the song was over");
                    // Set that we no longer have an active track
                    // that we're playing
                    self.loaded_song_playing = None;

                    // And communicate that up to the client
                    callback
                        .send(shared::SinkCallbackMessage::SongEnded)
                        .unwrap();
                }
            }
            None => (),
        }
    }
}
```

Ok, that was the most I'm going to throw at you today, hopefully you can follow what's going on there. The rest of the file is just a convenient "constructor" that will start this sink running forever in a seperate thread and return the sender/receiver to communicate with the spawned thread.

### **`src/backend.rs`** (continued)
```rust
use std::sync::mpsc;
use std::thread;

use crate::shared;

pub fn create_backend_with_client_and_callback() -> (
    mpsc::Sender<shared::SinkMessage>,
    mpsc::Receiver<shared::SinkCallbackMessage>,
) {
	// Create the channel to send data into the sink
    let (sender_for_client, recv_for_backend) = mpsc::channel();

	// Create the channel to receive data from the sink
    let (callback_from_backend, callback_to_client) = mpsc::channel();

	// Spawn the `run_forever` function with the correct receiver and sender for the sink
    thread::spawn(move || run_forever(recv_for_backend, callback_from_backend));

	// Return the correct sender and receiver for the client
    (sender_for_client, callback_to_client)
}

fn run_forever(
    rx: mpsc::Receiver<shared::SinkMessage>,
    callback: mpsc::Sender<shared::SinkCallbackMessage>,
) {
    println!("SINK:\tstarting to listen...");

	// Create the sink...
    let sink = SinkPlayback::new();

	// ...and let it rip!
    sink.run_forever(rx, callback);

    println!("SINK:\tdone listening");
}


// And then all of the code you just saw
```

Ok, that's the backend, that's the worst of it, I promise! Let's look at a simple client now:

All this client does it, for each of the 5 files baked into its source code:
* Start playback of the file
* Wait 1 second while the file is playing
* Pause playback for 2 seconds
* Resume playback until the file is done

### **`src/client.rs`**
```rust
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::backend;
use crate::shared;

// This is our entry point for our client
pub fn looping_main() {
	// We create the sink and get the channels to communicate with it
    let (sender, rx) = backend::create_backend_with_client_and_callback();
    // And then we just play these 5 files!
    play_tone_with_pause(&sender, &rx, "flacs/four.flac".to_string());
    play_tone_with_pause(&sender, &rx, "flacs/three.flac".to_string());
    play_tone_with_pause(&sender, &rx, "flacs/two_higher.flac".to_string());
    play_tone_with_pause(&sender, &rx, "flacs/two_lower.flac".to_string());
    play_tone_with_pause(&sender, &rx, "flacs/one.flac".to_string());
}

// This should just do what it says on the tin; let's verify
fn play_tone_with_pause(
	// We have access to the sender to the sink
    sender: &mpsc::Sender<shared::SinkMessage>,
    // And the callback from the sink
    rx: &mpsc::Receiver<shared::SinkCallbackMessage>,
    // And the file to play, easy!
    filename: String,
) {
	// Just remember this is relating to pausing, we'll come back to this
    let mut should_pause_once = true;

	// Tell the Sink that we want to load a the provided file
    sender
        .send(shared::SinkMessage::LoadSong(filename))
        .unwrap();

	// This will break, it shouldn't actually run forever
    loop {
	    // Again, we block for 1 second, not neccessarily to match up
	    // with the backend sink, but just so that we can stop to do things
	    // (read: pause [and just for fun])) while the sink is running
        match rx.recv_timeout(time::Duration::from_millis(1_000)) {
	        // If we do get a message back!
            Ok(msg) => match msg {
	            // Just log that we now know we are playing
                shared::SinkCallbackMessage::Playing => println!("we heard that we are playing"),
                // Just log that we now know we are paused
                shared::SinkCallbackMessage::Paused => println!("we heard that we are paused"),
                // If the song has ended, then let's break out of this loop
                shared::SinkCallbackMessage::SongEnded => {
                    println!("we learned that the song ended");
                    break;
                }
            },
            // If we got a timeout,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                println!("waited for a second");
                // If we still should issue a pause
                if should_pause_once {
                    println!("pausing for two second");
                    // Actually tell the sink that we want to pause
                    sender.send(shared::SinkMessage::PauseButton).unwrap();
                    // Wait 2 seconds...
                    thread::sleep(time::Duration::from_millis(2_000));
                    println!("resuming play after those seconds");
                    // Then start playback again!
                    sender.send(shared::SinkMessage::PlayButton).unwrap();
                    // Mark that we should no longer pause
                    // and just let playback resume until it ends
                    should_pause_once = false;
                }
            }
            // If we are ever disconnected, log so and just break out of the loop
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("recv sees that all clients have closed");
                break;
            }
        }
    }
}
```

Hopefully that wasn't so bad. Feel free to read over both of those a few times, just to keep the different senders and receivers straight. That's all I have planned for this post, let's just _see_ (unfortunately I will not be uploading a video with sound, you'll have to try that yourself) how it looks when this demo runs:

```
 $ cargo run -q
SINK:	starting to listen...
SINK:	 handling resp: LoadSong("flacs/four.flac")
we heard that we are playing
waited for a second
pausing for two second
SINK:	 handling resp: PauseButton
resuming play after those seconds
we heard that we are paused
SINK:	 handling resp: PlayButton
we heard that we are playing
waited for a second
SINK:	timeout on recv poll and we noticed the song was over
we learned that the song ended
SINK:	 handling resp: LoadSong("flacs/three.flac")
we heard that we are playing
waited for a second
pausing for two second
SINK:	 handling resp: PauseButton
resuming play after those seconds
we heard that we are paused
SINK:	 handling resp: PlayButton
we heard that we are playing
waited for a second
waited for a second
waited for a second
SINK:	timeout on recv poll and we noticed the song was over
we learned that the song ended
SINK:	 handling resp: LoadSong("flacs/two_higher.flac")
we heard that we are playing
waited for a second
pausing for two second
SINK:	 handling resp: PauseButton
resuming play after those seconds
we heard that we are paused
SINK:	 handling resp: PlayButton
we heard that we are playing
waited for a second
waited for a second
waited for a second
SINK:	timeout on recv poll and we noticed the song was over
we learned that the song ended
SINK:	 handling resp: LoadSong("flacs/two_lower.flac")
we heard that we are playing
waited for a second
pausing for two second
SINK:	 handling resp: PauseButton
resuming play after those seconds
we heard that we are paused
SINK:	 handling resp: PlayButton
we heard that we are playing
waited for a second
waited for a second
SINK:	timeout on recv poll and we noticed the song was over
we learned that the song ended
SINK:	 handling resp: LoadSong("flacs/one.flac")
we heard that we are playing
waited for a second
pausing for two second
SINK:	 handling resp: PauseButton
resuming play after those seconds
we heard that we are paused
SINK:	 handling resp: PlayButton
we heard that we are playing
waited for a second
waited for a second
SINK:	timeout on recv poll and we noticed the song was over
waited for a second
we learned that the song ended
recv sees that all clients have closed
```

Ok, it **is** pretty boring without the timing information seeing when things are logged and without the sound to verify that it is actually paused and playing, but just for posterity there it is. If you want to run this yourself to actually see and hear it live...

## Uploaded Code Examples

As always (when applicable) I have the code from this post available on [github](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/main/codeexamples/2023-03-26-music-playback). Feel free to try it out and tinker with it, if you like. Ok, that's all for today, let's wrap this up and talk about future installments.

# Conclusion

Hopefully this shed some light on how to interface with Sources, Streams, and Sinks to provide (in this case, questionably) useful audio playback. In the next post, I want to tie this in with the primitive Iced GUI we had been building in previous posts; stay tuned for that. Until then, take care!