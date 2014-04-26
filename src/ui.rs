use sdl2;
use sdl2_gfx;
use ttf = sdl2_ttf;
use sdl2::video;
use sdl2::render;
use sdl2::{event, keycode};
// for Renderer trait
use sdl2_gfx::primitives::DrawRenderer;
use sdl2::pixels::ToColor;
use sdl2::rwops;
use sdl2_ttf::LoaderRWops;


static SCREEN_WIDTH : int = 800;
static SCREEN_HEIGHT : int = 600;

mod game;

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
)

// colors
static BG_COLOR: u32  = 0xeee4daff;
static FG_COLOR: u32 = 0x776e65ff;
static CHAR_COLOR: u32 = 0xFF0000ff;
static CONTAINER_COLOR: u32 = 0x776e65ff;
static CELL_COLORS: &'static [u32] = &'static [
    0xeee4daff, 0xede0c8ff, 0xf2b179ff,
    0xf59564ff, 0xf67c5fff, 0xf65e3bff,
    0xedcf72ff, 0xedcc61ff, 0xedc850ff,
    0xedc53fff, 0xedc22eff, 0x3c3a32ff, ];
static SUPER_CELL_COLOR: u32 = 0xcc33ffff;

// Font
static TTF_FONT_RAW_BYTES: &'static [u8] = include_bin!("./res/OpenDyslexic-Regular.ttf");

#[allow(uppercase_variables, unused_must_use)]
fn draw_game(gm: &mut game::GameManager, ren: &render::Renderer, font: &ttf::Font,
             (x,y,w,h): (int,int,int,int)) -> Result<(), ~str> {
    assert_eq!(w, h);
    // BEST in 500x500
    let SIZE = gm.size;
    let CONTAINER_PADDING: int = 50  / (SIZE as int + 1);
    let CELL_WIDTH = (w - CONTAINER_PADDING * (SIZE as int + 1)) / SIZE as int ;
    assert!(CELL_WIDTH > 40); // Min width
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
        let bx = (x + CONTAINER_PADDING * (j + 1) + CELL_WIDTH * j) as i16;
        let by = (y + CONTAINER_PADDING * (i + 1) + CELL_WIDTH * i) as i16;
        ren.box_(bx, by, bx + CELL_WIDTH as i16, by + CELL_WIDTH as i16, color);
        // ren.string(bx, by, format!("({}, {})", j, i), CHAR_COLOR); // DEBUG
        if val != 0 {
            let (tex, tw, th) = {
                let wd = format!("{}", val);
                let (w, h) = font.size_of_str(wd).ok().expect("size of str");
                let text = font.render_str_blended(wd, FG_COLOR).ok().expect("renderred surface");
                (ren.create_texture_from_surface(text).ok().expect("create texture"), w, h)
            };

            let ratio = if tw > CELL_WIDTH {
                CELL_WIDTH as f64 / tw as f64
            } else if th > CELL_WIDTH {
                CELL_WIDTH as f64 / th as f64
            } else { 1.0 };

            let tw = (tw as f64 * ratio) as int;
            let th = (th as f64 * ratio) as int;

            ren.copy(tex, None, Some(rect!(bx as int + CELL_WIDTH / 2 - tw/2, by as int + CELL_WIDTH / 2 - th/2,
                                               tw, th)));
        }
    });
    Ok(())
}



fn draw_title(ren: &render::Renderer, font: &ttf::Font) -> Result<(), ~str> {
    let (tex2, w, h) = {
        let wd = "Rust - 2048";
        //font.set_style([ttf::StyleBold]);
        let (w, h) = try!(font.size_of_str(wd));
        let text = try!(font.render_str_blended(wd, FG_COLOR));
        (try!(ren.create_texture_from_surface(text)), w, h)
    };
    try!(ren.copy(tex2, None, Some(rect!(SCREEN_WIDTH / 2 - w / 2, 20, w, h))));
    Ok(())
}

// FIXME: tooooooo many type convertion
fn draw_popup(ren: &render::Renderer, font: &ttf::Font, msg: &str) -> Result<(), ~str> {
    let (tex, w, h) = {
        //font.set_style([ttf::StyleBold]);
        let (w, h) = try!(font.size_of_str(msg));
        let text = try!(font.render_str_blended(msg, FG_COLOR));
        (try!(ren.create_texture_from_surface(text)), w, h)
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
    try!(ren.copy(tex, None, Some(rect!(SCREEN_WIDTH / 2 - w / 2,
                                        SCREEN_HEIGHT / 2 - h / 2,
                                        w,
                                        h))));
    Ok(())
}


pub fn run(game_size: uint) -> Result<(), ~str> {
    let win = try!(video::Window::new(
        "Rust - 2048", video::PosCentered, video::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT,
        video::Shown));
    let ren = try!(render::Renderer::from_window(
        win, render::DriverAuto, render::Accelerated));

    let mut fpsm = sdl2_gfx::framerate::FPSManager::new();
    try!(fpsm.set_framerate(50));


    let font : ~ttf::Font = {
        let raw = try!(rwops::RWops::from_bytes(TTF_FONT_RAW_BYTES));
        // or try!(ttf::Font::from_file(&Path::new("./OpenDyslexic-Regular.ttf"), 48));
        try!(raw.load_font(48))
    };
    let mut gm = game::GameManager::new(game_size);

    let mut playing = false;
    let mut celebrating = false;

    'main : loop {
        'event : loop {
            fpsm.delay();
            try!(ren.set_draw_color(BG_COLOR.to_color()));
            try!(ren.clear());
            // == main drawing ==
            try!(draw_title(&*ren, &*font));
            try!(ren.string(0i16, 0i16, format!("frames: {}", fpsm.get_frame_count()), CHAR_COLOR));

            try!(ren.string(200, 90, format!("your score: {}", gm.score), CHAR_COLOR));

            try!(draw_game(&mut gm, &*ren, &*font, (SCREEN_WIDTH / 2 - 400 / 2, 100, 400, 400)));

            if celebrating || (playing && !gm.moves_available()) { // can't move
                try!(draw_popup(&*ren, &*font, format!("Score: {}! Max Cell: {}", gm.score, "NaN")));
                playing = false;
                celebrating = true;

            } else if !playing && !celebrating {
                try!(draw_popup(&*ren, &*font, "Press SPACE to start!"));
            }

            // == main drawing ends ==
            ren.present();

            match event::poll_event() {
                event::QuitEvent(_) => break 'main,
                event::KeyDownEvent(_, _, keycode::LeftKey, _, _) if playing => {
                    gm.move(game::Left);
                }
                event::KeyDownEvent(_, _, keycode::RightKey, _, _) if playing => {
                    gm.move(game::Right);
                }
                event::KeyDownEvent(_, _, keycode::UpKey, _, _) if playing => {
                    gm.move(game::Up);
                }
                event::KeyDownEvent(_, _, keycode::DownKey, _, _) if playing => {
                    gm.move(game::Down);
                }
                event::KeyDownEvent(_, _, key, _, _) => {
                    if key == keycode::EscapeKey {
                        break 'main
                    } else if key == keycode::SpaceKey {
                        if !playing {
                            playing = true;
                            celebrating = false;
                            gm.setup();
                        }
                    }

                }
                event::MouseButtonDownEvent(_, _, _, _, x, y) => {
                    println!("mouse btn down at ({},{})", x, y);
                }

                _ => {}
            }
        }
    }
    Ok(())
}
