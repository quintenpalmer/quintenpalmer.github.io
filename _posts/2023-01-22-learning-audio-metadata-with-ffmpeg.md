---
title: Learning Audio Metadata With FFmpeg
date: 2023-01-22
---

# Introducing the Metadata

Welcome [back](/2023/01/15/introducing-my-musiq-player-and-blog-series) for the first "real" installment in this blog series. Here, I want to introduce the metadata that is the centerpiece of my music player's entire experience. Once we work through the introduction here, there will be two major components of this:
* The specifications of the audio metadata formats that we operate on
* What tooling exists to inspect said metadata

## What is Metadata?

Let's start at the top, "what is metadata?". To quote [Wikipedia](https://en.wikipedia.org/wiki/Metadata):
> **Metadata** is "data that provides information about other data", but not the content of the data, such as the text of a message or the image itself.

I don't really have much to add beyond that for metadata in general, let's see what kind of metadata exists for our media.

## Audio File Metadata

Today the metadata we will be talking about will be data concerning album info, artist info, track info, etc. The data "proper" would be the bits that represent the audio waveform to reproduce for your speakers/headphones/etc. Each audio file will have this metadata inside their file contents.

A simple way to think of the metadata we will be working with here is as a map of keys to values. For all of the text metadata, it could map to [JSON](https://www.json.org/json-en.html), which I'll use to establish a baseline. An example of some metadata (exact names of the keys will vary based on the encoding format; we'll cover those exact details in a bit) we expect to see in a given music file would be something like:

```JSON
{
	"artist": "The Cool Band",
	"album_artist": "The Cool Band",
	"album": "Fun Songs for Dancing",
	"disc": "1",
	"disctotal": "1",
	"track": "1",
	"tracktotal": "14",
	"title": "Wonderful Time",
	"date": "2009",
	"genre": "Dance Pop"
}
```

The keys and their values should hopefully be pretty self-explanatory. I'll just mention a few things:
* I have not noticed that any of schemas for this metadata have a concept of data types, so every value is just a string
* `"artist"` vs `"album_artist"
	* `"album_artist"` is meant to be a single value that should be the same for every track in an album
	* `"artist"` can vary from track to track in an album
	* Think of an album released by a many artists (like some kind of collaborative effort/group project) and the `"album_artist"` would maybe be all involved artists or just "Various Artists" and the `"artist"` value would be the specific artist for each track
* `"disctotal"` and `"tracktotal"
	* These just let you know how many total values exist for the given album; `"disc"` and `"track"` should never exceed these values

To repeat, each file will have all of this information tagged on it. There is no `album.json` or anything that contains the album information that the tracks can refer to, it will all be repeated, or can be computed after by looking at all tracks that "belong togeter" (we'll get to how to decide what tracks are together in a future blog post).

With this hypothetical JSON example in mind, let's dig into some specific formats.

# Introducing the Audio Formats

There are three major definitions I want to establish for this section:

## The Definitions

### Audio File Format

An [Audio File Format](https://en.wikipedia.org/wiki/Audio_file_format) is a file format for storing digital audio, both the actual audio bits as well as any extra bits for metadata and/or other information.

### Audio Coding Format

An [Audio Coding Format](https://en.wikipedia.org/wiki/Audio_coding_format) is a content format to hold the bits that represent the actual audio. They can be lossy or lossless, compressed or uncompressed, and vary in other ways.

### Metadata Container

Some audio file formats define a specification for metadata, but some do not. If the file format does not define how to embed metadata directly, a separate metadata container will be leveraged. These metadata containers are what we will be focusing on today.

## Concrete Audio File Formats

We're only going to be focusing on two audio file formats here: [FLAC](https://en.wikipedia.org/wiki/FLAC) and [MP3](https://en.wikipedia.org/wiki/MP3). Please note: MP3 and FLAC are both a kind of audio file format as well as the name of their audio coding formats. Let's take a look at these two file formats and the related metadata containers that exist for them. Just to spell them both out:

### MP3

The `.mp3` audio file format uses the audio coding format of the same name: MP3. The most common metadata container used in `.mp3` files is the [ID3](https://en.wikipedia.org/wiki/ID3) container.

### FLAC

The `.flac` audio file format also uses an audio coding format of its same name: FLAC. The FLAC format also has support for metadata built into its spec, which we (and the industry in general) leverage.

# Specifics of Each Audio Format Metadata

Let's dig into the specifics of the metadata tagging used with each audio format. We will do so with tables that enumerate:
* Frame Key
	* The key name for this specification
* Value Description
	* Plain english description of what this value represents
* FFmpeg Canonical Key
	* We will treat these as the canonical frame key names
	* These are what FFmpeg uses as the key names across most formats instead of the specific names for each format
		* (We'll get into FFmpeg soon, one of these sections had to be first before we dug into the other, sorry)

## ID3 (MP3)

As enumerated on [Wikipedia](https://en.wikipedia.org/wiki/ID3#ID3v2_frame_specification) the ID3 spec lists the following that we will care about:

| ID3 Frame Key | Value Description | FFmpeg Canonical Key |
|---|---|---|
| `TPE1` | Track Artist | `artist` |
| `TPE2` | Album Artist | `album_artist` |
| `TALB` | Album Name | `album` |
| `TPOS` | Disc Number (first half of format `1/2`) | `disc` |
| `TPOS` | Disc Total (second half of format `1/2`) | `disctotal` |
| `TRCK` | Track Number (first half of format `1/10`) | `track` |
| `TRCK` | Track Total (second half of format `1/10`) | `tracktotal` |
| `TIT2` | Track Title | `title` |
| `TYER` | Year of Release | `date` |
| `TCON` | Song Genre | `genre` |

The value descriptions don't match precisely what Wikipedia reports, but the mapping I am describing do match what I've seen in the real world, for whatever that is worth. Theory vs Practice and whatnot.

## FLAC

The FLAC specification does not define the metadata field names to use (see section 9.6.1 of the [spec](https://datatracker.ietf.org/doc/draft-ietf-cellar-flac/)) so we'll follow their link to MusicBrainz's [basic tags](http://picard-docs.musicbrainz.org/en/variables/tags_basic.html) and [advanced tags](http://picard-docs.musicbrainz.org/en/variables/tags_advanced.html) and we can establish:

| FLAC Frame Key | Value Description | FFmpeg Canonical Key |
|---|---|---|
| `artist` | Track Artist | `artist` |
| `albumartist` | Album Artist | `album_artist` |
| `album` | Album Name | `album` |
| `discnumber` | Disc Number | `disc` |
| `totaldiscs`\* | Disc Total | `disctotal` |
| `tracknumber` | Track Number | `track` |
| `totaltracks`\* | Track Total | `tracktotal` |
| `title` | Track Title | `title` |
| `date` | Year of Release | `date` |
| `genre` | Song Genre | `genre` |

\* I have not seen `totaldiscs` nor `totaltracks` tagged in any music I've purchased, but I do see `disctotal` in 98% of tracks with a `discnumber` tagged (not all distributions seem to tag even the `discnumber` if there is only one disc) and `tracktotal` in >50% of my total tracks.

With all of this in mind, let's try to get some rubber on some road and use a real tool that will inspect real files, using `ffmpeg`.

# What is FFmpeg?

## Basics of FFmpeg

[FFmpeg](https://ffmpeg.org/) is a software project built to help you do anything you want with audio, video, images, and other multimedia.

To use their own words:
> A complete, cross-platform solution to record, convert and stream audio and video.

And to quote [Wikipedia](https://en.wikipedia.org/wiki/FFmpeg):
> **FFmpeg** is a [free and open-source software](https://en.wikipedia.org/wiki/Free_and_open-source_software "Free and open-source software") project consisting of a suite of [libraries](https://en.wikipedia.org/wiki/Library_(computing) "Library (computing)") and [programs](https://en.wikipedia.org/wiki/Computer_program "Computer program") for handling video, audio, and other [multimedia](https://en.wikipedia.org/wiki/Multimedia "Multimedia") files and streams. At its core is the [command-line](https://en.wikipedia.org/wiki/Command-line_interface "Command-line interface") `ffmpeg` tool itself, designed for processing of video and audio files.

The main thing that we'll be leveraging FFmpeg for throughout this blog post is to manipulate the metadata. FFmpeg is so much more powerful though, we really are using so little of what it is capable of. FFmpeg rocks, it's worth poking around with if you're curious about it! Anyways, back on track.

## FFmpeg Binaries

FFmpeg consists of three major binaries, but we'll really only be focusing on two today:

### `ffmpeg`

This is the real powerhouse that does the data transformation. The most common usages of `ffmpeg` will pass the following:
* A source of input (often a file)
* A list of transformations to perform
* A source to output to (also, often a file)

We'll do some examples very soon for what kinds of transfomations you can do.

### `ffprobe`

This tool inspects an input file and prints out the information about it in human or machine readable formats. We will be using this tool to show the metadata of files we have produced.

### `ffplay`

This tool will play back a given file, be it a music file, video file, image, or otherwise. We will barely use this tool in this blog post.

## Example Usages of `ffmpeg` and `ffprobe`

As mentioned with the specific file formats, ffmpeg has canonical names for the frame keys, the best documentatation that I can find that matches with what I've seen in the real world can be found [here](https://wiki.multimedia.cx/index.php/FFmpeg_Metadata#MP3). We'll be using these canonical keys, but note that it will turn into the specific frame key names that were described for both MP3 and FLAC\*.

\* With just two small notes:
* This means that specifying `"album_artist"` as a key for FLAC will actually transform it to `"albumartist"`, which are so close, but are technically different
* `"disctotal"` and `"tracktotal"` still seem to be what FFmpeg converts into for flac instead of the MusicBrainz's `"totaldiscs"` and `"totaltracks"`

### Generating a File with `ffmpeg`

Let's start by creating a few (very boring) file using `ffmpeg` and I'll discuss what all of the options do:

```bash
ffmpeg \
	-f lavfi \
	-i "sine=frequency=220:duration=4" \
    -metadata ARTIST="The Cool Band" \
    -metadata ALBUM_ARTIST="The Cool Band" \
    -metadata ALBUM="Funky Songs for Dancing" \
    -metadata DISCNUMBER=1 \
    -metadata DISCTOTAL=2 \
    -metadata TRACK=1 \
    -metadata TRACKTOTAL=14 \
    -metadata TITLE="Wonderful Time" \
    -metadata DATE=2009 \
    -metadata GENRE="Dance Pop" \
    example_song.flac
```

This should create a file that looks like our example JSON from above. Let's talk through each piece:
* `\` at the end of almost each line
	* This is so that we can have a multiline command; it's a [bash](https://www.gnu.org/savannah-checkouts/gnu/bash/manual/bash.html#Escape-Character) thing.
* `-f lavfi`
	* This sets the format for the input, which will be a generated tone, as we're not actually generating from an input file (`ffmpeg` would normally infer this for us)
* `-i "sine=frequency=220:duration=4"`
	* This is the tone that we want to generate, a sine wave, with a frequency of 220 for a duration of 4 seconds
* ``-metadata {KEY}={VALUE}`
	* These are us setting each of the individual metadata values
* `example_song.flac`
	* This tells `ffmpeg` to write all of this out to a file with this name. `ffmpeg` infers the encoding from the file extension `.flac`, the rest of the filename doesn't matter to `ffmpeg`

### Inspecting a File with `ffprobe`

Let's take a look at this file we just generated with `ffprobe` now:

```bash
ffprobe -hide_banner example_song.flac
Input #0, flac, from 'example_song.flac':
  Metadata:
    ARTIST          : The Cool Band
    album_artist    : The Cool Band
    ALBUM           : Funky Songs for Dancing
    disc            : 1
    DISCTOTAL       : 2
    track           : 1
    TRACKTOTAL      : 14
    TITLE           : Wonderful Time
    DATE            : 2009
    GENRE           : Dance Pop
    encoder         : Lavf59.27.100
  Duration: 00:00:04.00, start: 0.000000, bitrate: 112 kb/s
  Stream #0:0: Audio: flac, 44100 Hz, mono, s16
```

Taking a look at this output, let's discuss what we see here:
* `-hide_banner`
	* Normally the all of these tools display a lot of build information that is honestly just noisy, so we're going to supress it
* `example_song.flac`
	* If you run this in the same directory you generated the file in, this is the main input to `ffprobe`
* `Input #0`
	* This output line just lets us know what input this output is for (the one we passed it)
* `Metadata`
	* The output has a bunch of sub fields for each of the tags we encoded when we generated this file
* `Duration` and `Stream`
	* These are both output information about the actual sine wave that we generated; we won't be focusing on this part of the output nor data here

### Transforming a File with `ffmpeg`

Most people using `ffmpeg` use it to operate on existing data, so let's try that now. Some of you may have noticed that the initial JSON had an album named "Fun Songs for Dancing" while our file we tagged had the album "Funky Songs for Dancing". Let's fix that!

```bash
mv example_song.flac old_example_song.flac

ffmpeg \
	-i old_example_song.flac \
	-c copy \
	-metadata "ALBUM=Fun Songs for Dancing" \
	example_song.flac
```

Let's talk through this, piece by piece, again:
* `mv example_song.flac old_example_song.flac`
	* `ffmpeg` doesn't allow you to write out to the same file that you are using as input, so we have to rename the file with `mv` before we can start
	* For reference, it gives you the following error lines before exiting early:
		* `Output example_song.flac same as Input #0 - exiting`
		* `FFmpeg cannot edit existing files in-place.`
* `-i old_example_song.flac`
	* We specify this old filename as the input for `ffmpeg`
* `-c copy`
	* We want `ffmepg` to leave the contents of the actual audio information alone here, so we give it the `-c` (short for `-codec`) option with the value `copy` to tell `ffmpeg` to copy the audio information 1:1
* `-metadata "ALBUM=Fun Songs for Dancing"`
	* This is exactly as when we used the `-metadata` option creating the file initially. This does the correction of the album metadata from Funky to Fun
	* Note the `"` used to create a single argument to pass to `ffmpeg` too
		* You only need to capture the arguments that would otherwise have a space in them, some other ways that you could do this would be:
			* `-metadata ALBUM="Fun Songs for Dancing"`
			* `-metadata ALBUM=Fun\ Songs\ for\ Dancing`
* `example_song.flac`
	* This specifies the output filename, also like when creating the file initially

### Playing a File with `ffplay`

If you want to hear 4 seconds of a 220Hz sine wave, you can play back our generated file with:

```bash
ffplay -autoexit example_song.flac
```

Real quick:
* `-autoexit`
	* By default, `ffplay` will not exit when the playback is over; this tells it to do so
* `example_song.flac`
	* This specifies the input file to play

## Trying FFmpeg Yourself

I have a [collection of shell scripts](https://github.com/quintenpalmer/quintenpalmer.github.io/blob/main/codeexamples/2022-01-22-ffmpeg) that you can run yourself to toy around with FFmpeg as we did here. You'll need to install `ffmpeg` either from the package manager on your system or from their [download page](https://ffmpeg.org/download.html). Enjoy!

# Conclusion

Alright, that was a decent amount of specifications and real-world usage of `ffmpeg`. If you want to poke around with existing music files you have, hopefully you have the knowledge to know what to look for and what to make of it all.

# Next Installment

We'll be using the information laid out here as the foundation for the contents of the next blog post, where we'll use Rust and some [audio](https://crates.io/crates/claxon) [libraries](https://crates.io/crates/id3)  to process music files and build up a tree of what my Musiq App considers a "canonical" "library" (intentional separate scare quotes). Stay tuned!