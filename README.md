# Introduction

Hello! My name is Quinten and this is the first installment of a series of technical blog posts about a project I have been working on called `musiq` or `musiqapp` (pronounced like music, but I like the letter Q). Let's start with explaining what this project aims to solve for, and then I'll explain what I hope to cover with this series.

(tl;dr: it's a Rust application for Linux that organizes and plays back music and I'll be digging into various UX/database/API topics)

## What is this Musiqapp?

My `musiqapp` application is a media player that I have been working on to solve a problem for myself: I have been acquiring digital media (I can dig into the philosophy of why I have gone this route another time) and want an intuitive and responsive app to let myself organize, explore, and play back my music.

Without getting too technical yet, it's currently:
* A [Rust](https://www.rust-lang.org/) project
* Using the [Iced](https://docs.rs/iced/latest/iced/) framework for the graphical user interface
* Leveraging [SQLite](https://www.sqlite.org/index.html) for the data storage
* And [Rodio](https://docs.rs/rodio/latest/rodio/) for the audio playback

A quick overview of its most noteworthy features includes:
* Ability to organize all music by artist/album/tracks
* Play specified music with a play-queue that the user can manage
* Create and play back user-created playlists
* Record and report the time of playback of each track
* Sort track/albums/artists by (a list including but not limited to):
	* Name
	* Play-count
	* Date-added
	* Length
	* Random

I'll definitely dig more into this as I blog more about this, but I'll keep it short for now. Next let's cover what I hope to write about with this series:

## What will this Blog Series be?

All of the content in this series will be about this `musiqapp` media player. Most of the topics will be technical, but I do want to highlight some of the user experience decisions I've made, especially since the desire for a particular UX has been a driving factor of what has made me want to write my own app for this in the first place.

With that in mind, some of the topics I'm thinking I want to cover first will be:
* Deciding on my User Experience (for the reasons stated above)
* The Iced architecture and how I've fit my needs into that model
* Translating my data storage from a collection of JSON files to SQLite
* Playing nice with Linux norms (~/.config/, ~/.local/share/, and [MPRIS](https://specifications.freedesktop.org/mpris-spec/latest/))
* Prototyping and hacking using the functionality that the app itself uses

# Expectations (For You To Hold)

Let me establish what you should expect from this series.

## Why Am I Writing This?

Mostly to practice my writing, technical and otherwise. Maybe it will also serve as advertising of sorts for this project once I have it in a state that I think other interested parties in the world could use `musiqapp` for themselves. It also feels like a reasonable way to keep me honest continuing to work on the project itself and keep the code in a presentable state.

## Topics

Pardon the reference to above, I found myself facing a bit of a chicken-and-egg problem with how to introduce the blog and set expectations, but please see the "**What will this Blog Series be?**" above for a more full description of what to expect with the topics. To recap, expect me to cover almost entirely technical topics digging into what reasoning went into my decision making and what hurdles I encounter and how I work around them.

## Timeline

This blog series is part of a New Year's Resolution with a friend of mine, Eric; you can find his first blog post [here](https://edbrown23.github.io/blog/2023/01/15/welcome-to-the-blog). As part of this resolution, we are aiming to blog once every two weeks; first one is due by January 15th 2023.

## Technical References

How will I communicate the technical nitty-gritty? There are a few things I want to share for how I aim to do this.

### Git

Ok, please hear me out. Normally I'm an avid [git](https://git-scm.com/) user and have always been the largest proponent of proper git usage at every company I've worked at. I almost always have clean git history even on projects I work on by myself. But for this project, I have been loose and have no git history to speak of yet. The code base is not something I thought I wanted to share with the world just yet, but this blog series is forcing my hand.

I will aim to conjure a reasonable git history by the next blog post (it will just add what the project looks like at this point in time in a compelling story of bite-sized commits and branches). I do intend to share this project with the world as free and open source software, I really did just start with a prototype that I intended to throw away, but here I am using it daily in its current state.

Bottom line: expect the project to be uploaded in its entirety for you to browse and reference compared to what I write.

### Inline Snippets

I definitely plan to include code snippets for the technical topics that I cover and may go so far as to include a working code link that demonstrates what a given post is aiming to demonstrate. Whether I go that far with expanding on each topic, I will definitely include links to the hosted git history for the relevant logic.

### Links to Documentation

I will definitely link to the documentation that I have been using, which I imagine will be helpful for anyone trying to extract the essence of my lessons to anything they may be working on themself. I won't expect people to read up on these docs to be able to follow along, they will just be supplementary.

## Lastly: About Me

My technical background is that I have a Bachelors of Science degree in Computer Science and have been a professional Software Developer from 2012-2020 (I have been taking time away from professional software dev for the past 2 years). I only mention this for you to be able to place an expectation of what you think I may or may not know given that scope of experience.

# Wrapping Up

If you think that you may be interested in the content of this series, I would love to have you back for the first technical write-up. If not, thank you for taking the time to digest this all to make that evaluation for yourself. Either way, Happy New Year!
