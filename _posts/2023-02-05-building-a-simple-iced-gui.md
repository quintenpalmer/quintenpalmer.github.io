---
title: Building a Simple Iced GUI
date: 2023-02-05
---

# Welcome (Back)

Hello, I'm writing a series of tech blogs this year and this is the 4th installment of that [series](/2023/01/15/introducing-my-musiq-player-and-blog-series) [of](/2023/01/22/learning-audio-metadata-with-ffmpeg) [blogs](/2023/01/29/parsing-audio-files-with-rust). You don't need to read them all first, but you are definitely welcome to (either before or circling back after). I would recommend the [parsing audio files with rust](/2023/01/29/parsing-audio-files-with-rust) post just to get an idea of the shape of the data our "datastore" layer provides. I will not be explaining that heavily in this post; if you're ok just trusting the shape of that data to make sense, you can probably skip all previous posts.

In this post we are going to build a simple [Iced](https://docs.rs/iced/latest/iced) application to get a rudimentary view of a music library (by library we mean collection of songs, just to be clear). Not too much to introduce this time; let's dig in!

# Introducing Iced

Before we dive into writing this app, let's get a taste of how Iced works and what it provides. Feel free to read through Iced's [Overview](https://docs.rs/iced/0.7.0/iced/index.html#overview) before or after reading my summary.

## In Iced's Own Words

Starting with a quote from Iced's aforementioned [Overview](https://docs.rs/iced/0.7.0/iced/index.html#overview):

Inspired by [The Elm Architecture](https://guide.elm-lang.org/architecture/), Iced expects you to split user interfaces into four different concepts:
-   **State** — the state of your application
-   **Messages** — user interactions or meaningful events that you care about
-   **View Logic** — a way to display your **State** as widgets that may produce **Messages** on user interaction
-   **Update Logic** — a way to react to **Messages** and update your **State**

## Reworking Iced's Example

Folding their example code into a working example in one code block:

#### **`main.rs`**
```rust
use iced::widget::{button, column, text};
use iced::Sandbox;

// This represents the State of our application:
struct CounterState {
    // The value of our counter
    value: i32,
}

// This are our Message
// They represent the possible interactions the user can take: incrementing and decrementing the counter (as we'll see, through button presses)
#[derive(Debug, Clone, Copy)]
pub enum CounterMessage {
    IncrementPressed,
    DecrementPressed,
}

// We are implementing the Sandbox trait in this example (see documentation for what this entails)
impl iced::Sandbox for CounterState {
    type Message = CounterMessage;

    // We need a `new` for Iced to know what State to start with
    fn new() -> Self {
        CounterState { value: 1 }
    }

    // We also need to provide a "title" for our app
    fn title(&self) -> String {
        "Counter App".to_string()
    }

    // The Update Logic is where we accept messages to update the State
    fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::IncrementPressed => {
                self.value += 1;
            }
            CounterMessage::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    // The View Logic displays our State and the other components the user can use to interact with our app
    fn view(&self) -> iced::Element<CounterMessage> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("+").on_press(CounterMessage::IncrementPressed),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("-").on_press(CounterMessage::DecrementPressed),
        ]
        .into()
    }
}

fn main() {
    CounterState::run(iced::Settings::default()).unwrap();
}
```

As mentioned, here is the [`Sandbox`](https://docs.rs/iced/latest/iced/trait.Sandbox.html) trait.

If you run this you'll get a small window that looks like the following, allowing you to click to increment and decrement the counter:

![Animated Counter Gif](/assets/2023-02-05/animated_counter.gif)

This is also available as a demo you can run [here](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/main/codeexamples/2023-02-05-music-gui).

## Additional Iced (And Related) Resources

Hopefully this way of structuring an interactive app makes sense; if not I can definitely augment this section or feel free to browse more documentation on the subject:

* [Elm](https://guide.elm-lang.org/architecture/)
* [Redux](https://redux.js.org/introduction/core-concepts)
* [Iced](https://docs.rs/iced/latest/iced/#overview)

With the Iced architecture under our belt, let's try to actually build something slightly more complex than this toy counter example!

# Building Our Iced GUI

## Initial Code

To start, let's build a rust project and immediately add `iced` as a dependency:

```bash
cargo init --name simplemusicgui
```

and then we'll modify our `Cargo.toml`

### **`Cargo.toml`**
```toml
[package]
name = "simplemusicgui"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# We're adding iced as a dependency here
iced = "0.7"

# And we're going to add our audio parsing library from last time here
simpleaudioparser = { path = "../2023-01-29-parsing" }
```

Note that we also added the `simpleaudioparser` that we built in the last blog post; we will use this to scan for and parse actual audio files with this demo.

And now, let's build our initial skeleton of files:

```bash
 $ tree
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

Alright, and now to actually start fleshing out some code for this demo app. We already went over the `Cargo.toml` and the `Cargo.lock` is not for us to edit. Let's get this working!

## Write All The Code

Let's start with the `src/main.rs` and work through these files to build a working app:

### **`src/main.rs`**
```rust
// We'll be treating the `simpleaudioparser` as a datastore
// so this is just a "rename" to let us refer to it as `datastore` in this crate
pub use simpleaudioparser as datastore;

// This is just a module to hold all of our gui logic (which is all of the logic, but just to make it extra clear)
mod gui;

fn main() {
    // We'll come back to this at the end
    println!("TODO: actually run the app");
}

```

Pretty simple `main.rs` at this point, the only fancy thing is the `pub use`; more documentation can be found [here](https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility). `src/gui/mod.rs` is next, which will also be pretty boring:

### **`src/gui/mod.rs`**
```rust
// This will hold the app state and we need to export it so that `main.rs` can access it
pub mod state;

// These will be all of the messages that we pass from the UI to the update function
mod message;
// This will be that update logic that takes messages and changes the state of the app
mod update;
// This will be the view function that displays the state to the user
mod view;

// This is where we will implement iced's trait to actually have an app to run
// This also needs to be public so that external users of the app state can see the trait satisfaction
pub mod impls;
```

Also, pretty simple, hopefully the comments explain why we have so many files and what responsibility the functions in each file will have. Now for the state!

### **`src/gui/state.rs`**
```rust
use crate::datastore;

// It's our app's whole state!
pub struct State {
    // This page field represents which "page" we're looking at
    // By page, I mean like a web page
    // (there very well could be a better term for this)
    pub page: Page,
    // The datastore/Library has all of the music info we will use
    pub datastore: datastore::model::Library,
}

// These are the web-page-like Pages
pub enum Page {
    // This is like index.html, to use more web parlance
    Home,
    // This is the page to list and view all artists in the Library
    ArtistList,
    // This page is to look at all albums for a given artist
    // The single `String` field is the artist name
    ArtistAlbumList(String),
    // This page is to look at all of the tracks for the artist's album
    // The first `String` is the artist name and the second `String` is the album name
    AlbumTrackList(String, String),
}

```

Pretty simple state, all things considered (I think). We have an enum for the "Page" the user is looking at, which holds any relevant information for each page. And then we have the datastore/library so we can always look up any artist/album information we'll need for any given views in the future. Ok, now the messages:

### **`src/gui/message.rs`**
```rust
#[derive(Debug, Clone)]
pub enum Message {
    // We only have one kind of message: a navigation message
    // I still have it in its own enum to leave room for more in the future
    // and to make it extra clear that those messages are to navigate
    Nav(Navigate),
}

#[derive(Debug, Clone)]
pub enum Navigate {
    // This is to navigate to the home/index.html page
    Home,
    // This indicates to navigate to the artist list page
    // Note that it requires no parameters, it's to list all artists in the library
    ArtistList,
    // This is to navigate to the artist page
    // which currently is only to list the artist's albums
    // The `String` is the artist name, again
    ArtistAlbumList(String),
    // This is to navigate to the album page
    // which currently just lists tracks
    // The `String`s are the artist name and album name, respectively
    AlbumTrackList(String, String),
}
```

There is obviously a _very_ strong parallel between the navigation messages and the page states. Excruciatingly parallel, it makes me want to be able to DRY them up or something, but they are conceptually different constructs, so they are defined once over in each location. Ok, now on to some actual functions: update!

### **`src/gui/update.rs`**
```rust
use super::{message, state};

// This signature will be to match the signature we will need to satisfy iced's trait
// We need a mutable reference to the state so we can actually update it
// And we got a whole copy of the message to key off of
pub fn handle_message(state: &mut state::State, message: message::Message) {
    match message {
        // This parent function is rather boring
        // as the only kind of messages are nav messages
        message::Message::Nav(nav_message) => handle_nav(state, nav_message),
    }
}

// This will key off of the navigation message and update our state accordingly
fn handle_nav(state: &mut state::State, nav_message: message::Navigate) {
    match nav_message {
        // A home nav message means to view the home page
        message::Navigate::Home => state.page = state::Page::Home,
        // An artist list nav message means to view the artist list page
        message::Navigate::ArtistList => state.page = state::Page::ArtistList,
        // A specific artist nav message means to view the specific artist page
        message::Navigate::ArtistAlbumList(artist_name) => {
            state.page = state::Page::ArtistAlbumList(artist_name)
        }
        // A specific album nav message means to view the specific album page
        message::Navigate::AlbumTrackList(artist_name, album_name) => {
            state.page = state::Page::AlbumTrackList(artist_name, album_name)
        }
    }
}
```

Hopefully each of these boring pieces start to make sense as they are all adding up together. Next we're going to look at the view logic, but we're going to leave that as a TODO as we then just glue it all together with the trait implementation, then circle back to the full view logic.

### **`src/gui/view/mod.rs`**
```rust
use iced;
use iced::widget::text;

use super::{message, state};

// We get an immutable reference to the state
// as we don't need to modify it from our view function
// And we return an iced Element, which is the generic widget type
pub fn view_state<'a>(state: &'a state::State) -> iced::Element<'a, message::Message> {
    // We'll fill this in soon
    // Currently we ignore all app state and just always show this todo message
    iced::text("TODO: fill me in").into()
}
```

And gluing this together, the implementation of Iced's [`Sandbox`](https://docs.rs/iced/0.7.0/iced/trait.Sandbox.html) trait:

### **`src/gui/impls.rs`**
```rust
use iced;

use crate::datastore;

use super::{message, state, update, view};

// I'll link to documentation below for the Sandbox trait
impl iced::Sandbox for state::State {
    // We need to tell iced what our message type
    type Message = message::Message;

    // This is how iced will start our app state
    fn new() -> Self {
        state::State {
            // We start on the home page
            page: state::Page::Home,
            // And with a library loaded from the current directory
            datastore: datastore::model::Library::from_library_directory(".").unwrap(),
        }
    }

    // A simple title for our simple app
    fn title(&self) -> String {
        "Simple Music Viewer".to_string()
    }

    // We call into the update function we wrote earlier here
    fn update(&mut self, message: message::Message) {
        update::handle_message(self, message)
    }

    // We call into the view function we wrote earlier here too
    fn view(&self) -> iced::Element<message::Message> {
        view::view_state(self)
    }
}
```

And our app would technicall run now! See the ["Hello World"](https://docs.rs/iced/0.7.0/iced/trait.Sandbox.html#a-simple-hello-world) for the Iced `Sandbox` trait for an even simpler satisfaction of this trait.

It would admittedly look very boring as our view is always the same `"TODO"` text, so let's fill in the view logic and then take a look at what we get:

### **`src/gui/view/mod.rs`**
```rust
use iced;
use iced::widget::{button, Column, Row, Scrollable};

use crate::datastore;

use super::{message, state};

// We have separate modules for each page's view, just for organization
mod album;
mod artistalbums;
mod artists;
mod home;

pub fn view_state<'a>(state: &'a state::State) -> iced::Element<'a, message::Message> {
    // We first get the view for the page, and the breadcrumbs for that page
    let (body, breadcrumbs) = view_page(&state.page, &state.datastore);

    // These breadcrumbs help the user navigating back up as they navigate around
    // We always have the home breadcrumb
    let mut crumb_button_row = Row::new()
        .spacing(10)
        .push(button("Home").on_press(message::Message::Nav(message::Navigate::Home)));

    // And then we add all breadcrumbs for the page
    for crumb_button in breadcrumbs.into_iter() {
        crumb_button_row = crumb_button_row.push(Scrollable::new(crumb_button));
    }

    // The final return is a column with the breadcrumbs, and the body for the page we are viewing
    Column::new().push(crumb_button_row).push(body).into()
}

fn view_page<'a>(
    page: &'a state::Page,
    datastore: &'a datastore::model::Library,
) -> (
    // This return is the actual view for the page
    iced::Element<'a, message::Message>,
    // This return is the list of breadcrumbs for said page
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    // Without annotating each match, each page has their own view
    match page {
        state::Page::Home => home::view_home(),
        state::Page::ArtistList => artists::view_artist_list(&datastore),
        state::Page::ArtistAlbumList(ref artist_name) => {
            artistalbums::view_artist_album_list(artist_name.clone(), &datastore)
        }
        state::Page::AlbumTrackList(ref artist_name, ref album_name) => {
            album::view_album_track_list(artist_name.clone(), album_name.clone(), &datastore)
        }
    }
}

```

Ok, for each individual view, I'm going to skip heavy annotations and you can skim or skip through this next section; just leaving it here for anyone who does want to see it all without skipping over to the code example:

### **`src/gui/view/home.rs`**
```rust
use iced;
use iced::widget::{button, text, Column};

use super::super::message;

pub fn view_home<'a>() -> (
    iced::Element<'a, message::Message>,
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    (
        Column::new()
            .padding(10)
            .push(text("Welcome").size(46))
            // We "link" to the artist list page from the home page
            .push(button("Artists").on_press(message::Message::Nav(message::Navigate::ArtistList)))
            .into(),
        // We have no additional breadcrumbs from the home page
        Vec::new(),
    )
}
```

### **`src/gui/view/artists.rs`**
```rust
use iced;
use iced::widget::{button, text, Column, Scrollable};

use crate::datastore;

use super::super::message;

pub fn view_artist_list<'a>(
    datastore: &'a datastore::model::Library,
) -> (
    iced::Element<'a, message::Message>,
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    // Link to this same page as a breadcrumb
    let breadcrumbs =
        vec![button("Artists").on_press(message::Message::Nav(message::Navigate::ArtistList))];

    let mut artist_list_column = Column::new();
    for artist_name in datastore.artists.keys() {
        // Link to each artist with a button that emits the artist list message on-press
        artist_list_column = artist_list_column.push(button(text(artist_name.clone())).on_press(
            message::Message::Nav(message::Navigate::ArtistAlbumList(artist_name.clone())),
        ))
    }

    (
        Column::new()
            .padding(10)
            .push(text("Artists:").size(46))
            .push(Scrollable::new(artist_list_column))
            .into(),
        breadcrumbs,
    )
}

