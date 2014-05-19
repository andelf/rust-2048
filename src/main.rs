#![crate_id = "game2048"]
#![crate_type = "bin"]

#![feature(globs, macro_rules)]

extern crate rand;
extern crate sdl2;
// extern crate sdl2_image;
extern crate sdl2_ttf;
extern crate sdl2_gfx;

use std::os;
use std::from_str::from_str;

#[allow(dead_code)]
mod ui;


fn main() {
    let args = os::args();

    sdl2::init(sdl2::InitVideo);
    // sdl2_image::init([sdl2_image::InitPng, sdl2_image::InitJpg]);
    sdl2_ttf::init();

    let size : uint = match args.len() {
        1 => 4,
        3 => from_str(*args.get(2)).unwrap_or(4),
        _ => {
            fail!("usage: ./game2048 --size NUM")
        }
    };

    match ui::run(size) {
        Ok(_) => (),
        Err(e) => fail!("Error while running game: {}", e),
    }

    sdl2_ttf::quit();
    // sdl2_image::quit();
    sdl2::quit();
}
