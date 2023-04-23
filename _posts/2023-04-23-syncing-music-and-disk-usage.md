---
title: Syncing Music (and Disk Usage)
date: 2023-04-23
---

# Introduction

As established in a [previous post](/2023-03-12-why-acquire-music) about acquiring music, some of the benefits I have been pursuing require having the media files on each device that I want to use to play music. This makes it easy once everything is set up, but does require some initial investment into hardware and software solutions. This post will go over the basics of how I've approached this.

# Managing and Syncing Files

## Organizing the Files

The general structure that I follow is organization that looks like: `Artist/Album/Disc/Track.flac`, where the `Disc/` is omitted if there is only one disc. I also keep a `cover.jpg` at the `Album/` level; as an example:

```
 $ tree path/to/music/
path/to/music/
├── Chillest
│   └── Songs To Dream To
│       ├── cover.jpg
│       ├── Disc 1
│       │   ├── 1.Lying There.flac
│       │   ├── 2.Heavy Eyelids.flac
│       │   └── 3.Dozing.flac
│       └── Disc 2
│           ├── 1.Enter the Dream.flac
│           ├── 2.The Adventure.flac
│           └── 3.Sunlight.flac
└── The Rockers
    └── Party Time
        ├── 1.Intro.flac
        ├── 2.The Hit.flac
        ├── 3.Outro.flac
        └── cover.jpg

6 directories, 11 files
```

In this example there are two artists, "Chillest" and "The Rockers", with two albums, "Songs To Dream To" and "Party Time", respectively. You can see under each album we have a `cover.jpg` and "Songs To Dream To" has a "Disc 1" and "Disc 2".

Of note: the naming for the artists and albums don't matter as far as my software is concerned. I have audited them to make sure they are close enough that it makes sense to me, a human consumer of the directory structure, but they could diverge in the future if I'm not careful. Famous last words, I know! The only piece of directory structure that my software needs correct, is that if an album is multi-disc it needs to have disc directories to know where to find the `cover.jpg` (it always assumes in the same directory if the album is single-disc and one-directory-up from the track if the album is multi-disc).

## Adding New Files

My software requires that all files adhere to this structure, so I had to massage my existing music library to get it into the right shape. And every time I buy new music, I need to make sure it matches this schema. For new music, I always start extracting whatever `.tar` or `.zip` file I've downloaded into a hand-crafted directory of `Artist/Album/` (moving into `Disc 1`, `Disc 2`, etc if necessary). I will then need to make sure there is a `cover.jpg`. A few fun ways that I often make that happen:

```bash
# either "copy" the cover.png (I could make my app smarter and just fall back to looking for this)
ffmpeg -i cover.png -c copy cover.jpg
# or extract the cover from the embedded image that is always there coming from Bandcamp
ffmpeg -i TRACK.FLAC -c copy cover.jpg
```

It's a bit of a process every time I buy music, but I haven't bothered to automate it yet. With that directory structure established, let's talk about syncing files so that all devices can have access to these wonderful music files!

## Syncing Files

