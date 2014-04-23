use rand;
use std::iter::DoubleEndedIterator;
use std::fmt;


#[deriving(Eq, Show)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vector(self) -> (int, int) {
        match self {
            Up    => (0, -1),
            Right => (1, 0),
            Down  => (0, 1),
            Left  => (-1, 0)
        }
    }

    // Haskell succ/pred???
    fn all_directions() -> ~[Direction] {
        ~[Up, Right, Down, Left]
    }

}

pub struct Traversal {
    xs: ~[uint],
    ys: ~[uint],

    idx: uint,
    max_idx: uint,
    size: uint
}

impl Traversal {
    pub fn new(size: uint, dir: Direction) -> Traversal {
        let (x, y) = dir.to_vector();
        let mut xs = range(0u, size).collect::<Vec<uint>>();
        let mut ys = range(0u, size).collect::<Vec<uint>>();
        if x == 1 {
            xs.reverse()
        }
        if y == 1 {
            ys.reverse()
        }
        Traversal { xs: xs.as_slice().into_owned(), ys: ys.as_slice().into_owned(),
                    idx: 0, max_idx: size * size,
                    size: size,
        }
    }
}

impl Iterator<(uint, uint)> for Traversal {
    fn next(&mut self) -> Option<(uint, uint)> {
        if self.idx == self.max_idx {
            None
        } else {
            let ret = (self.xs[self.idx / self.size],
                       self.ys[self.idx % self.size]);
            self.idx += 1;
            Some(ret)
        }
    }
}

#[deriving(Eq, Clone)]
pub struct Tile {
    pub x: uint,
    pub y: uint,
    pub value: int,

    pub prev_pos: Option<(uint, uint)>,
    pub merged_from: Option<((uint,uint), (uint, uint))>
}

impl Tile {
    pub fn new((x, y): (uint, uint), value: int) -> Tile {
        Tile { x: x, y: y, value: value,
               prev_pos: None, merged_from: None }
    }

    pub fn save_position(&mut self) {
        self.prev_pos = Some((self.x, self.y));
    }

    pub fn update_position(&mut self, (x, y): (uint, uint)) {
        self.x = x;
        self.y = y
    }

    pub fn pos(&self) -> (uint, uint) {
        (self.x, self.y)
    }

}

impl fmt::Show for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.buf.write_str(format!("[{:4d}]", self.value))
    }
}

// Layout for console debug
//
// +-----+-----+-----+-----+
// |(0,0)|(0,1)|     |     |
// +-----+-----+-----+-----+
// |(1,0)|     |     |     |
// +-----+-----+-----+-----+
// |     |     |     |     |
// +-----+-----+-----+-----+
// |     |     |     |(3,3)|
// +-----+-----+-----+-----+
//
//


#[deriving(Eq, Show, Clone)]
pub struct Grid {
    pub size: uint,
    pub cells: Vec<Vec<Option<Tile>>>
}

impl Grid {
    pub fn new(size: uint) -> Grid {
        Grid {
            size: size,
            cells: Vec::from_elem(size,
                                  Vec::from_elem(size, None))
        }
    }

    pub fn random_available_cell(&self) -> Option<(uint, uint)> {
        let cells = self.available_cells();

        match cells.len() {
            0 => None,
            n => {
                Some(*cells.get(rand::random::<uint>() % n))
            }
        }
    }

    pub fn available_cells(&self) -> Vec<(uint, uint)> {
        let mut cells = Vec::new();

        self.each_cell(|x, y, tile| {
            if tile.is_none() {
                cells.push((x,y))
            }
        });
        cells
    }

    pub fn each_cell(&self, callback: |uint, uint, Option<&Tile>|) {
        for x in range(0u, self.size) {
            for y in range(0u, self.size) {
                callback(x, y, self.cells.get(x).get(y).as_ref())
            }
        }
    }

    pub fn each_mut_cell(&mut self, callback: |uint, uint, &mut Option<Tile>|) {
        for x in range(0u, self.size) {
            for y in range(0u, self.size) {
                callback(x, y, self.cells.get_mut(x).get_mut(y))
            }
        }
    }


    pub fn cells_available(&self) -> bool {
        self.available_cells().len() != 0
    }

    pub fn cell_available(&self, (x, y): (uint, uint)) -> bool {
        self.cells.get(x).get(y).is_none()
    }

    pub fn cell_occupied(&self, (x, y): (uint, uint)) -> bool {
        self.cells.get(x).get(y).is_some()
    }

    pub fn insert_tile(&mut self, tile: Tile) {
        *self.cells.get_mut(tile.x).get_mut(tile.y) = Some(tile);
    }

    pub fn remove_tile(&mut self, tile: Tile) {
        println!("remove ({}, {})", tile.x, tile.y);
        *self.cells.get_mut(tile.x).get_mut(tile.y) = None;
    }

    pub fn within_bounds(&self, (x, y): (uint, uint)) -> bool {
        x < self.size && y < self.size
    }

    pub fn cell_content(&self, (x, y): (uint, uint)) -> Option<Tile> {
        if self.within_bounds((x, y)) {
            *self.cells.get(x).get(y)
        } else {
            None
        }
    }

