use std::fmt;
use std::iter;
use rand;


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vector(self) -> (isize, isize) {
        match self {
            Direction::Up    => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down  => (0, 1),
            Direction::Left  => (-1, 0)
        }
    }

    // Haskell succ/pred???
    fn all_directions<'r>() -> Vec<Direction> {
        vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left]
    }
}

pub struct Traversal {
    xs: Vec<usize>,
    ys: Vec<usize>,

    idx: usize,
    max_idx: usize,
    size: usize
}

impl Traversal {
    pub fn new(size: usize, dir: Direction) -> Traversal {
        let (x, y) = dir.to_vector();
        let mut xs = (0..size).collect::<Vec<usize>>();
        let mut ys = (0..size).collect::<Vec<usize>>();
        if x == 1 {
            xs.reverse()
        }
        if y == 1 {
            ys.reverse()
        }
        Traversal { xs: xs.to_vec(), ys: ys.to_vec(),
                    idx: 0, max_idx: size * size,
                    size: size,
        }
    }
}

impl Iterator for Traversal {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
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

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub value: isize,

    pub prev_pos: Option<(usize, usize)>,
    pub merged_from: Option<((usize,usize), (usize, usize))>
}

impl Tile {
    pub fn new((x, y): (usize, usize), value: isize) -> Tile {
        Tile { x: x, y: y, value: value,
               prev_pos: None, merged_from: None }
    }

    pub fn save_position(&mut self) {
        self.prev_pos = Some((self.x, self.y));
    }

    pub fn update_position(&mut self, (x, y): (usize, usize)) {
        self.x = x;
        self.y = y
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }

}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:4b}]", self.value)
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


