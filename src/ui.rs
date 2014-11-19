use std::num::Float;
use sdl2;
use sdl2_gfx;
use sdl2_ttf;
use sdl2::video;
use sdl2::render;
use sdl2::render::{RenderDriverIndex};
use sdl2::keycode::KeyCode;

use sdl2::event;
use sdl2::event::Event;
// for Renderer trait
use sdl2_gfx::primitives::DrawRenderer;
use sdl2::pixels::{Color, RGB, RGBA};
use sdl2::rwops;
use sdl2_ttf::LoaderRWops;
use sdl2::SdlResult;
use game;
use game::Direction;

static SCREEN_WIDTH : int = 800;
static SCREEN_HEIGHT : int = 600;

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
)

// colors
static BG_COLOR: Color = RGB(0xee, 0xe4, 0xda);
static FG_COLOR: Color = RGB(0x77, 0x6e, 0x65);
static CHAR_COLOR: Color = RGB(0xee, 0x33, 0x66);
static CONTAINER_COLOR: Color = RGBA(0x77, 0x6e, 0x65, 200);
static CELL_COLORS: &'static [Color] = &[
    RGBA(0xee, 0xe4, 0xda, 120), RGB(0xed, 0xe0, 0xc8), RGB(0xf2, 0xb1, 0x79),
    RGB(0xf5, 0x95, 0x64), RGB(0xf6, 0x7c, 0x5f), RGB(0xf6, 0x5e, 0x3b),
    RGB(0xed, 0xcf, 0x72), RGB(0xed, 0xcc, 0x61), RGB(0xed, 0xc8, 0x50),
    RGB(0xed, 0xc5, 0x3f), RGB(0xed, 0xc2, 0x2e), RGB(0x3c, 0x3a, 0x32), ];
static SUPER_CELL_COLOR: Color = RGB(0xcc, 0x33, 0xff);

// Font
static TTF_FONT_RAW_BYTES: &'static [u8] = include_bin!("./res/OpenDyslexic-Regular.ttf");

#[allow(unused_must_use)]
fn draw_game(gm: &mut game::GameManager, ren: &render::Renderer, font: &sdl2_ttf::Font,
                (x,y,w,h): (int,int,int,int)) -> SdlResult<()> {
    assert_eq!(w, h);
    // BEST in 500x500
    let size = gm.size;
    let container_padding: int = 50  / (size as int + 1);
    let cell_width = (w - container_padding * (size as int + 1)) / size as int ;
    assert!(cell_width > 40); // Min width
    try!(ren.box_(x as i16, y as i16, (x+w) as i16, (y+h) as i16, CONTAINER_COLOR));
    gm.grid.each_cell(|j, i, tile_opt| {
        let i = i as int;
        let j = j as int;
        let val = match tile_opt {
            Some(&tile) => tile.value,
            None        => 0,
        };
        let c = if val == 0 {
            0           // or will be +Infinity
        } else {
            (val as f64).log2() as uint
        };
        let color = CELL_COLORS.get(c).map(|&co| co).unwrap_or(SUPER_CELL_COLOR);
        let bx = (x + container_padding * (j + 1) + cell_width * j) as i16;
        let by = (y + container_padding * (i + 1) + cell_width * i) as i16;
        ren.box_(bx, by, bx + cell_width as i16, by + cell_width as i16, color);
        // ren.string(bx, by, format!("({}, {})", j, i), CHAR_COLOR); // DEBUG
        if val != 0 {
            let (tex, tw, th) = {
                let wd = format!("{}", val);
                let (w, h) = font.size_of_str(wd.as_slice()).ok().expect("size of str");
                let text = font.render_str_blended(wd.as_slice(), FG_COLOR).ok().expect("renderred surface");
                (ren.create_texture_from_surface(&text).ok().expect("create texture"), w, h)
            };

            let ratio = if tw > cell_width {
                cell_width as f64 / tw as f64
            } else if th > cell_width {
                cell_width as f64 / th as f64
            } else { 1.0 };

            let tw = (tw as f64 * ratio) as int;
            let th = (th as f64 * ratio) as int;

            ren.copy(&tex, None, Some(rect!(bx as int + cell_width / 2 - tw/2, by as int + cell_width / 2 - th/2,
                                            tw, th)));
        }
    });
    Ok(())
}



fn draw_title(ren: &render::Renderer, font: &sdl2_ttf::Font) -> SdlResult<()> {
    let (tex2, w, h) = {
        let wd = "Rust - 2048";
        //font.set_style([sdl2_ttf::StyleBold]);
        let (w, h) = try!(font.size_of_str(wd));
        let text = try!(font.render_str_blended(wd, FG_COLOR));
        (try!(ren.create_texture_from_surface(&text)), w, h)
    };
    try!(ren.copy(&tex2, None, Some(rect!(SCREEN_WIDTH / 2 - w / 2, 20, w, h))));
    Ok(())
}