```

### **`src/gui/view/artistalbums.rs`**
```rust
use iced;
use iced::widget::{button, text, Column, Row, Scrollable};

use crate::datastore;

use super::super::message;

pub fn view_artist_album_list<'a>(
    artist_name: String,
    datastore: &'a datastore::model::Library,
) -> (
    iced::Element<'a, message::Message>,
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    // Include the artist list and a link to this artist's page as the breadcrumbs
    let breadcrumbs = vec![
        button("Artists").on_press(message::Message::Nav(message::Navigate::ArtistList)),
        button(text(artist_name.clone())).on_press(message::Message::Nav(
            message::Navigate::ArtistAlbumList(artist_name.clone()),
        )),
    ];

    let mut albums_column = Column::new().padding(10);
    for album_name in datastore.artists.get(&artist_name).unwrap().albums.keys() {
        // Link to each of this artist's albums with buttons
        albums_column = albums_column.push(button(text(album_name.clone()).size(26)).on_press(
            message::Message::Nav(message::Navigate::AlbumTrackList(
                artist_name.clone(),
                album_name.clone(),
            )),
        ));
    }

    (
        Column::new()
            .padding(10)
            .push(
                Row::new()
                    .push(text(artist_name).size(46))
                    .push(text("(Artist)").size(26)),
            )
            .push(text("Albums:").size(36))
            .push(Scrollable::new(albums_column))
            .into(),
        breadcrumbs,
    )
}
```

### **`src/gui/view/album.rs`**
```rust
use iced;
use iced::widget::{button, text, Column, Row, Scrollable, Space};

