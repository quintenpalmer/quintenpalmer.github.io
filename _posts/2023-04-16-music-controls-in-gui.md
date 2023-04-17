---
title: Music Controls in GUI
date: 2023-04-16
---

# Introduction

Continuing from [last week](/2023/03/26/music-playback-and-controls), we're going to be adding media controls to our [music GUI](/2023/02/05/building-a-simple-iced-gui) that we've been building throughout this blog series. I highly recommend catching up with those posts before starting on this one. With those established as prerequisites, let's get started.

# Writing Our Media Controls

## Importing and Copy-Pasting

For this project, we're going to re-use existing code from previous posts! We're going to import the code from the early [parsing logic](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/5b0697df272b0c9ed7247a824517282cc0514504/codeexamples/2023-01-29-parsing) and also the most-recently written [playback functionality](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/5b0697df272b0c9ed7247a824517282cc0514504/codeexamples/2023-03-26-music-playback). We're then going to copy-paste the code from the afforementioned [music GUI](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/5b0697df272b0c9ed7247a824517282cc0514504/codeexamples/2023-03-26-music-playback). The reason we're copy-pasting the GUI logic is that we want to add functionality inside of so much of its inner workings that importing would just provide too much friction. Here's how we're getting to our starting point:

### `cargo` and `cp`
```
mkdir 2023-04-16-music-controls/
cargo init --name simplemusicplayback
cp -r ../2023-02-05-music-gui/src/gui/ 2023-04-16-music-controls/src/
```

Some simple `cargo init` and the `cp` to get the existing GUI logic.

### **`Cargo.toml`**
```toml
[package]
name = "simplemusiccontrols"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
iced = "0.7"

simpleaudioparser = { path = "../2023-01-29-parsing" }
simplemusicplayback = { path = "../2023-03-26-music-playback" }
```

We've now added [iced](https://docs.rs/iced/latest/iced) as well as the [parsing](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/b206885531b429be537819d57cdbd355f53c3c0e/codeexamples/2023-01-29-parsing) and [playback](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/b206885531b429be537819d57cdbd355f53c3c0e/codeexamples/2023-03-26-music-playback) logic as dependencies, let's take a look at our main before we make any actual changes.

### **`main.rs`**
```rust
use simpleaudioparser as datastore;
use simplemusicplayback::shared as shared;
use simplemusicplayback::backend as sink;

mod gui;

fn main() {
    println!("Hello, world!");
}
```

We have the copy-pasted `gui` module, as well as some renames of the `simple*` imports. arguably these could be named what we want in the `Cargo.toml` file, but we'll roll with this, if that's fine with everyone.

And to recap, a look at our `tree`:

### **`tree`**
```
tree -I target
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── gui
    │   ├── impls.rs
    │   ├── message.rs
    │   ├── mod.rs
    │   ├── state.rs
    │   ├── update.rs
    │   └── view
    │       ├── album.rs
    │       ├── artistalbums.rs
    │       ├── artists.rs
    │       ├── home.rs
    │       └── mod.rs
    └── main.rs

4 directories, 13 files
```

## Hooking Everything Up

Alright, I'm to ask you bear with me a bit here; I'm not going to go through each change I make as part of introducing this control flow in this post. The [`codeexamples`](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/5b0697df272b0c9ed7247a824517282cc0514504/codeexamples/2023-04-16-music-controls) demo has all of the changes for a working app again, and if you want to reference it along the way, you're more than welcome to. I would say that the diff you can see [here](https://github.com/quintenpalmer/quintenpalmer.github.io/compare/f9885b7..a104029) should be the most legible way to see what we're adding in this post. Ok with that as a dislaimer, and those as references, let's still get to writing some new code together.

### Add `sink` "Callback" Communication

There's a bit of a chicken-and-egg problem with adding the controls to talk to the sink and hear back what it has to say, without entangling them all into One Big Diff. I'm going to start with what we can hear back from the `sink` even though we won't be able to talk to it at first. I call this the "callback", which may be a bit of a misnomer. If enough people are unhappy with this name, I can change it. With that in mind, let's introduce some infrastructure to communicate with the `sink` from the `simplemusicplayback`.

