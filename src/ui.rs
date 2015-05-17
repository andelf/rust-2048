use std::path::Path;
use sdl2;
use sdl2_gfx;
use sdl2_ttf;
use sdl2::video;
use sdl2::video::{WindowPos};
use sdl2::render;
use sdl2::render::{RenderDriverIndex};
use sdl2::keycode::KeyCode;

use sdl2::event::Event;
// for Renderer trait
use sdl2_gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::SdlResult;
use sdl2::rwops;
use sdl2_ttf::LoaderRWops;
use game;
use game::Direction;

static SCREEN_WIDTH : i32 = 800;
static SCREEN_HEIGHT : i32 = 600;

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
);

// colors
static BG_COLOR: Color = Color::RGB(0xee, 0xe4, 0xda);
static FG_COLOR: Color = Color::RGB(0x77, 0x6e, 0x65);
static CHAR_COLOR: Color = Color::RGB(0xee, 0x33, 0x66);
static CONTAINER_COLOR: Color = Color::RGBA(0x77, 0x6e, 0x65, 200);
static CELL_COLORS: &'static [Color] = &[
    Color::RGBA(0xee, 0xe4, 0xda, 120), Color::RGB(0xed, 0xe0, 0xc8), Color::RGB(0xf2, 0xb1, 0x79),
    Color::RGB(0xf5, 0x95, 0x64), Color::RGB(0xf6, 0x7c, 0x5f), Color::RGB(0xf6, 0x5e, 0x3b),
    Color::RGB(0xed, 0xcf, 0x72), Color::RGB(0xed, 0xcc, 0x61), Color::RGB(0xed, 0xc8, 0x50),
    Color::RGB(0xed, 0xc5, 0x3f), Color::RGB(0xed, 0xc2, 0x2e), Color::RGB(0x3c, 0x3a, 0x32), ];
static SUPER_CELL_COLOR: Color = Color::RGB(0xcc, 0x33, 0xff);

// Font
#[cfg(target_os="macos")]
static UNDER_MACOSX: bool = true;

#[cfg(any(target_os="windows", target_os="linux", target_os="freebsd"))]
static UNDER_MACOSX: bool = false;

#[allow(unused_must_use)]
fn draw_game(gm: &mut game::GameManager, ren: &mut render::Renderer, font: &sdl2_ttf::Font,
                (x,y,w,h): (i32,i32,i32,i32)) -> SdlResult<()> {
    assert_eq!(w, h);
    // BEST in 500x500
    let size = gm.size;
    let container_padding: i32 = 50  / (size as i32 + 1);
    let cell_width = (w - container_padding * (size as i32 + 1)) / size as i32 ;
    assert!(cell_width > 40); // Min width
    try!(ren.box_(x as i16, y as i16, (x+w) as i16, (y+h) as i16, CONTAINER_COLOR));
    gm.grid.each_cell(|j, i, tile_opt| {
        let i = i as i32;
        let j = j as i32;
        let val = match tile_opt {
            Some(ref tile) => tile.value,
            None           => 0,
        };
        let c = if val == 0 {
            0           // or will be +Infinity
        } else {
            (val as f64).log2() as usize
        };
        let color = CELL_COLORS.get(c).map(|&co| co).unwrap_or(SUPER_CELL_COLOR);
        let bx = (x + container_padding * (j + 1) + cell_width * j) as i16;
        let by = (y + container_padding * (i + 1) + cell_width * i) as i16;
        ren.box_(bx, by, bx + cell_width as i16, by + cell_width as i16, color);
        // ren.string(bx, by, format!("({}, {})", j, i), CHAR_COLOR); // DEBUG
        if val != 0 {
            let (tex, tw, th) = {
                let wd = format!("{}", val);
                let (w, h) = font.size_of_str(wd.as_ref()).ok().expect("size of str");
                let text = font.render_str_blended(wd.as_ref(), FG_COLOR).ok().expect("renderred surface");
                (ren.create_texture_from_surface(&text).ok().expect("create texture"), w, h)
            };

            let ratio = if tw > cell_width {
                cell_width as f64 / tw as f64
            } else if th > cell_width {
                cell_width as f64 / th as f64
            } else { 1.0 };

            let tw = (tw as f64 * ratio) as i32;
            let th = (th as f64 * ratio) as i32;

            ren.drawer().copy(&tex, None, Some(rect!(bx as i32 + cell_width / 2 - tw/2, by as i32 + cell_width / 2 - th/2,
                                            tw, th)));
        }
    });
    Ok(())
}