// FIXME: tooooooo many type convertion
fn draw_popup(ren: &render::Renderer, font: &sdl2_ttf::Font, msg: &str) -> SdlResult<()> {
    let (tex, w, h) = {
        //font.set_style([sdl2_ttf::StyleBold]);
        let (w, h) = try!(font.size_of_str(msg));
        let text = try!(font.render_str_blended(msg, FG_COLOR));
        (try!(ren.create_texture_from_surface(&text)), w, h)
    };
    try!(ren.rounded_box((SCREEN_WIDTH / 2 - w / 2) as i16,
                         (SCREEN_HEIGHT / 2 - h / 2) as i16,
                         (SCREEN_WIDTH / 2 + w / 2) as i16,
                         (SCREEN_HEIGHT / 2 + h / 2) as i16,
                         5,
                         BG_COLOR));
    try!(ren.rounded_rectangle((SCREEN_WIDTH / 2 - w / 2) as i16,
                               (SCREEN_HEIGHT / 2 - h / 2) as i16,
                               (SCREEN_WIDTH / 2 + w / 2) as i16,
                               (SCREEN_HEIGHT / 2 + h / 2) as i16,
                               5,
                               FG_COLOR));
    try!(ren.copy(&tex, None, Some(rect!(SCREEN_WIDTH / 2 - w / 2,
                                        SCREEN_HEIGHT / 2 - h / 2,
                                        w,
                                        h))));
    Ok(())
}


pub fn run(game_size: uint) -> SdlResult<()> {
    let win = try!(video::Window::new(
        "Rust - 2048", video::PosCentered, video::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT,
        video::SHOWN));
    let ren = try!(render::Renderer::from_window(
        win, RenderDriverIndex::Auto, render::ACCELERATED));

    let mut fpsm = sdl2_gfx::framerate::FPSManager::new();
    try!(fpsm.set_framerate(50));


    let font = {
        let raw = try!(rwops::RWops::from_bytes(TTF_FONT_RAW_BYTES));
        // or try!(sdl2_ttf::Font::from_file(&Path::new("./OpenDyslexic-Regular.ttf"), 48));
        try!(raw.load_font(48))
    };
    let mut gm = game::GameManager::new(game_size);

    let mut playing = false;
    let mut celebrating = false;

    'main : loop {
        'event : loop {
            fpsm.delay();
            try!(ren.set_draw_color(BG_COLOR));
            try!(ren.clear());
            // == main drawing ==
            try!(draw_title(&ren, &font));
            try!(ren.string(0i16, 0i16, format!("frames: {}", fpsm.get_frame_count()).as_slice(), CHAR_COLOR));

            try!(ren.string(200, 90, format!("your score: {}", gm.score).as_slice(), CHAR_COLOR));

            try!(draw_game(&mut gm, &ren, &font, (SCREEN_WIDTH / 2 - 400 / 2, 100, 400, 400)));

            if celebrating || (playing && !gm.moves_available()) { // can't move
                try!(draw_popup(&ren, &font, format!("Score: {}! Max Cell: {}", gm.score, "NaN").as_slice()));
                playing = false;
                celebrating = true;

            } else if !playing && !celebrating {
                try!(draw_popup(&ren, &font, "Press SPACE to start!"));
            }

            // == main drawing ends ==
            ren.present();

            match event::poll_event() {
                Event::Quit(_) => break 'main,
                Event::KeyDown(_, _, KeyCode::Left, _, _) if playing => {
                    gm.move_to(Direction::Left);
                }
                Event::KeyDown(_, _, KeyCode::Right, _, _) if playing => {
                    gm.move_to(Direction::Right);
                }
                Event::KeyDown(_, _, KeyCode::Up, _, _) if playing => {
                    gm.move_to(Direction::Up);
                }
                Event::KeyDown(_, _, KeyCode::Down, _, _) if playing => {
                    gm.move_to(Direction::Down);
                }
                Event::KeyDown(_, _, key, _, _) => {
                    if key == KeyCode::Escape {
                        break 'main
                    } else if key == KeyCode::Space {
                        if !playing {
                            playing = true;
                            celebrating = false;
                            gm.setup();
                        }
                    }

                }
                Event::MouseButtonDown(_, _, _, _, x, y) => {
                    println!("mouse btn down at ({},{})", x, y);
                }

                _ => {}
            }
        }
    }
    Ok(())
}