I use [Syncthing](https://syncthing.net/) to copy my music files around. I currently have 4 devices that I sync between: a Raspberry Pi, a laptop, a desktop, and a smartphone. While Syncthing can be used to talk between any number of peers, I do use a single-server-many-client layout, just for simplicity.

### Raspberry Pi 4 Model B Server

My server is a [Raspberry Pi 4 Model B](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/)! It has more than enough power to serve as the "media server" as I call it. It's running [Raspberry Pi OS](https://www.raspberrypi.com/software/) (previous called Raspbian), which has a recent enough version of Syncthing available. This is always running, so whenever there is new music added, it's aware of it as soon as it can be. This is the only device that any other device I own connects to with Syncthing. It serves as the single of source of truth, just to avoid any complications/conflicts syncing files.

### The Clients

#### Android Phone

I have a 512GB SD card in my phone (where Android also has a syncthing client), which is more than enough room for the 300GB of music that I currently have.

#### Framework Laptop

My [Framework Laptop](https://frame.work/products/laptop-diy-11-gen-intel) has a 2TB NVMe SSD, which is more than more than enough space for my music library, and then everything else I do with my laptop (including writing these blogs!).

#### Desktop Computer

I also have a custom-built desktop computer with ~6TB of storage split between NVMe SSDs and a spinning HDD, which is more than more than more than enough storage.

### Syncthing Flow

The basic flow when I acquire music (usually purchased and downloaded onto my laptop) is that I will prepare the files as described above and then once they are in my synced library directory it will sync up to the RPi4 media server and then down to my phone when I start the Syncthing app there and down to my desktop, next time I power that up. My music app will automatically scan the directory and pick up the new tracks next time it is booted up, and VLC on my phone just requires that I select its "Refresh" button. And that's "it", now I can enjoy my music on any of my devices! Except one, that I haven't mentioned yet, let's see what that's about...

# Disk Usage Concerns

I casually mentioned earlier that I have 300GB of music data; that is actually a decent amount! I prefer `.flac` files, in some kind of foolish pursuit of purism (I've tried to do a blind test playing back `.mp3` and `.flac` files with nice headphones and I can't hear any difference), so this definitely adds up in size faster than it needs to.

## Steam Deck Client

I also recently bought a [Steam Deck](https://www.steamdeck.com/en/), and only got the 64GB eMMC storage option (I may be addicted to consumerism, but I still like to be cheap when I can!). I plan to get another 512GB SD card, like I did for my phone, and while 300GB of music would fit in there, I would like to load games into that space as well, since, you know, it's a device primarily made for playing games!

## Introducing `mp3ify`

In preparation for this, I recently produced a `.mp3` copy of my library, and I'd like to share how I did.

### New Custom Tool: `extension`

The first thing I wrote was a simple utility that just gives the `.suffix` extention of an input file. Really quickly, it looks like this:

#### **`Cargo.toml`**
```toml
[package]
name = "cli"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[[bin]]
name = "extension"

[dependencies]
```

#### **`main.rs`**
```rust
use std::env;
use std::path;

fn get_extension<P: AsRef<path::Path>>(input_path: P) -> String {
    match input_path.as_ref().extension() {
        Some(v) => v.to_string_lossy().to_string(),
        None => "".to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, input_path] => println!(".{}", get_extension(input_path)),
        _ => panic!("must supply a single path"),
    }
}
```

We can build this and add it to our `$PATH` so that we can call it like this:

```bash
 $ extension ~/.bin/volumectl-debounce.sh
.sh
```

Pretty simple! Let's take a look at some existing software I'll use with this `extension` binary to conjure an `.mp3` copy of my library.

### Existing Tool: `fd`

[`fd`](https://github.com/sharkdp/fd) is a utility similar to the GNU utility [`find`](https://www.gnu.org/software/findutils/), and it's been my preferred find-ing utility since I discovered it. One nice thing that we'll be leveraging that `fd` provides, is that while executing a given command for each input, the commands are run in parallel, as [described here](https://github.com/sharkdp/fd#command-execution).

Using `fd` and `extension` together, we can actually produce a list of all file extensions currently in my library:

```bash
fd -t f -x extension {} | sort -u
.flac
.gif
.jpg
.m4a
.mp3
.pdf
.PDF
.png
.rtf
.txt
```

The `-t f` option says to only find files (not directories, or anything else) and the `-x extension {}` says to run each found file through our `extension` binary providing the found file as an argument (that's what the `{}` means, it's a placeholder to expand from the found entry). `sort -u` just produces a deduplicated list from its inputs. We see here, `.mp3`, `.flac`, and `.m4a` as music files, `.gif`, `.png`, and `.jpg` for image files, and then also `.pdf`, `.PDF`, `.rtf`, and `.txt` as other bonus files. Once we're actually copying files around, we'll let all of these extra files come along for the ride, even if my software doesn't do anything with them (yet??).

### Existing Tool: `ffmpeg`

We know about [`ffmpeg`](https://ffmpeg.org/) from a [previous post](/2023/01/22/learning-audio-metadata-with-ffmpeg)! What we'll be using from `ffmpeg` this time around, is its `libmp3lame` integration, to produce `.mp3` files. I'll be keeping the `.mp3` files to the most reasonable highest quality, specified with a quality level of "0", as [documented here](https://trac.ffmpeg.org/wiki/Encode/MP3). So we could run something like:

```bash
ffmpeg -i input.flac -codec:a libmp3lame -qscale:a 2 output.mp3
```

And we would get an output `.mp3` file from an input `.flac` file that _should_ be indistinguishable from the source `.flac` file. Ok, equipped with all of this, let's dive into the `mp3ify` script that will tie it all together!

### New Custom Tool: `mp3ify`

This is a bash script, that is documented inline with what's going on; I'll let that inline documentation describe what's going on, but the basic idea is that it will take (a few descriptions of) an input file and copy it if it's a file we want to let pass through (like an `.mp3` or `.jpg`) or convert it to an `.mp3` if it's a `.flac` file.

#### **`mp3ify`**
```bash
#!/bin/bash

# These three arguments really could be computed from just the first one,
# but `fd` will do some of this work of stripping extensions,
# and computing parent directories,
# so I just let `fd` do that for me

# The first argument should be the full path of the found file
# `fd` can pass this through with `{}`
FULLPATH="${1}"
# We also expect the file without any extenion (so we can add .mp3 ourself)
# `fd` can compute this with `{.}`
FULLPATHSANSEXT="${2}"
# We also, also expect the parent dir of the file, so we can `mkdir -p`
# `fd` can compute this with `{//}`
FULLPATHSANSFILE="${3}"

# We compute the extension of the file we found
FILEEXT=$(extension "${FULLPATH}")


# This is a hardcoded "landing directory"
# The script could be made parameterize-able over this directory
PARENTDIR="/home/quinten/coldstorage/media/music/compressed/"

# If we find a flac file
if [ "${FILEEXT}" == ".flac" ]; then
    set -e
    set -x
    # Try to make the parent directory in the landing directory
    # This will be something like "ArtistName/AlbumName/Disc 1"
    mkdir -p "${PARENTDIR}${FULLPATHSANSFILE}"
    # The `echo n` just tells ffmpeg to not try to overwrite
    # any existing files if there are any
    # This `ffmpeg` invocation should look familiar from above
    # And we build the full path with the `.mp3` extension
    echo n | ffmpeg -i "${FULLPATH}" -qscale:a 0 "${PARENTDIR}${FULLPATHSANSEXT}.mp3"
    # If the file is any of there other known files
elif [ "${FILEEXT}" == ".mp3" ] || \
     [ "${FILEEXT}" == ".m4a" ] || \
     [ "${FILEEXT}" == ".jpg" ] || \
     [ "${FILEEXT}" == ".png" ] || \
     [ "${FILEEXT}" == ".rtf" ] || \
     [ "${FILEEXT}" == ".gif" ] || \
     [ "${FILEEXT}" == ".txt" ] || \
     [ "${FILEEXT}" == ".pdf" ] || \
     [ "${FILEEXT}" == ".PDF" ]; then
    set -e
    set -x
    # Still try to make the parent dir
    mkdir -p "${PARENTDIR}${FULLPATHSANSFILE}"
    # And then copy and "update-only" with the -u flag
    cp -u "${FULLPATH}" "${PARENTDIR}${FULLPATHSANSFILE}"
else
	# Otherwise, let the caller know that we didn't try to operate on this file
    echo not operating on ${FULLPATH} with ${FILEEXT} which is not .flac nor .mp3
fi
```

And then the way we use this `mp3ify` is to just `cd` into the library directory and run:

```bash
cd ~/path/to/flac/library/
fd -t f -x mp3ify {} {.} {//}
```

I called this out in-line in `mp3ify` but the `{}`, `{.}`, and `{//}` produce: the filename, the filename-without-extension, and the-filename's-parent-directory, spelled out with an example `/home/quinten/library/Artist/Album/track.flac`, we would have:
* `{}` = `/home/quinten/library/Artist/Album/track.flac`
* `{.}` = `/home/quinten/library/Artist/Album/track`
* `{//}` = `/home/quinten/library/Artist/Album/`

If you run that command from above on a reasonably-sized library, you can watch as your machine consumes hundreds of watts while your CPU goes hard running ffmpeg as fast as it can (or it did for me at least). Once its done though, you should have a metadata-preserved copy of your library that should sound the same as your `.flac` copy, and takes maybe 1/3 the space (mine went from `300GB` to `96GB`).

One nice thing about this script is that it's idempotent, so we can safely run it again and it will crank much less hard as it verifies everything is in the correct state. And!! We can run it when we have new music and it will only generate `.mp3` files for the new `.flac` files we add (copying new `cover.jpg` files along the way too)!

After I buy the SD card for my Steam Deck, I will use Syncthing to copy this data up to my media server and then sync it back down to the Steam Deck. I'll post a tiny update when I do that.

# Conclusion

Alright, now you know how I manage my music files for all of my devices. Hopefully this was somewhat of a breath of fresh air from my dense Rust posts. Next week will still probably be [MPRIS](https://specifications.freedesktop.org/mpris-spec/latest/) which will include Rust code again, sorry that that was pushed back, if you were looking forward to that. Until then, take care (it's accidentally become my catch phrase from the last few posts).