fn draw_title(ren: &mut render::Renderer, font: &sdl2_ttf::Font) -> SdlResult<()> {
    let (tex2, w, h) = {
        let wd = "Rust - 2048";
        let (w, h) = try!(font.size_of_str(wd));
        let text = try!(font.render_str_blended(wd, FG_COLOR));
        (ren.create_texture_from_surface(&text).unwrap(), w, h)
    };
    ren.drawer().copy(&tex2, None, Some(rect!(SCREEN_WIDTH / 2 - w / 2, 20, w, h)));
    Ok(())
}

// FIXME: tooooooo many type convertion
fn draw_popup(ren: &mut render::Renderer, font: &sdl2_ttf::Font, msg: &str) -> SdlResult<()> {
    let (tex, w, h) = {
        let (w, h) = try!(font.size_of_str(msg));
        let text = try!(font.render_str_blended(msg, FG_COLOR));
        (try!(ren.create_texture_from_surface(&text)), w, h)
    };
    ren.rounded_box((SCREEN_WIDTH / 2 - w / 2) as i16,
                    (SCREEN_HEIGHT / 2 - h / 2) as i16,
                    (SCREEN_WIDTH / 2 + w / 2) as i16,
                    (SCREEN_HEIGHT / 2 + h / 2) as i16,
                    5,
                    BG_COLOR).unwrap();
    ren.rounded_rectangle((SCREEN_WIDTH / 2 - w / 2) as i16,
                          (SCREEN_HEIGHT / 2 - h / 2) as i16,
                          (SCREEN_WIDTH / 2 + w / 2) as i16,
                          (SCREEN_HEIGHT / 2 + h / 2) as i16,
                          5,
                          FG_COLOR).unwrap();
    ren.drawer().copy(&tex, None, Some(rect!(SCREEN_WIDTH / 2 - w / 2, SCREEN_HEIGHT / 2 - h / 2,
                                             w, h)));
    Ok(())
}

#[allow(non_shorthand_field_patterns)]
pub fn run(game_size: usize) -> SdlResult<()> {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
    sdl2_ttf::init();

    let win = try!(video::Window::new(
        &sdl_context,
        "Rust - 2048", WindowPos::PosCentered, WindowPos::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT,
        video::SHOWN));
    let mut ren = try!(render::Renderer::from_window(
        win, RenderDriverIndex::Auto, render::ACCELERATED));

    let mut fpsm = sdl2_gfx::framerate::FPSManager::new();
    try!(fpsm.set_framerate(50));

    let font = if UNDER_MACOSX {
        try!(sdl2_ttf::Font::from_file(&Path::new("/System/Library/Fonts/HelveticaNeueDeskInterface.ttc"), 48))
    } else {
        let raw_ttf_bytes: &'static [u8] = include_bytes!("./res/OpenDyslexic-Regular.ttf");
        let raw = try!(rwops::RWops::from_bytes(raw_ttf_bytes));
        try!(raw.load_font(48))
    };

    // font.set_style(sdl2_ttf::STYLE_BOLD);

    let mut gm = game::GameManager::new(game_size);

    let mut playing = false;
    let mut celebrating = false;

    let mut event_pump = sdl_context.event_pump();

    'main : loop {
        'event : for event in event_pump.poll_iter() {
            fpsm.delay();
            ren.drawer().set_draw_color(BG_COLOR);
            ren.drawer().clear();
            // == main drawing ==
            draw_title(&mut ren, &font).unwrap();
            try!(ren.string(0i16, 0i16, format!("frames: {}", fpsm.get_frame_count()).as_ref(), CHAR_COLOR));

            try!(ren.string(200, 90, format!("your score: {}", gm.score).as_ref(), CHAR_COLOR));

            try!(draw_game(&mut gm, &mut ren, &font, (SCREEN_WIDTH / 2 - 400 / 2, 100, 400, 400)));

            if celebrating || (playing && !gm.moves_available()) { // can't move
                try!(draw_popup(&mut ren, &font, format!("Score: {}! Max Cell: {}", gm.score, "NaN").as_ref()));
                playing = false;
                celebrating = true;

            } else if !playing && !celebrating {
                draw_popup(&mut ren, &font, "Press SPACE to start!").unwrap();
            }

            // == main drawing ends ==
            ren.drawer().present();
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: KeyCode::Left, ..} if playing => {
                    gm.move_to(Direction::Left);
                }
                Event::KeyDown {keycode: KeyCode::Right, ..} if playing => {
                    gm.move_to(Direction::Right);
                }
                Event::KeyDown {keycode: KeyCode::Up, ..} if playing => {
                    gm.move_to(Direction::Up);
                }
                Event::KeyDown {keycode: KeyCode::Down, ..} if playing => {
                    gm.move_to(Direction::Down);
                }
                Event::KeyDown {keycode: key, ..} => {
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
                Event::MouseButtonDown {x: x, y: y, ..} => {
                    println!("mouse btn down at ({},{})", x, y);
                }

                _ => {}
            }
        }
    }
    Ok(())
}
