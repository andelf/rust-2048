# rust-2048

[![Build Status](https://travis-ci.org/andelf/rust-2048.svg?branch=master)](https://travis-ci.org/andelf/rust-2048)
[![Build Status](https://drone.io/github.com/andelf/rust-2048/status.png)](https://drone.io/github.com/andelf/rust-2048/latest)

2048 game in rust. Original is in [2048](http://gabrielecirulli.github.io/2048/).

Working in Progress.

## How to Install/Play

Refer ``.travis.yml``. :)

## TODO

* fix bugs in moving cell
* rewrite game logic instead of copy from paraze/2048-rs
* AI
* Animation

## Overview

* SDL2 ui ( [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2), [rust-sdl2_ttf](https://github.com/andelf/rust-sdl2_ttf),
  [rust-sdl2_gfx](https://github.com/andelf/rust-sdl2_gfx) )
* Copied 2048 code from https://github.com/paraze/2048-rs , works, but has some bug in move and merging.

![rust-2048 screenshot 1][ss01]

![rust-2048 screenshot 2][ss02]

[ss01]: http://i.imgur.com/fv1Y3PJ.png
[ss02]: http://i.imgur.com/Q9VlhXD.png
