---
title: Why Acquire Music
date: 2023-03-12
---

# Why (And Why Not To) Acquire Music

This post will be a departure from the previous posts, focusing on some more philosophical ideas and thinking about things a bit larger than tech. I also missed a week; I'll publish two in these coming two weeks.

The main ideas I'll be focusing on the pros and cons of acquiring music instead of using any of the various music streaming services. Let's dig right in.

# The Pros

## Playback of Local Files

One of the easier pros to understand is how much easier it is to play back local files (either on the local network if using a local media server, or from the local filesystem if using an app that reads the media from the same machine). Not only is this just easier to program for, but has benefits even in our (nearly) always connected world a lot of us live in.

One of the ways this can be beneficial if an internet connection is not available, from an ISP outage in the home, if out in a remote location where cell service is spotty, or otherwise. I was just coming back from a trip in New Hampshire last weekend and everyone was having spotty cell service, so I was actually able to use my Musiq app on my laptop to provide music for the ride home!

Local files can also help with snappier feedback of UI elements leading to a more pleasant UX. One of the nice things with my current Musiq app is that every action the user takes is near-instantaneous, as there are no network calls or anything async to load the "page" for an artist or album or otherwise.

## (Hopefully) Better Support Artists

If you've been following the music streaming industry recently (~10 years being recently), you may have seen buzz about how little artists get through their contracts with these streaming services. Whether you count it on how little an artist makes when someone streams a song or how potentially imbalanced the payout distribution is for smaller artists, it seems there is room for improvement. While purchasing music through larger platforms may result in similarly small cuts to the artist, I acquire what music I can through [Bandcamp]() (especially on [Bandcamp Fridays]()), which hopefully gets more money to the artist. Not everyone is on Bandcamp though, so I do fall back to purchase through those larger platforms in those cases.

I do believe that markets in general move as the sum total of all of the actions of their participants, so I hope my little-more-than-streaming-support goes some substantial distance, in the long run. Maybe my actions matter less than I think, but even if just for the psychological benefit that I get, I'll keep counting it as a win in my head.

## Immune to Volatile Streaming Contracts

Once you purchase and download a song file (assuming you don't lose the file) the file is yours and (assuming it's DRM-free \[enough\]) you can always play back that music! You may have also heard of artists recently pulling music from streaming services, if you acquire the music for yourself, events like these won't effect you. While these events don't happen that often, it's still a nice plus.

# The Cons

## The Price

"Hey, you said it was a pro to get more money to the artists; now you're complaining it's expensive to support those artists, you can't have it both ways!" I can hear you say. Fair point, it's a calculated cost each individual has to make if they are able to, and if it is worth it to them. I am in a financial position where I have the money to try to support artists by buying their music, but I will not pretend it is not expensive. Although, if you find yourself listening to the same music all of the time, this _can_ actually be inverted to a pro for you in the long run. Assuming a $10/month subscription fee, you can do the math on how many years of subscribing to one of the services would equal the albums that you ultimately want and listen to. There is this tension between the cost to you and the financial benefit to the creators, it's up to each individual to decide how they want to balance that out.

## Potentially Less Support for Artists

As I just mentioned, if you mostly just listen to the same music by the same artist, you could end up in a situation where you actually support the artist less over time. If you just buy the one or two albums you like by an artist and just listen to them over and over again, this would be cheaper for you, but the artist will only receive any financial support from you that one time (as opposed to with streaming of those same albums over and over, where they do see _a_ cut for each stream). This is how the industry worked before streaming services, so maybe that's totally fine. I don't want to get too philosophical here, as to which model better serves the artists or the consumers, so I'll leave you to draw your own conclusions here. Given how much new music I buy from new artists, I feel comfortable with my acquiring of music and how that should be sustainable enough for artists.

## Out-of-the-Box Quality-of-Life

Ok, I go a bit deep on this topic, buckle up. It gets pretty personal to me, so you can probably skip this if you're not feeling up for some pretty in-depth analysis of music players that I've tried.

As I've previously stated, I was not entirely happy with any of the (admittedly) free music players that I found and tried. That is what put me on the journey to build the music player that I have been using for the past year-ish. [Jellyfin]() and [VLC for Android]() were both leading contenders for a while. And it's nothing too damning for any of the individual players, just that none of them had _all_ of what I was looking for among the following:
* Organizing the library how I wanted by in-file tagged metadata based on artist/album/etc
	* Some players would provide a file-system based UI, which worked well enough for my music which is pretty neatly organized in the filesystem, but felt just one notch too crude, especially given that I knew the files were tagged with rich metadata that could provide something cleaner
	* Jellyfin and VLC both actually have clean UIs with useful organization and sorting/filtering options!
* Provide decently rich play-queue functionality (append to end, insert next, preempt to play now)
	* Some players would only let me (or make it too easy) to replace the play queue with the selected song (and then just start playing the album from that song forward)
		* Even just accidentally clicking this once or twice was enough to infuriate me, losing what songs I had curated to play next
	* Jellyfin and VLC both satisfy this requirement too, you'll soon see where they didn't quite cross the finish line for me though
* Allow playback everywhere (desktop/laptop/mobile/etc)
	* VLC for Android has the playback on mobile devices (out in the wild with no service, even!) covered
		* But the VLC for Linux/Desktop doesn't have the same organization nor functionality, so it only checks half of this checkbox
	* Jellyfin was what I would use on the desktop before building my app, but failed on this front
		* It does give you a nice experience on a machine in the local network where the media server is installed, so from a desktop or laptop or even a phone in the home, it's great!
		* But, it doesn't have any way to sync the media onto a device that you make take outside of the network (going for a walk with your phone, you've got no access to your library, without opening up access to your home network or some reverse proxy, or something more involved than I was looking to do)
	* Other players also only existed for Android or Linux (my two major platforms), so they mostly have the same conclusions drawn here
* Track play history
	* Not many players would keep track of this, and especially not in the detail that I wanted
	* Jellyfin would record how many times I had listened to each song that I listened to, which is about half of what I wanted
		* I still want to provide auto generated playlists like "Weekend Evenings" and "2010s Winter" that have songs that I often listen to on weekend evenings, or during winter in the 2010s, etc
			* Having the actual date and time that I listened to each track is necessary for this
	* VLC for Android doesn't really have any persistent storage of play history
		* It will keep track of the last ~50 songs I've listened to, but will forget what I have listened to after that

That's been my story so far. If you find something that satisfies all of what you are looking for, congratulations! Just know that you may want to try them out, before you start purchasing music to move off of streaming services.

# Conclusion

For me, I decided that going the route of buying music was what I wanted to go. I'm not trying to convince anyone to do otherwise. Maybe this post plants the seed by highlighting the merits and pain-points. This isn't the solution for everyone and it doesn't have to be. Ok, that's all for now.

The next post will be in a week, and I do plan to go back to technical topics with the media playback in the demo app that we've been building through this blog series. Stay tuned.