use crate::datastore;

use super::message;

pub fn view_album_track_list<'a>(
    artist_name: String,
    album_name: String,
    datastore: &'a datastore::model::Library,
) -> (
    iced::Element<'a, message::Message>,
    Vec<iced::widget::Button<'a, message::Message>>,
) {
    // Link to the artist list page, specific artist page, and this album in the breadcrumbs
    let breadcrumbs = vec![
        button("Artists").on_press(message::Message::Nav(message::Navigate::ArtistList)),
        button(text(artist_name.clone())).on_press(message::Message::Nav(
            message::Navigate::ArtistAlbumList(artist_name.clone()),
        )),
        button(text(album_name.clone())).on_press(message::Message::Nav(
            message::Navigate::AlbumTrackList(artist_name.clone(), album_name.clone()),
        )),
    ];

    let mut discs_column = Column::new().padding(10);
    for disc in datastore
        .artists
        .get(&artist_name)
        .unwrap()
        .albums
        .get(&album_name)
        .unwrap()
        .discs
        .values()
    {
        let mut tracks_column = Column::new().padding(10);
        for track in disc.tracks.values() {
            // Include the track number and title information for each track
            let track_row = Row::new()
                .spacing(10)
                .push(text(format!("{:>3}", track.track.unwrap_or(1))).size(26))
                .push(text(track.track_title.clone()).size(26));
            tracks_column = tracks_column.push(track_row);
        }
        // Include separate sections for each disc
        discs_column = discs_column
            .push(text(format!("Disc: {}", disc.number)))
            .push(tracks_column);
    }

    (
        Column::new()
            .padding(10)
            .push(
                Row::new()
                    .push(text(album_name).size(46))
                    .push(text("(Album)").size(26)),
            )
            .push(
                Row::new()
                    .push(Space::with_width(iced::Length::Units(50)))
                    .push(text(artist_name).size(36))
                    .push(text("(Artist)").size(26)),
            )
            .push(text("Tracks:").size(36))
            .push(Scrollable::new(discs_column))
            .into(),
        breadcrumbs,
    )
}

```

Ok! Ok! Now we have an app we can actually run and can do relatively uesful things! Let's see what it looks like:

## Running Our GUI

With a simple `cargo run` we can see what this GUI looks like! Here is an animated gif of it:

![Animated gif of Music GUI](/assets/2023-02-05/music_gui/music_gui_animated.gif)

And if you [click here](/2023/02/05/extra-music-gui-screenshots) you can see all screenshots laid out and annotated.

# Conclusion

Hopefully this (relatively) simple app gives you an idea of what to expect with my real Musiq app that I will soon reveal to the world. This post definitely felt like more of a code-dump than I was expecting; sorry if it was either too dense or too dry. Stay tuned for the next post, where I think I'm planning to show audio playback.
