#![crate_name = "game2048"]

extern crate sdl2;
extern crate sdl2_ttf;
extern crate sdl2_gfx;
extern crate rand;

use std::env;

mod ui;
mod game;
use std::str::FromStr;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let size : usize = match args.len() {
        1 => 4,
        3 => i64::from_str(args[2].as_ref()).unwrap_or(4) as usize,
        _ => {
            panic!("usage: ./game2048 --size NUM")
        }
    };

    match ui::run(size) {
        Ok(_) => (),
        Err(e) => panic!("Error while running game: {}", e),
    }

}