#[derive(PartialEq, Debug, Clone)]
pub struct Grid {
    pub size: usize,
    pub cells: Vec<Vec<Option<Tile>>>
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        let mut cells = Vec::new();
        for _ in 0..size {
            let mut row = Vec::new();
            row.extend(iter::repeat(None).take(size));
            cells.push(row);
        }
        Grid {
            size: size,
            cells: cells
        }
    }

    pub fn random_available_cell(&self) -> Option<(usize, usize)> {
        let cells = self.available_cells();

        match cells.len() {
            0 => None,
            n => {
                Some(cells[rand::random::<usize>() % n])
            }
        }
    }

    pub fn available_cells(&self) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();

        self.each_cell(|x, y, tile| {
            if tile.is_none() {
                cells.push((x,y))
            }
        });
        cells
    }

    pub fn each_cell<F>(&self, mut callback: F)
        where F: FnMut(usize, usize, Option<&Tile>) {
        for x in 0..self.size {
            for y in 0..self.size {
                callback(x, y, self.cells[x][y].as_ref())
            }
        }
    }

    pub fn each_mut_cell<F>(&mut self, mut callback: F)
        where F: FnMut(usize, usize, &mut Option<Tile>)  {
        for x in 0..self.size {
            for y in 0..self.size {
                callback(x, y, &mut self.cells[x][y])
            }
        }
    }


    pub fn cells_available(&self) -> bool {
        self.available_cells().len() != 0
    }

    pub fn cell_available(&self, (x, y): (usize, usize)) -> bool {
        self.cells[x][y].is_none()
    }

    // pub fn cell_occupied(&self, (x, y): (usize, usize)) -> bool {
    //     self.cells[x][y].is_some()
    // }

    pub fn insert_tile(&mut self, tile: Tile) {
        self.cells[tile.x][tile.y] = Some(tile.clone());
    }

    pub fn remove_tile(&mut self, tile: Tile) {
        println!("remove ({}, {})", tile.x, tile.y);
        self.cells[tile.x][tile.y] = None;
    }

    pub fn within_bounds(&self, (x, y): (usize, usize)) -> bool {
        x < self.size && y < self.size
    }

    pub fn cell_content(&self, (x, y): (usize, usize)) -> Option<Tile> {
        if self.within_bounds((x, y)) {
            self.cells[x][y].clone()
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn debug_prisize(&self) {
        for col in self.cells.iter() {
            for cell in col.iter() {
                match cell.clone() {
                    Some(t) => print!("{}\t", t),
                    None    => print!("[    ]\t"),
                }
            }
            print!("");
        }
    }

}

#[derive(Debug)]
pub struct GameManager {
    pub size: usize,
    pub start_tiles: usize,

    pub grid: Grid,
    pub score: usize,
    pub playing: bool
}

impl GameManager {
    pub fn new(size: usize) -> GameManager {
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
            let value = if rand::random::<usize>() % 10 < 9 { 2 } else { 4 };
            let tile = Tile::new(self.grid.random_available_cell().unwrap(), value);
            print!("add new at {:?}, val = {:?}", tile.pos(), value);
            self.grid.insert_tile(tile);
        }
    }

    fn add_start_tiles(&mut self) {
        for _ in 0..self.start_tiles {
            self.add_random_tile();
        }
    }

    pub fn prepare_tiles(&mut self) {
        self.grid.each_mut_cell(|_x, _y, tile| {
            match tile.as_mut() {
                Some(mut t) => {
                    t.merged_from = None;
                    t.save_position();
                    //*tile = Some(t);
                },
                None => ()
            }
        })
    }

    pub fn move_tile(&mut self, tile: Tile, (x, y): (usize, usize)) {
        println!("move {:?} to {:?}", tile.pos(), (x,y));
        let mut tile = tile;
        self.grid.cells[tile.x][tile.y] = None;
        tile.update_position((x, y));
        //*self.grid.cells.get_mut(tile.x).get_mut(tile.y) = None;
        self.grid.cells[x][y] = Some(tile);
        //*pos = Some(tile);
        //(*pos).unwrap().update_position((x, y));
    }

    pub fn move_to(&mut self, dir: Direction) -> bool {
        let mut moved = false;

        self.prepare_tiles();
        for (x, y) in self.build_traversal(dir) {
            let tile_opt = self.grid.cell_content((x, y));
            match tile_opt {
                Some(mut tile) => {
                    let (farthest_pos, next_pos) = self.find_farthest_position((x,y), dir.to_vector());
                    let next_opt = self.grid.cell_content(next_pos);
                    match next_opt {
                        Some(ref next) if next.value == tile.value && next.merged_from.is_none() => {
                            let mut merged = Tile::new(next_pos, tile.value * 2);
                            println!("{:?}, {:?} merged to {:?}", tile.pos(), next_pos, next_pos);
                            merged.merged_from = Some((tile.pos(), next.pos()));

                            self.grid.insert_tile(merged);
                            self.grid.remove_tile(tile);

                            tile.update_position(next_pos);

                            self.score += merged.value as usize;
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
            print!("some cell moved, add new one!");
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

    fn find_farthest_position(&self, (x, y): (usize, usize), (dx, dy): (isize, isize)) -> ((usize, usize), (usize, usize)) {
        let (mut prev_x, mut prev_y) = (x as isize, y as isize);
        let (mut next_x, mut next_y) = (prev_x + dx, prev_y + dy);

        while self.grid.within_bounds((next_x as usize, next_y as usize)) &&
              self.grid.cell_available((next_x as usize, next_y as usize)) {
                  prev_x = next_x;
                  prev_y = next_y;
                  next_x = prev_x + dx;
                  next_y =  prev_y + dy;
        }
        // (farthest, next)
        ((prev_x as usize, prev_y as usize),
         (next_x as usize, next_y as usize))
    }

    fn tile_matches_available(&self) -> bool {
        for x in 0..self.size {
            for y in 0..self.size {
                match self.grid.cell_content((x,y)) {
                    Some(tile) => {
                        for dir in Direction::all_directions().iter() {
                            let (dx, dy) = dir.to_vector();
                            let nx = (x as isize + dx) as usize;
                            let ny = (y as isize + dy) as usize;
                            match self.grid.cell_content((nx, ny)) {
                                Some(ref other) if other.value == tile.value => {
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
