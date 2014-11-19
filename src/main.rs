#![crate_name = "game2048"]
#![crate_type = "bin"]

#![feature(globs, macro_rules)]

extern crate sdl2;
// extern crate sdl2_image;
extern crate sdl2_ttf;
extern crate sdl2_gfx;

use std::os;

#[allow(dead_code)]
mod ui;
mod game;


fn main() {
    let args = os::args();

    sdl2::init(sdl2::INIT_VIDEO);
    // sdl2_image::init([sdl2_image::InitPng, sdl2_image::InitJpg]);
    sdl2_ttf::init();

    let size : uint = match args.len() {
        1 => 4,
        3 => from_str(args[2].as_slice()).unwrap_or(4),
        _ => {
            panic!("usage: ./game2048 --size NUM")
        }
    };

    match ui::run(size) {
        Ok(_) => (),
        Err(e) => panic!("Error while running game: {}", e),
    }

    sdl2_ttf::quit();
    // sdl2_image::quit();
    sdl2::quit();
}
