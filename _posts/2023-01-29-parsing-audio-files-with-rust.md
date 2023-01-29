---
title: Parsing Audio Files with Rust
date: 2023-01-29
---

# Establishing Prerequisites

## Audio Metadata

This is a follow-up to my [previous blog post](/2023/01/22/learning-audio-metadata-with-ffmpeg) establishing the schema and structure of audio file metadata. If you already know all about that, you're ready to go! But if not, that is probably worth a read (or at least a skim) before starting in on this post.

## Rust

Going forward, most of the content will be about my MusiqApp project, written in [Rust](https://www.rust-lang.org). If you want to fully understand the mechanisms of what's going on, you will probably want to know Rust, but if you want to treat it as a sort of pseudo-code, it should mostly be legible as that. Ok, with those established, let's get into it!

# Let's Write Some Rust!

## Establish a Skeleton

Let's start with the simplest possible Rust project:

```bash
cargo init --name simpleaudioparser
```

which leaves us with:

```bash
 $ tree
.
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files

 $ cat Cargo.toml
[package]
name = "simpleaudioparser"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

 $ cat src/main.rs
fn main() {
    println!("Hello, world!");
}
```

And then let's add a skeleton of our module structure. This may look like a scary amount of code, but it's mostly just laying out type signatures, that we'll explain along the way.

The files we will start with will be:

### **`main.rs`**
```rust
mod util;
mod parse;
mod scan;
mod organize;
pub mod model;
pub mod impls;

fn main() {
    println!("Hello, world!");
}
```

This declares 6 the modules that we'll fill in (with implementation left as todos for now) and leaves the default hello world main; we'll come back to that too. The `model` and `impls` modules are public as they will define the structure and methods we would want to expose outside of this crate. Let's move next to the `model.rs` module.

### **`model.rs`**
```rust
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Error {}

pub struct Library {
    pub artists: BTreeMap<String, Artist>,
}

pub struct Artist {
    pub name: String,
    pub albums: BTreeMap<String, Album>,
}

pub struct Album {
    pub name: String,
    pub discs: BTreeMap<u32, Disc>,
}

pub struct Disc {
    pub number: u32,
    pub tracks: BTreeMap<u32, AudioFileTrackMetadata>,
}

pub struct AudioFileTrackMetadata {
    pub artist: String,
    pub album_artist: Option<String>,
    pub album: Option<String>,
    pub disc_no: Option<u32>,
    pub disc_total: Option<u32>,
    pub track: Option<u32>,
    pub track_total: Option<u32>,
    pub track_title: String,
    pub genre: Option<String>,
    pub date: Option<String>,
}
```

The `Error` enum will be filled in as we introduce error cases we wish to capture.

More usefully, this defines a structure of a `Library` which has a collection of `Artist` entries, each with a collection of `Album` entries, then with a collection of `Disc` values, each with a collection of these `AudioFileTrackMetadata` values.

Note in the `AudioFileTrackMetadata` that the only values that are not `Option`-al are the `artist` and `track_title`. We will only ever assume that those values will be present, and the rest can be empty, and we'll just assign the `Option::None` when there is no value present.

### **`scan.rs`**
```rust
use std::path;

use crate::model;

pub fn find_audio_files(scan_path: &path::PathBuf) -> Result<Vec<path::PathBuf>, model::Error> {
    todo!("teach me how to find audio files")
}
```

This is our first real function, sweet! It takes a reference to a [`path::PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html) and then it will be responsible for trying to build a [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) of the `path::PathBuf` values for the files that are audio files that we know how to parse. The `Result<..., model::Error>` of the return captures that we realize that this function could fail for various reasons.

### **`parse.rs`**
```rust
use std::path;

use crate::model;

pub fn parse_all_audio_files(
    paths: Vec<path::PathBuf>,
) -> Result<Vec<model::AudioFileTrackMetadata>, model::Error> {
    paths
        .into_iter()
        .map(|audio_file_path| parse_single_audio_file(audio_file_path))
        .collect()
}

fn parse_single_audio_file(
    audio_file_path: path::PathBuf,
) -> Result<model::AudioFileTrackMetadata, model::Error> {
    todo!("teach me how to parse a single file")
}
```

These two functions just split out the responsibility of parsing one file and then trying to parse a collection of files.

`parse_all_audio_files` uses Rust's [iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) to iterate over each filename passed in and call `parse_single_audio_file` on each entry. Note that it's even able to `.collect()` over a collection of `Result<_, model::Error>` and return a single `Result<Vec<_>, model::Error>` instead of the simpler collection, would be a `Vec<Result<_, model::Error>>`, as documented [here](https://doc.rust-lang.org/std/result/#collecting-into-result). This transformation is still cool to me!

`parse_single_audio_file` takes a single path and will parse and return the "canonical" `AudioFileTrackMetadata` for this file, if possible (the fallible nature captured in the `Result<_, model::Error>` again).

### **`organize.rs`**
```rust
use crate::model;

pub fn organize_tracks(
    tracks: Vec<model::AudioFileTrackMetadata>,
) -> Result<model::Library, model::Error> {
    todo!("teach me how to organize tracks")
}
```

This function will take a list of `AudioFileTrackMetadata` values and try to organize them into a `Library` (again, again, returning the `model::Error` if it fails for any reason). Note that we're done dealing with the actual files and just operate on the canonical `model::AudioFileTrackMetadata` now.

### **`impls.rs`**
```rust
use std::path;

use crate::{model, organize, parse, scan};

impl model::Library {
    pub fn from_library_directory<P: AsRef<path::Path>>(
        library_directory: P,
    ) -> Result<Self, model::Error> {
        let audio_file_paths = scan::find_audio_files(&library_directory.as_ref().to_path_buf())?;

        let audio_file_track_metadata_entries = parse::parse_all_audio_files(audio_file_paths)?;

        let library = organize::organize_tracks(audio_file_track_metadata_entries)?;

        Ok(library)
    }
}
```

I'm introducing this method/function fully fleshed out, as it really just calls the other three main functions and gives a nice API to our modules with `model::Library::from_library_directory(...)`. And just one last module, a simple `util`:

### **`util.rs`**
```rust
pub fn get_maybe_extension_string(p: &path::PathBuf) -> Option<String> {
    match p.extension() {
        Some(v) => Some(v.to_str().unwrap().to_lowercase()),
        None => None,
    }
}
```

This takes a reference to a `path::PathBuf` and returns a lowercase representation of the extension, if it exists. We'll use it soon as we start to fill in the bodies of these functions. Speaking of which, let's start doing so, first with the `scan::find_audio_files` function.

## Fill In the Scan

Let's start filling in the implementation of the least audio-file-specific, the scan.

### Prepare `model::Error`

Let's make it so that an `Error` can hold `io::Error` values, which may happen while we're scanning:

#### **`model.rs`**
```rust
// -- omitting all prior content

#[derive(Debug)]
pub enum Error {
    // This is our new io error variant
    IO(io::Error),
}

// And we can now always convert from an io error into our
// custom error
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

// -- omitting all other content
```

With this possible, let's actually fill in `find_audio_files`:

### Implementing `find_audio_files`

There is a decent amount going on here; I've included inline comments to help describe each step.

#### **`scan.rs`**
```rust
use std::fs;
use std::path;

use crate::{model, util};

pub fn find_audio_files(scan_path: &path::PathBuf) -> Result<Vec<path::PathBuf>, model::Error> {
    // Build the vec of file paths that are deemed
    // matching audio files.
    let mut audio_files = Vec::new();

    // Read all values in the directory of the passed-in PathBuf
    for child_entry in fs::read_dir(scan_path)? {
        // The iterator is over `Result<fs::DirEntry, io::Error>` so we "try" it with the ? syntax
        let child_entry = child_entry?;
        // Save this value just for brevity in the future
        let child_path = child_entry.path();

        // If this entry is a directory,
        // then recursively call this same function with this child entry as the `scan_path`
        // and add those audio files to our return value
        if child_entry.file_type()?.is_dir() {
            audio_files.append(&mut find_audio_files(&child_path)?);
        }
        // If the entry is a regular file, then let's see
        // if it's an audio file we want to process
        if child_entry.file_type()?.is_file() {
            // Grab the extention (if present)
            let maybe_extension = util::get_maybe_extension_string(&child_path);

            match maybe_extension {
                // If there is an extension, match on the string
                // as a &str so we can use string literals in our matches
                Some(extension) => match extension.as_str() {
                    // We're going to start with only flac files
                    "flac" => audio_files.push(child_path),
                    // Debug that we're skipping a file with an extension we don't know about.
                    _ => println!(
                        "DEBUG: Skipping file with unknown extension: {}",
                        child_path.to_string_lossy()
                    ),
                },
                // Debug that we're skipping a file with no extension
                None => println!(
                    "DEBUG: skipping file with no extension: {}",
                    child_path.to_string_lossy()
                ),
            }
        }
    }

    // Return the files we found that we saw were "audio" files
    Ok(audio_files)
}
```

Ok, that's a decent amount of code, but hopefully the inline comments help. Here is some documentation to help on top of that:
* [`?`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)
	* The ? operator is the preferred method of error propagation in Rust
	* Note that, since we have `impl From<io::Error> for model::Error` we can automatically convert from an `io::Error` to our `model::Error` 
* [`fs::read_dir`](https://doc.rust-lang.org/std/fs/fn.read_dir.html)
	* This documents how it returns a `Result<fs::ReadDir, io::Error>`
* [`impl Iterator for fs::ReadDir`](https://doc.rust-lang.org/std/fs/struct.ReadDir.html#impl-Iterator-for-ReadDir)
	* This documents that the iterator is over (note that it's another `Result`) a `Result<fs::DirEntry, io::Error>`
* [`fs::DirEntry`](https://doc.rust-lang.org/std/fs/struct.DirEntry.html)
	* This has the methods we use:
		* [`fs::DirEntry.path()`](https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path)
		* [`fs::DirEntry.file_type()`](https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_type)

I'll let you poke around the rest of the documentation. The ultimate purpose of this function is to recursively walk through a directory and find all of the `*.flac` files, which is what we want! On to the parse functions:

## Fill In the Parser

### Prepare `Cargo.toml` and `model.rs`

First, we'll need to include the [`claxon`](https://docs.rs/claxon/latest/claxon/) crate, which will do the FLAC parsing for us.

#### **`Cargo.toml`**
```toml
# -- omitting package information

[dependencies]
claxon = "0.4"
```

We'll also need to augment the `model::Error` type for some new errors we'll see/generate:

#### **`model.rs`**
```rust
// -- omitting other `use` imports

use claxon;

pub enum Error {
    IO(io::Error),
    // This is our new claxon error
    Claxon(claxon::Error),
    // This will be returned if expected metadata is missing
    // This holds the path to the file and the key name 
    MissingMetadataKey(String, &'static str),
    // This will be returned if we cannot
    // parse a value as a u32
    // This holds the path to the file and the key name 
    ExpectedU32MetadataValue(String, &'static str),
}

// -- omitting IO impl

// And we can now always convert from a claxon error into our
// custom error
impl From<claxon::Error> for Error {
    fn from(e: claxon::Error) -> Self {
        Error::Claxon(e)
    }
}
```

Next, let's work on `parse.rs` and split the file on extension (again) and dispatch off to a FLAC parser if it's a `.flac` file:

### Implementing `parse_single_audio_file`

#### **`parse.rs`**
```rust
use std::path;

use crate::{model, util};

// -- omitting `parse_all_audio_files`

pub fn parse_single_audio_file(
    audio_file_path: path::PathBuf,
) -> Result<model::AudioFileTrackMetadata, model::Error> {
    // Get the potential extension
    let maybe_extension = util::get_maybe_extension_string(&audio_file_path);

    match maybe_extension {
        Some(extension) => match extension.as_str() {
            // If there is an extension, and it is a `.flac` file
            // Let's call into our new `flac::parse_flac_file`
            "flac" => flac::parse_flac_file(audio_file_path),
            _ => panic!("unknown audio file extension"),
        },
        None => panic!("file without extension"),
    }
}
```

And now let's introduce the actual FLAC parsing:

#### **`parse.rs`**
```rust
// This module is for our flac parsing (we could add parsers for other file types in the future)
mod flac {
    use std::collections::BTreeMap;
    use std::path;

    use crate::model;

    pub fn parse_flac_file(
        path: path::PathBuf,
    ) -> Result<model::AudioFileTrackMetadata, model::Error> {
        // First, let's pass this known-to-be-flac file to `claxon`
        let reader = claxon::FlacReader::open(&path)?;

        // Claxon's reader can parse all of the tags for us,
        // and we'll transform them into a map of Strings to Strings,
        // with the key lowercased, for simplicity of lookup
        let tag_map = reader
            .tags()
            .map(|(k, v)| (k.to_string().to_lowercase(), v.to_string()))
            .collect::<BTreeMap<String, String>>();

        // This is where we then look up for all of the keys we hope to find
        // We do most of the lookups into our map with three helper functions

        // Note: "artist" and "title" are the only keys we require, the rest may or may not be set
        // If one of the disc or track values are not numbers, then we will error out
        Ok(model::AudioFileTrackMetadata {
            artist: get_string_result(&tag_map, "artist", &path)?,
            album_artist: get_string_option(&tag_map, "albumartist"),
            album: get_string_option(&tag_map, "album"),
            disc_no: get_u32_optional_result(&tag_map, "discnumber", &path)?,
            disc_total: get_u32_optional_result(&tag_map, "disctotal", &path)?,
            track: get_u32_optional_result(&tag_map, "tracknumber", &path)?,
            track_total: get_u32_optional_result(&tag_map, "tracktotal", &path)?,
            track_title: get_string_result(&tag_map, "title", &path)?,
            genre: get_string_option(&tag_map, "genre"),
            date: get_string_option(&tag_map, "date"),
        })
    }

    // This retrieves a String, if it is present
    // and returns `Option::None` if not
    fn get_string_option(tag_map: &BTreeMap<String, String>, key: &'static str) -> Option<String> {
        tag_map.get(key).map(|x| x.clone())
    }

    // This retrieves a String, but will fail if it is not present
    // The failure is through the Result::Err(...) flow
    // The extra arguments are just for the error construction
    fn get_string_result(
        tag_map: &BTreeMap<String, String>,
        key: &'static str,
        path: &path::PathBuf,
    ) -> Result<String, model::Error> {
        Ok(tag_map
            .get(key)
            .ok_or(model::Error::MissingMetadataKey(
                path.to_string_lossy().to_string(),
                key,
            ))?
            .clone())
    }

    // Get's a u32 value, if present
    // but can still fail if it cannot parse the value as u32
    // If no value is present, it will still just return `Option::None`
    fn get_u32_optional_result(
        tag_map: &BTreeMap<String, String>,
        key: &'static str,
        path: &path::PathBuf,
    ) -> Result<Option<u32>, model::Error> {
        Ok(match tag_map.get(key) {
            Some(v) => Some(v.parse::<u32>().map_err(|_| {
                model::Error::ExpectedU32MetadataValue(path.to_string_lossy().to_string(), key)
            })?),
            None => None,
        })
    }
}
```

A few more links to documentation:
* [`claxon::FlacReader::new(...)`](https://docs.rs/claxon/latest/claxon/struct.FlacReader.html#method.new)
	* This is our major entry point into `claxon`
* [`claxon::FlacReader.tags()`](https://docs.rs/claxon/latest/claxon/struct.FlacReader.html#method.tags)
	* This gives us the metadata tags, that we convert into the `BTreeMap<String, String>`

You may be a bit disappointed that we didn't dig into the weeds of the flac implementation too much; the `reader.tags()` did a lot of the heavy lifting for us. If you're curious to dig into that yourself, you can find a decent starting point in the source code [here](https://github.com/ruuda/claxon/blob/20fd6a78830ec75918175b2375c21dd667b894ce/src/lib.rs#L239). I'm happy to let this library do that lifting for me, and I just get a map of strings to strings, though.

Moving on, the last step here, will be organizing this library; let's give that a shot.

## Fill In the Organizer

### Prepare `model.rs`

We've got one more error to add, and some new methods on our `AudioFileTrackMetadata`, let's see them:

#### **`model.rs`**
```rust
// -- omitting all prior code

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Claxon(claxon::Error),
    ID3(id3::Error),
    MissingMetadataKey(String, &'static str),
    ExpectedU32MetadataValue(String, &'static str),
    // This is our new conflict error
    // It holds the artist, album, disc_no, track_no, and track title
    ConflictingTrack(String, String, u32, u32, String),
}

// -- omitting old impls

// Note the Artist Name `String` in this map
pub struct Library {
    pub artists: BTreeMap<String, Artist>,
}

// Also note the Artist Name `String` in this map
pub struct Artist {
    pub name: String,
    pub albums: BTreeMap<String, Album>,
}

// Also note the Disc Number `u32` in this map
pub struct Album {
    pub name: String,
    pub discs: BTreeMap<u32, Disc>,
}

// Also lastly note the Track Number `u32` in this map
pub struct Disc {
    pub number: u32,
    pub tracks: BTreeMap<u32, AudioFileTrackMetadata>,
}


// This is still just our old struct as-is
pub struct AudioFileTrackMetadata {
    pub artist: String,
    pub album_artist: Option<String>,
    pub album: Option<String>,
    pub disc_no: Option<u32>,
    pub disc_total: Option<u32>,
    pub track: Option<u32>,
    pub track_total: Option<u32>,
    pub track_title: String,
    pub genre: Option<String>,
    pub date: Option<String>,
}

// These methods resolve various "assumable" keys
// They are the keys we were just noting in the `Library`
// structure just above here; they keys are:
// * Artist's Name
// * Album's Name
// * Disc Number
// * Track Number
impl AudioFileTrackMetadata {
    pub fn resolve_album_artist(&self) -> String {
        // Use the album_artist if specified,
        // otherwise just use the artist
        match self.album_artist {
            Some(ref v) => v.clone(),
            None => self.artist.clone(),
        }
    }

    pub fn resolve_album(&self) -> String {
        // If there is no album specified,
        // assume it is a single and the album
        // can just be the track title
        match self.album {
            Some(ref v) => v.clone(),
            None => self.track_title.clone(),
        }
    }

    pub fn resolve_disc_number(&self) -> u32 {
        // If there is no disc specified,
        // assume that it is a single disc release
        match self.disc_no {
            Some(ref v) => *v,
            None => 1,
        }
    }

    pub fn resolve_track_number(&self) -> u32 {
        // If there is no track number assigned,
        // assume that it's a single with just one track
        // as part of it's "album" of a release
        match self.track {
            Some(ref v) => *v,
            None => 1,
        }
    }
}
```

Ok, with that all prepared, let's actually organize these tracks.

### Implementing `organize_tracks`

#### **`organize.rs`**
```rust
use std::collections::BTreeMap;

use crate::model;

pub fn organize_tracks(
    tracks: Vec<model::AudioFileTrackMetadata>,
) -> Result<model::Library, model::Error> {
    // Start with an empty library
    let mut library = model::Library {
        artists: BTreeMap::new(),
    };

    // Iterate over all tracks we've found
    for track in tracks.into_iter() {
        // These instances of `.entry(...).or_insert(...)` pattern are using what is called the Entry API
        // It looks up for a value, if it exists at the key specified as the input to `.entry(x)`
        // and if no value exists, will insert the value specified in `.or_insert(y)`
        // The final result of both calls is that you always have a value at that key and it is returned by `.or_insert(...)`

        // Get or insert the artist with the resolved artist name
        let artist_entry = library
            .artists
            .entry(track.resolve_album_artist())
            .or_insert(model::Artist {
                name: track.resolve_album_artist(),
                albums: BTreeMap::new(),
            });

        // Get or insert the album under that artist with the resolved album name
        let album_entry =
            artist_entry
                .albums
                .entry(track.resolve_album())
                .or_insert(model::Album {
                    name: track.resolve_album(),
                    discs: BTreeMap::new(),
                });

        // Get or insert the disc under that album with the resolved disc number
        let disc_entry = album_entry
            .discs
            .entry(track.resolve_disc_number())
            .or_insert(model::Disc {
                number: track.resolve_disc_number(),
                tracks: BTreeMap::new(),
            });

        // Get or insert the track under that disc with the resolved track number
        // If there exists a value here, it means we have a conflict, which we do not want to allow
        let maybe_conflict = disc_entry
            .tracks
            .insert(track.resolve_track_number(), track);

        // Error out with there is Some(conflict)
        match maybe_conflict {
            Some(conflict) => {
                return Err(model::Error::ConflictingTrack(
                    conflict.resolve_album_artist(),
                    conflict.resolve_album(),
                    conflict.resolve_disc_number(),
                    conflict.resolve_track_number(),
                    conflict.track_title,
                ))
            }
            None => (),
        };
    }

    // After all of the iterating, we should have a full library with every track we passed in
    Ok(library)
}
```

A few more links to documentation:
* [`BTreeMap.entry(...)`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.entry)
	* This is how we look up the values under the BTreeMap and then insert a value if we have not visited this key yet, through: [`Entry.or_insert(...)`](https://doc.rust-lang.org/std/collections/btree_map/enum.Entry.html#method.or_insert)
* [`BTreeMap.insert(...)`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.insert)
	* Note that this returns the value that was present, if there was a value already present; this is how we detect conflicts

Now that these functions are all filled in, let's actually put them to use and see them in action!

## Write a Useful `main()`

Let's write a real `main()` function that uses our new logic:

### **`main.rs`**
```rust
// We'll accept the path to a music library as a command line argument
// which we'll access through the `env` crate
use std::env;

// These are our same modules we declared originally
mod util;
mod parse;
mod scan;
mod organize;
mod model;
pub mod impls;

fn main() {
    println!("Let's read some audio metadata");
    // Accept the library path as a command line argument
    // and parse it here
    let args: Vec<String> = env::args().collect();

    println!("Let's build the library by parsing all of the files");
    // Call the `impls` method `model::Library::from_library_directory`
    // This should build a full library for us
    // if the passed directory has a library in the shape we expect
    let library = model::Library::from_library_directory(args[1].clone()).unwrap();

    // Print out the artists, albums, discs, and tracks
    // in a tab-indented tree
    println!("Now let's print all of the tracks we found");
    for artist in library.artists.values() {
        println!("\tArtist: {}", artist.name);
        for album in artist.albums.values() {
            println!("\t\tAlbum: {}", album.name);
            for disc in album.discs.values() {
                println!("\t\t\tDisc: {}", disc.number);
                for track in disc.tracks.values() {
                    println!("\t\t\t\tTrack: {: >3} - {}", track.resolve_track_number(), track.track_title);
                }
            }
        }
    }
}
```

And if we run it, with some sample input:

```
 $ cargo run -q -- path_to_music/
Let's read some audio metadata
Let's build the library by parsing all of the files
Now let's print all of the tracks we found
	Artist: Chillest
		Album: Songs To Dream To
			Disc: 1
				Track:   1 - Lying There
				Track:   2 - Heavy Eyelids
				Track:   3 - Dozing
			Disc: 2
				Track:   1 - Enter the Dream
				Track:   2 - The Adventure
				Track:   3 - Sunlight
	Artist: The Rockers
		Album: Party Time
			Disc: 1
				Track:   1 - Intro
				Track:   2 - The Hit
				Track:   3 - Outro
```

Pretty nifty, if I do say so myself! It's nothing too impressive, but for the amount of code we wrote, it's nice to have this little tree of output to show for. If you want to run this yourself...

# Working Example

I have a working example of all of this code [here](https://github.com/quintenpalmer/quintenpalmer.github.io/tree/main/codeexamples/2022-01-29-parsing) if you want to run it yourself, or just see it all in one place.

If you want some sample files to run on, you can `sh generate_flac_library.sh` in that directory and it will generate a few `.flac` files that you can then pass to the rust binary with `cargo run -- path_to_music/`.

Also, you may notice the linked code has support for [`.mp3`](https://en.wikipedia.org/wiki/MP3) files as well with the [`id3`](https://docs.rs/id3/latest/id3/) crate, if you want to try adding support for [`.m4a`](https://en.wikipedia.org/wiki/MP4_file_format) with [mp4ameta](https://docs.rs/mp4ameta/latest/mp4ameta/) or [`.ogg`](https://en.wikipedia.org/wiki/Ogg) with [ogg](https://docs.rs/ogg/latest/ogg/) and [lewton](https://docs.rs/lewton/0.10.2/lewton/), those should be good starting points.

# Conclusion

Hopefully that was easy enough to follow along, and a shed a bit of light on how to wrangle audio metadata (with Rust)! If you're curious to read how the specs actually work, and how the libraries actually parse out the data, feel free to do so, and if you find anything cool, maybe share some of it with us all!

# Next Installment

Thanks for reading along today. Stay tuned for the next post, where we'll use [Iced](https://docs.rs/iced/latest/iced/) to build an app, leveraging this parsed metadata. Cheers!
