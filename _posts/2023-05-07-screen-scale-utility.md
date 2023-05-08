---
title: Screen Scale Utility
date: 2023-05-07
---

# Introduction

As state in the [Miscellaneous May](/2023-05-07-misc-may) post, this post won't be about anything music related, but instead is about a utility I wrote to change the screen scale factor in [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) on [Linux](https://en.wikipedia.org/wiki/Linux).

This post is an overview of my `swayscreenctl` project. It satisfies a need that I have on my laptop with a relatively high DPI screen: sometimes I want all of the elements to be very large as I'm relatively far away from the screen, and sometimes I want to use a split screen and need everything as small as it can get, and some other times I want something in between. To make my life easier trying to swap between the different "resolutions" for the screen, I've written the following scripts and utilities; let's take a look!

# `swayscreenctl`

## Knowledge Pre-Requisites

### What is Sway?

I use the [`sway`](https://swaywm.org/) windowing manager built on the [`wayland`](https://wayland.freedesktop.org/) compositor.

One can interface with sway through the `swaymsg` command line utility, which can send messages to the running instance of sway to query for and update attributes, including screen resolution and hi-dpi scaling (which is what we want to leverage today).

### Examples of `swaymsg`

One thing we can do with `swaymsg` is to query for all of the current outputs, which for me, runs as such:

```sh
 $ swaymsg -t get_outputs
Output eDP-1 'Unknown 0x095F 0x00000000' (focused)
  Current mode: 2256x1504 @ 59.999 Hz
  Position: 0,0
  Scale factor: 2.000000
  Scale filter: nearest
  Subpixel hinting: rgb
  Transform: normal
  Workspace: 2
  Max render time: off
  Adaptive sync: disabled
  Available modes:
    2256x1504 @ 47.998 Hz
    2256x1504 @ 59.999 Hz
```

Note in particular the `Scale factor: 2.0`.

With the laptop that I'm using, it is easiest to read when the screen is scaled by some value greater than 1.

For example, when scaled by 2, the effective screen would be:
1128x752 pixels instead of the native 2256x1504.

As we'll use the following in the future, here is an example of setting the scale factor:

```sh
 $ swaymsg output eDP-1 scale 1
```

Which would change the scale factor back to 1, so that the effective screen would be back to the full:
2256x1504 pixels.

## Desired End Result and User Experience

I wanted a way to control the scale factor from some combination of hotkeys (maybe the media keys with some modifiers?).

To start I knew that I would need some set of scale factors that I wanted to toggle between, and some way to move up and down through those values.

I had a collection of values that I had been manually swapping between; simplified for this summary, these numbers are useful scale factors to move between:

```
1
1.41
1.7625
2
```

These multipliers mostly end up with nice pixel values for the width and height of my screen, and are spaced far enough apart from each other that they feel like useful jumps, while not being too jarring to jump between.

## The Rust Code

I knew that I would want some simple Rust code that would jump between the scale factors, likely given a list of scale factors and whether to go up or down in that list of scale factors.

I ended up with:
 - The enum `Direction` which signalled to move up or down (or to the bottom or the top)
	 - A parser that tried to parse this enum from string representations
 - A function `closest_up`
	 - which moves to the next largest value from a given point
 - A function `closest_down`
	 - which moves to the next smallest value from a given point
 - A function `resolve_new_scale`
	 - which selects the next value in a list, up or down, from a given value
 - A function `resolve_screen_scales_from_config_file`
	 - which parses out all of the screen values to move between from a config file
 - A `main` function
	 - that parses the inputs from the command line and calls into the appropriate functions above, the command line args being:
		 - the direction
		 - the current scale factor
		 - the config file to use
   - this function then prints out the scale factor to set to `stdout`, which the consumer of this will expect

Here is example usage of this Rust code, and then the code below that:

(with the following config file)

### **`~/.config/swayscreenctl/small`**
```
 $ cat ~/.config/swayscreenctl/small
1
1.41
1.7625
2
```

### **`shell`** demo
```sh
 $ rust_screen_scale down 1.6 ~/.config/swayscreenctl/small
1.41

 $ rust_screen_scale up 1.6 ~/.config/swayscreenctl/small
1.7625
```

With the directions "up" and "down" from a "current value" of 1.6 and the "small" config file.

The Final Rust Code:

### **`main.rs`**
```rust
use std::env;
use std::fs;
use std::num;

pub enum Direction {
    Bottom,
    Down,
    Up,
    Top,
}

impl Direction {
    pub fn parse(s: &String) -> Result<Direction, String> {
        match s.as_str() {
            "bottom" => Ok(Direction::Bottom),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            "top" => Ok(Direction::Top),
            _ => Err("<direction> must be one of 'up' or 'down'".to_string()),
        }
    }
}

fn closest_up(values: Vec<f32>, current: f32) -> f32 {
    for value in values.iter() {
        if current < *value {
            return value.clone();
        }
    }
    return values[values.len() - 1];
}

fn closest_down(values: Vec<f32>, current: f32) -> f32 {
    for value in values.iter().rev() {
        if current > *value {
            return value.clone();
        }
    }
    return values[0];
}

fn resolve_new_scale(direction: Direction, current_scale: f32, mut scales: Vec<f32>) -> f32 {
    scales.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match direction {
        Direction::Bottom => scales[0],
        Direction::Down => closest_down(scales, current_scale),
        Direction::Up => closest_up(scales, current_scale),
        Direction::Top => scales[scales.len() - 1],
    }
}

fn resolve_screen_scales_from_config_file(config_file_path: String) -> Result<Vec<f32>, String> {
    let file_contents = fs::read_to_string(config_file_path).map_err(|e| format!("{:?}", e))?;
    let scales = file_contents
        .split("\n")
        .into_iter()
        .filter(|line| line.len() != 0)
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.parse::<f32>())
        .collect::<Result<Vec<f32>, num::ParseFloatError>>()
        .map_err(|e| format!("{:?}", e))?;
    return Ok(scales);
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err("Must supply <direction>, <current-scale>, and <config-file-path>".to_string());
    }

    let direction = Direction::parse(&args[1])?;
    let scale = args[2]
        .parse::<f32>()
        .map_err(|_| "<current-scale> must be a floating point number".to_string())?;
    let config_file_path = args[3].clone();

    let scales = resolve_screen_scales_from_config_file(config_file_path)?;

    let new_scale = resolve_new_scale(direction, scale, scales);

    println!("{}", new_scale);

    return Ok(());
}
```

## Bash Code (Built On Top Of The Rust Code)

I didn't really want to deal with subprocesses or including crates to communicate with sway directly, so I decided to make the executable that I actually call into a shell script that calls into this Rust code.

This bash code:
* Takes in the display to operate on
* Takes the operation (up, down, etc)
* Resolves a default config file name
* Queries for the current screen scale

And then calls into the Rust code appropriately and sets the scale with the output it gets from the Rust code.

The only other pieces of noteworth information here being:
* It can just `get` the current screen scale if the user provides the `get` argument
* It has a default config file if the caller does not provide a config file
* It will print out the current effective-resolution and scale after it is done setting the scale (or passing if it was told to `get`)

Some Example usage of this bash code would be (from a current scale factor of 1.6, again):

### **`shell`** `swayscreenctl` demo
```sh
 $ swayscreenctl down ~/.config/swayscreenctl/small
1600x1066 (1.409999966621399)

 $ swayscreenctl up ~/.config/swayscreenctl/small
1280x853 (1.7625000476837158)
```

And the Final Bash Code looks like this:

### **`swayscreenctl`**
 ```bash
#!/bin/bash

set -e
#set -x

if [ $# -lt 2 ]; then
    echo "must supply <display> <command> (e.g. up, down, bottom, top, get)"
    exit
fi

DISPLAY=$1
OPERATION=$2

CURRENT_SCALE=$(swaymsg -t get_outputs | jq ".[] | select(.name == \"${DISPLAY}\") | .scale")


if [ $OPERATION != "get" ] && [ $OPERATION != "get-simple" ]; then

    if [ $OPERATION == "select" ]; then
        SCALE_TO_SET=$3
    else
        CONFIG_FILE=~/.config/swayscreenctl/config

        if [ $# -eq 3 ]; then
            CONFIG_FILE=$3
        fi

        SCALE_TO_SET=$(rust_screen_scale $OPERATION $CURRENT_SCALE $CONFIG_FILE)
    fi

    swaymsg output ${DISPLAY} scale $SCALE_TO_SET

    pkill -SIGRTMIN+2 i3status-rs
fi

CURRENT_SWAY_OUTPUT=$(swaymsg -t get_outputs | jq ".[] | select(.name == \"${DISPLAY}\")")
CURRENT_SCALE=$(echo $CURRENT_SWAY_OUTPUT | jq '.scale')
CURRENT_X=$(echo $CURRENT_SWAY_OUTPUT | jq '.rect.width')
CURRENT_Y=$(echo $CURRENT_SWAY_OUTPUT | jq '.rect.height')

if [ $OPERATION == "get-simple" ]; then
    echo "${CURRENT_X}x${CURRENT_Y}"
else
    echo "${CURRENT_X}x${CURRENT_Y} ($(printf "%.4f" ${CURRENT_SCALE}))"
fi
```

## Sway Bindings

After we have all of this logic, we want to be able to use it from the most convenient place of all: the keyboard!

I bound to the brightness media keys with modifiers to call into the bash script a few different ways.

I knew that I wanted to be able to move up and down through some reasonable values, which I set as the default `config`:

#### **`~/.config/swayscreenctl/config`
```
1
1.175
1.327
1.6
1.88
2
```

But I also figured that there may be more resolutions that I want to hop between, so I built this larger list of scales that may be useful to scan through:

### **`~/.config/swayscreenctl/large`**
```
1
1.128
1.175
#1.2
1.25
#1.3
1.327
1.41
#1.46875
#1.5
1.504
1.6
1.7625
#1.8048
1.88
#1.92
2
2.256
2.4
2.5
2.82
3
3.2
3.5
3.76
4
5
6
7.52
8
9.024
```

Note that some values are commented out with the leading `#`, they are just there because they are values that work reasonably well as scale factors, but I didn't need to move between that many values even with "full control". They're there just in case I want them in the future.

With all of this in mind, I leverage the default config resolution of the script to resolve
to the short list of scales, and only specify the large list of scales when I want the finer-grain control.

The end result is the following bindings:

#### **`~/.config/sway/config.d/20_bindings.conf`**
```config
### Screen Hi-DPI Scale
bindsym $mod+XF86MonBrightnessUp                 exec swayscreenctl eDP-1 up
bindsym $mod+XF86MonBrightnessDown               exec swayscreenctl eDP-1 down

bindsym $mod+Shift+XF86MonBrightnessUp           exec swayscreenctl eDP-1 up ~/.config/swayscreenctl/large
bindsym $mod+Shift+XF86MonBrightnessDown         exec swayscreenctl eDP-1 down ~/.config/swayscreenctl/large

bindsym $mod+Control+XF86MonBrightnessUp         exec swayscreenctl eDP-1 top
bindsym $mod+Control+XF86MonBrightnessDown       exec swayscreenctl eDP-1 bottom
```

## Summary

I wrote this script one day when I was tired of manually swapping the scale factor, and it was a fun project for me to tackle. Plus I still use this scale factor navigation on the daily, which I'm happy with! Hopefully you enjoyed a bit lighter of (still admittedly technical) content. If anyone asks for it, I can upload the rust and bash scripts into a git repository so others can use this if they like.

Stay tuned for next week and I may show off my `sonosctl` script!