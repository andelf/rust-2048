#![crate_id = "game2048"]
#![crate_type = "bin"]

#![feature(globs, macro_rules)]

extern crate rand;
extern crate sdl2;
// extern crate sdl2_image;
extern crate sdl2_ttf;
extern crate sdl2_gfx;

#[allow(dead_code)]
mod game;

fn main() {
    sdl2::init([sdl2::InitVideo]);
    // sdl2_image::init([sdl2_image::InitPng, sdl2_image::InitJpg]);
    sdl2_ttf::init();

    match game::run() {
        Ok(_) => (),
        Err(e) => fail!("Error while running game: {}", e),
    }

    sdl2_ttf::quit();
    // sdl2_image::quit();
    sdl2::quit();
}