    fn debug_print(&self) {
        for col in self.cells.iter() {
            for cell in col.iter() {
                match *cell {
                    Some(t) => print!("{}\t", t),
                    None    => print!("[    ]\t"),
                }
            }
            println!("");
        }
    }

}

#[deriving(Show)]
pub struct GameManager {
    pub size: uint,
    pub start_tiles: uint,

    pub grid: Grid,
    pub score: uint,
    pub playing: bool
}

impl GameManager {
    pub fn new(size: uint) -> GameManager {
        GameManager { size: size,
                      start_tiles: 2,
                      grid: Grid::new(size),
                      score: 0,
                      playing: false }
    }

    pub fn setup(&mut self) {
        self.playing = true;

        self.add_start_tiles();
    }

    pub fn add_random_tile(&mut self) {
        if self.grid.cells_available() {
            let value = if rand::random::<uint>() % 10 < 9 { 2 } else { 4 };
            let tile = Tile::new(self.grid.random_available_cell().unwrap(), value);
            println!("add new at {}, val = {}", tile.pos(), value);
            self.grid.insert_tile(tile);
        }
    }

    fn add_start_tiles(&mut self) {
        for _ in range(0, self.start_tiles) {
            self.add_random_tile();
        }
    }

    pub fn prepare_tiles(&mut self) {
        self.grid.each_mut_cell(|_x, _y, tile| {
            if tile.is_some() {
                let mut t = tile.unwrap().clone();
                t.merged_from = None;
                t.save_position();
                *tile = Some(t);
            }
        })
    }

    pub fn move_tile(&mut self, tile: Tile, (x, y): (uint, uint)) {
        println!("move {} to {}", tile.pos(), (x,y));
        let mut tile = tile;
        *self.grid.cells.get_mut(tile.x).get_mut(tile.y) = None;
        tile.update_position((x, y));
        //*self.grid.cells.get_mut(tile.x).get_mut(tile.y) = None;
        let pos = self.grid.cells.get_mut(x).get_mut(y);
        *pos = Some(tile);
        //(*pos).unwrap().update_position((x, y));
    }

    pub fn move(&mut self, dir: Direction) -> bool {
        let mut moved = false;

        self.prepare_tiles();
        for (x, y) in self.build_traversal(dir) {
            let tile_opt = self.grid.cell_content((x, y));
            match tile_opt {
                Some(mut tile) => {
                    let (farthest_pos, next_pos) = self.find_farthest_position((x,y), dir.to_vector());
                    let next_opt = self.grid.cell_content(next_pos);
                    match next_opt {
                        Some(next) if next.value == tile.value && next.merged_from.is_none() => {
                            let mut merged = Tile::new(next_pos, tile.value * 2);
                            println!("{}, {} merged to {}", tile.pos(), next_pos, next_pos);
                            merged.merged_from = Some((tile.pos(), next.pos()));

                            self.grid.insert_tile(merged);
                            self.grid.remove_tile(tile);

                            tile.update_position(next_pos);

                            self.score += merged.value as uint;
                            // The mighty 2048 tile
                            moved = true;
                        }
                        _ => {
                            if tile.pos() != farthest_pos {
                                self.move_tile(tile, farthest_pos);
                                moved = true;
                            }
                        }
                    }
                }
                _ => ()
            }
        }

        if moved {
            // xxx moves_av
            println!("some cell moved, add new one!");
            self.add_random_tile();
        }

        moved
    }

    pub fn moves_available(&self) -> bool {
        self.grid.cells_available() || self.tile_matches_available()
    }

    // visit order
    fn build_traversal(&self, dir: Direction) -> Traversal {
        Traversal::new(self.size, dir)
    }

    fn find_farthest_position(&self, (x, y): (uint, uint), (dx, dy): (int, int)) -> ((uint, uint), (uint, uint)) {
        let (mut prev_x, mut prev_y) = (x as int, y as int);
        let (mut next_x, mut next_y) = (prev_x + dx, prev_y + dy);

        while self.grid.within_bounds((next_x as uint, next_y as uint)) &&
              self.grid.cell_available((next_x as uint, next_y as uint)) {
                  prev_x = next_x;
                  prev_y = next_y;
                  next_x = prev_x + dx;
                  next_y =  prev_y + dy;
        }
        // (farthest, next)
        ((prev_x as uint, prev_y as uint),
         (next_x as uint, next_y as uint))
    }

    fn tile_matches_available(&self) -> bool {
        for x in range(0u, self.size) {
            for y in range(0u, self.size) {
                match self.grid.cell_content((x,y)) {
                    Some(tile) => {
                        for dir in Direction::all_directions().iter() {
                            let (dx, dy) = dir.to_vector();
                            let nx = (x as int + dx) as uint;
                            let ny = (y as int + dy) as uint;
                            match self.grid.cell_content((nx, ny)) {
                                Some(other) if other.value == tile.value => {
                                    return true;
                                }
                                _ => ()
                            }
                        }

                    }
                    _ => ()
                }

            }
        }
        false
    }

}