#### **`src/gui/state.rs`**
```rust
use std::cell;
use std::sync::mpsc;

use crate::datastore;
use crate::shared;

pub struct State {
    pub page: Page,
    pub datastore: datastore::model::Library,
    // A holder for our sink state
    pub sink: Sink,
}

// ... old nav messages

// Said holding struct
pub struct Sink {
	// Our way to send messages to the sink
	// We won't do anything with this in our first pass, but stay tuned
    pub sink_message_sender: mpsc::Sender<shared::SinkMessage>,
    // Our way to hear back from the sink
    // We'll see why it's the RefCell<Option<...>> in a bit
    pub sink_callback_recv: cell::RefCell<Option<mpsc::Receiver<shared::SinkCallbackMessage>>>,
}
```

If you can trust me that the `RefCell<Option<...>>` will be needed for now, this is a pretty simple diff, just adding the `mpsc::Sender`/`mpsc::Receiver` we saw in from last post.

#### **`src/gui/message.rs`**
```rust
use crate::shared;

#[derive(Debug, Clone)]
pub enum Message {
    Nav(Navigate),
    // This contains the callback message that we will `recv`
    SinkCallback(shared::SinkCallbackMessage),
    // This error flow is to satisfy `iced`
    // We won't actually do anything with it that useful, but it is needed
    ErrorResponse(Result<(), String>),
}

// ... old navigation messages
```

Hopefully, a pretty manageable `message.rs` change as well; nothing too big yet.

#### **`src/gui/subscription.rs`**
```rust
use super::message;
use super::state;

// This will be hooked up in `impls.rs`
// But it takes the entire state, just for convenience
// And it returns a new-to-us `iced::Subscription`
// See the link in the paragraph below for documentation
pub fn sink_callback(app: &state::State) -> iced::Subscription<message::Message> {
	// We use this `unfold` function to keep taking the `recv` output
	// from our `sink_callback_recv` we added above, always returning
	// the message we find up for iced to propogate to our app for us
    iced::subscription::unfold(
        "sink message callback",
        // The lifetimes don't line up where we can get a reference to this
        // so we have to just .take() the RefCell's inner value,
        // which is the Option, that we then .unwrap() below.
        // We are definitely bending some rules here, if you know of a better way
        // I would definitely like to hear from you!
        app.sink.sink_callback_recv.take(),
        move |mut callback| async move {
            let msg = callback.as_mut().unwrap().recv().unwrap();
            (Some(message::Message::SinkCallback(msg)), callback)
        },
    )
}
```

See the documentation for [`Subscriptions`](https://docs.rs/iced/0.7.0/iced/subscription/type.Subscription.html) to see how they work; the basic idea is they are a way for the external world to get messages into the `Message` control-flow system that `iced` is built upon.

A reminder that the messages being passed up and re-wrapped are the shared messages:
```rust
#[derive(Clone, Debug)]
pub enum SinkCallbackMessage {
    Playing,
    Paused,
    SongEnded,
}
```

If you can trust that this [`unfold`](https://docs.rs/iced/0.7.0/iced/subscription/fn.unfold.html) will always relay the [re-wrapped](https://github.com/quintenpalmer/quintenpalmer.github.io/blob/787dbf8685eb277b4453f700ef1b5758e59fa8db/codeexamples/2023-04-16-music-controls/src/gui/message.rs#L8) [message](https://github.com/quintenpalmer/quintenpalmer.github.io/blob/787dbf8685eb277b4453f700ef1b5758e59fa8db/codeexamples/2023-03-26-music-playback/src/shared.rs#L9-L13) that it finds from the `mpsc::Receiver` that the `sink` [pushes](https://github.com/quintenpalmer/quintenpalmer.github.io/blob/787dbf8685eb277b4453f700ef1b5758e59fa8db/codeexamples/2023-03-26-music-playback/src/backend.rs#L85) [up](https://github.com/quintenpalmer/quintenpalmer.github.io/blob/787dbf8685eb277b4453f700ef1b5758e59fa8db/codeexamples/2023-03-26-music-playback/src/backend.rs#L93) [through](https://github.com/quintenpalmer/quintenpalmer.github.io/blob/787dbf8685eb277b4453f700ef1b5758e59fa8db/codeexamples/2023-03-26-music-playback/src/backend.rs#L105), then I think that's enough of an understanding of what's going on here. Let's see a skeleton of the update code operating on these new messages that we can now see.

#### **`src/gui/update.rs`**
```rust
use std::sync::mpsc;

use crate::shared;

use super::{message, state};

pub fn handle_message(
    state: &mut state::State,
    message: message::Message,
) -> iced::Command<message::Message> {
    println!("handling _a_ message...");
    match message {
        // ... old nav control flow

		// This calls into our new `recv`/callback handler
        message::Message::SinkCallback(callb) => {
            handle_sink_callback(state, callb);
            iced::Command::none()
        }
        // This calls into our new handler for errors
        message::Message::ErrorResponse(error_message) => {
            handle_error(state, error_message);
            iced::Command::none()
        }
    }
}

// ... old nav control flow

// We don't do anything useful yet, here, but we will soon!
fn handle_sink_callback(state: &mut state::State, callback_message: shared::SinkCallbackMessage) {
    match callback_message {
        shared::SinkCallbackMessage::Playing => {
            println!("we're now officially playing");
            // todo, actually do something with this knowledge
        }
        shared::SinkCallbackMessage::Paused => {
            println!("we're now paused");
            // todo, actually do something with this knowledge
        }
        shared::SinkCallbackMessage::SongEnded => {
            println!("the song has officially ended");
            // todo, actually do something with this knowledge
        }
    }
}

// This is as useful as this function will ever get
fn handle_error(_state: &mut state::State, error_message: Result<(), String>) {
    match error_message {
        Ok(()) => println!("no error was seen"),
        Err(err_string) => println!("We had seen this error: {}", err_string),
    }
}
```

This is a bigger code block, but nothing too intense is happening; it's all just kind of verbose. Let's get into the `impls.rs` and then we can start gluing more together.

#### **`src/gui/impls.rs`**
```rust
use std::cell;

use iced;

use crate::datastore;
use crate::sink;

use super::{message, state, subscription, update, view};

impl iced::Application for state::State {
    // ... omitted type declarations

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
	    // We start the sink and get its sender and receiver
        let (sink_sender, sink_recv) = sink::create_backend_with_client_and_callback();
        let state = state::State {
            page: state::Page::Home,
            datastore: datastore::model::Library::from_library_directory(".").unwrap(),
            // We build the sink state from the sink's sender and receiver
            sink: state::Sink {
                sink_message_sender: sink_sender,
                sink_callback_recv: cell::RefCell::new(Some(sink_recv)),
            },
        };
        (state, iced::Command::none())
    }

    // ... old title, update, and view functions

	// We hook up the subscription to relay the callback messages to our app
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::sink_callback(&self)
    }
}

```

Hopefully nothing too surprising here, this is mostly just glue code with our state and subscriptions now working. Let's see some actual control state that we change with our callback handling.

### Add Playback State

#### **`src/gui/state.rs`**
```rust
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

// ... old nav messages

// ... new sink messages

pub struct PlaybackInfo {
	// If there is a song that is being played back in any capacity,
	//   the Option will have Some value
	// If the song is currently playing,
	//   the second value in the tuple will be true
	// If it is paused,
	//  the bool will be false
    pub currently_playing: Option<(datastore::model::AudioFileTrackMetadata, bool)>,
}
```

We see here that the playback state is just the currently playing song, if any, represented with an `Option<(track, bool)>`. Let's fill these in within our `update.rs`

#### **`src/gui/update.rs`**
```rust
// ... imports 

// ... main handler unchanged

// ... old nav handler
//
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

// ... new error handler
```

We're just toggling the playing state, if there is a current track for play/pause messages, and we're unsetting the currently playing song entirely, when we learn that the track is over. Pretty easy!

### Send Messages to the Sink!

Ok, now we're ready to actually send messages to the sink! No new state beyond what we've added will be needed, just a new message, new update logic, and some view changes; buckle up.

#### **`src/gui/message.rs`**
```rust
use crate::datastore;
use crate::shared;

#[derive(Debug, Clone)]
pub enum Message {
    Nav(Navigate),
    Control(Control),
    // This isn't new this time around, just including for context
    SinkCallback(shared::SinkCallbackMessage),
    // Same with this variant
    ErrorResponse(Result<(), String>),
}

// ... old nav message

// This is the new message!
// It's very similar to the shared::SinkMessage
// but we do actually want the metadata on the track sometimes,
// so it's a new enum that just looks similar to the shared message
#[derive(Debug, Clone)]
pub enum Control {
    Play,
    Pause,
    PlayTrack(datastore::model::AudioFileTrackMetadata),
}

```

Just a new `Control` variant that let's us play/pause/load-a-song!

#### **`src/gui/update.rs`**
```rust
use std::sync::mpsc;

use crate::shared;

use super::{message, state};

pub fn handle_message(
    state: &mut state::State,
    message: message::Message,
) -> iced::Command<message::Message> {
    println!("handling _a_ message...");
    match message {
        // ... old nav control flow
		// ... new callback handler
        // ... new error handler
        message::Message::Control(control_message) => {
            println!("handling control message");
            handle_control(state, control_message)
        }
    }
}

// ... old nav control flow

// ... new callback handler

// ... new error handler

fn handle_control(
    state: &mut state::State,
    control_message: message::Control,
) -> iced::Command<message::Message> {
    match control_message {
	    // We just call into this message-sending-helper for play
        message::Control::Play => sink_message(
            state.sink.sink_message_sender.clone(),
            shared::SinkMessage::PlayButton,
        ),
        // We call into the same message-sending-helper for pause
        message::Control::Pause => sink_message(
            state.sink.sink_message_sender.clone(),
            shared::SinkMessage::PauseButton,
        ),
        // We record the currently-playing track in our playback state
        // and then call into the message-sending-helper
        message::Control::PlayTrack(track) => {
            state.playback.currently_playing = Some((track.clone(), true));
            sink_message(
                state.sink.sink_message_sender.clone(),
                shared::SinkMessage::LoadSong(track.full_path.to_string_lossy().to_string()),
            )
        }
    }
}

// This is just a helper function that calls into the real new logic below
fn sink_message(
    tx: mpsc::Sender<shared::SinkMessage>,
    message: shared::SinkMessage,
) -> iced::Command<message::Message> {
	// This perform function takes the async function
	// and the message to communicate any errors up to the iced app
	// as parameters
    iced::Command::perform(
        MessageCommandSender::new(tx, message).send_message(),
        message::Message::ErrorResponse,
    )
}

// This struct captures a copy of the mpsc::Sender
// and the message so it can operate on it
// as it communicates with the sink
struct MessageCommandSender<T> {
    tx: mpsc::Sender<T>,
    message: T,
}

impl<T: std::fmt::Debug> MessageCommandSender<T> {
	// Simple constructor
    fn new(tx: mpsc::Sender<T>, message: T) -> Self {
        MessageCommandSender {
            tx: tx,
            message: message,
        }
    }

	// This async function is where the real logic happens
	// but it's not too bad!
	// We just call the mpsc::Sender::send function
	// and return the error shape we saw in the beginning of the post
	// (which was () for OK and just a String for Err)
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
```

The big thing to document for all of this new logic is: [`iced::Command::perform`](https://docs.rs/iced/0.7.0/iced/struct.Command.html#method.perform), which is what lets us call into our thread-communicating functions without mucking up iced, trying to block as we send messages.

Again, if you can trust that this code sends the message we want down into the `sink`, going through the hoops needed to make `iced` happy, that should be good enough, as we wrap up here. If you want to dig in though, it is cool to see everything line up. Let's just add some buttons to actually make any of this control and callback logic happen!

#### **`src/gui/view/album.rs`**
```rust
// ... omitting imports

pub fn view_album_track_list<'a>(
    artist_name: String,
    album_name: String,
    datastore: &'a datastore::model::Library,
) -> (
    iced::Element<'a, message::Message>,
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    // ... omitting leading logic
        for track in disc.tracks.values() {
            let track_row =
                Row::new()
                    .spacing(10)
                    // This is the only new thing, we add a button
                    // that emits the message::Control::Playtrack(...)
                    // for this given track
                    .push(button(">").on_press(message::Message::Control(
                        message::Control::PlayTrack(track.clone()),
                    )))
                    .push(text(format!("{:>3}", track.track.unwrap_or(1))).size(26))
                    .push(text(track.track_title.clone()).size(26));
            tracks_column = tracks_column.push(track_row);
        }
    // ... omitting trailing logic as well
}
```

That's how we can now play songs! And this is how we can see what's playing, and play/pause:

#### **`src/gui/view/mod.rs`**
```rust
use iced;
use iced::widget::{button, text, Column, Row, Scrollable};

use crate::datastore;

use super::{message, state};

// .. omitting module declaration

pub fn view_state<'a>(state: &'a state::State) -> iced::Element<'a, message::Message> {

    // ... omitting body and breadcrumb construction

	// Build the playback info
    let playback_info = view_playback_info(&state.playback);

    let mut ret = Column::new();

    ret = ret.push(crumb_button_row);
    // Of note: fill the body so that it always takes up as much
    // vertical space as possible
    ret = ret.push(Row::new().push(body).height(iced::Length::Fill));
    // If there is any currently playing track, include it at the bottom
    match playback_info {
        Some(actual_playback_info) => ret = ret.push(actual_playback_info),
        None => (),
    };
    ret.into()
}

// ... page view function

// This is our new function
fn view_playback_info<'a>(
    playback: &'a state::PlaybackInfo,
) -> Option<iced::Element<'a, message::Message>> {
    match playback.currently_playing {
        Some((ref track, playing)) => {
	        // If there is a track playing, make a new 10-unit tall bar
            let mut row = Row::new().spacing(10);
            // If the track is currently playing
            if playing {
	            // Create an ascii pause button that will pause playback
                row = row.push(
                    button("||").on_press(message::Message::Control(message::Control::Pause)),
                );
            // If the track is not currently playing
            } else {
	            // Create an ascii play button that will resume playback
                row = row
                    .push(button(" >").on_press(message::Message::Control(message::Control::Play)));
            }
            // Always add the track's title
            row = row.push(text(track.track_title.clone()));
            Some(row.into())
        }
        None => None,
    }
}

```

And with all of that we now have an app that will play back our track and we can play and pause the tracks! Let's see it in action with some screenshots.

## App In Action

Here's a quick demo (already navigated to the view for this album), where we just click play on the first track, let it play for a bit, pause it, resume playback, and then let it finish (note that the bottom playback bar disappears just after the song has finished, without any user input).

![Playing Back Media](/assets/2023-04-16/controls.gif)

## Uploaded Code Example

I know I already linked to it in the beginning, but I will just post it again here, the code that this post is based-on can be found [here](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/787dbf8685eb277b4453f700ef1b5758e59fa8db/codeexamples/2023-04-16-music-controls) and if you want to see a diff of mostly-the-important changes from the copy-pasted old GUI, [this diff](https://github.com/quintenpalmer/quintenpalmer.github.io/compare/f9885b7..a104029) is a good reference.

# Conclusion

Ok, that was a heavy one, sorry if it got too much and you're reading this after giving up on all of the nitty gritty. Also apologies for this being a week late. Hopefully you either found something useful in here, or were happy with how it all came together. I really have been impressed with Iced as I've used it more. I really like the core Elm-inspired model, and the way Iced gives you controls like with the `Subscription` make it feel like you can build just about anything with it. Anyways, tune in next time, and I'll either explore how to interface with [`MPRIS`](https://specifications.freedesktop.org/mpris-spec/latest/) or maybe take another week off of the technical content; we'll see. Until then, take care!