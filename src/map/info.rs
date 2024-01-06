use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Tile {
    // If it is a mine, num is usize::MAX
    // Otherwise, num is the number of mines around it
    num: usize,
    ty: TileType,

    // This is for refreshing images when auto-flipping tiles
    dirty: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Unknown,
    Open,
    Flag,
    Question,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            num: 0,
            ty: TileType::Unknown,
            dirty: false,
        }
    }
}

impl Tile {
    pub fn is_mine(&self) -> bool {
        self.num == usize::MAX
    }

    pub fn is_open(&self) -> bool {
        self.ty == TileType::Open
    }

    pub fn is_flag(&self) -> bool {
        self.ty == TileType::Flag
    }

    pub fn get_num(&self) -> usize {
        self.num
    }

    pub fn get_type(&self) -> TileType {
        self.ty
    }

    pub fn get_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_num(&mut self, num: usize) {
        self.num = num;
    }

    pub fn set_type(&mut self, ty: TileType) {
        self.ty = ty;
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn inc(&mut self) {
        self.num += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapStatus {
    Play,
    Win,
    Lose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMode {
    Classic,
    Flagger,
}

impl From<usize> for GameMode {
    fn from(n: usize) -> Self {
        match n {
            0 => Self::Classic,
            1 => Self::Flagger,
            _ => panic!("Invalid game mode {}", n),
        }
    }
}

#[derive(Resource, Debug)]
pub struct Map {
    width: usize,
    height: usize,
    mines: usize,
    scale: f32,
    tiles: Box<[Tile]>,
    pub status: MapStatus,
    pub mode: GameMode,

    flags: usize,
    opened: usize,
    pub is_started: bool,

    pub cursor: (usize, usize),
    pub cursor_dirty: bool,

    pub begin_time: f32,
    pub time: f32,
}

impl Map {
    pub fn new(width: usize, height: usize, mines: usize, scale: f32, mode: GameMode) -> Self {
        let size = width * height;
        let tiles = vec![Tile::default(); size].into_boxed_slice();
        Self {
            width,
            height,
            scale,
            mines,
            tiles,
            status: MapStatus::Play,
            mode,
            flags: 0,
            opened: 0,
            is_started: false,
            cursor: (width / 2, height / 2),
            cursor_dirty: false,
            begin_time: 0.0,
            time: 0.0,
        }
    }

    fn clear_status(&mut self) {
        self.status = MapStatus::Play;
        self.flags = 0;
        self.opened = 0;
        self.is_started = false;
        self.cursor = (self.width / 2, self.height / 2);
        self.cursor_dirty = true;
        self.begin_time = 0.0;
        self.time = 0.0;
    }

    pub fn restart(&mut self) {
        self.clear_status();

        self.generate();
    }

    pub fn replay(&mut self) {
        self.clear_status();

        self.recover_tiles();
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y * self.width + x)
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y * self.width + x)
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_mines(&self) -> usize {
        self.mines
    }

    pub fn get_opened(&self) -> usize {
        self.opened
    }

    pub fn get_flags(&self) -> usize {
        self.flags
    }

    pub fn index_of(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn nth(&self, n: usize) -> Option<&Tile> {
        self.tiles.get(n)
    }

    pub fn is_opened(&self, x: usize, y: usize) -> bool {
        if let Some(tile) = self.get_tile(x, y) {
            tile.get_type() == TileType::Open
        } else {
            false
        }
    }

    pub fn position_of(&self, x: usize, y: usize, z_pos: f32) -> Vec3 {
        calc_tile_pos(self.width, self.height, self.scale, x, y, z_pos)
    }

    pub fn move_cursor(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cursor = (x, y);
            self.cursor_dirty = true;
        }
    }

    pub fn update_time(&mut self, time: f32) {
        if self.status == MapStatus::Play && self.is_started {
            self.time = time;
        }
    }

    pub fn get_played_time(&self) -> f32 {
        self.time - self.begin_time
    }

    fn open_tile(&mut self, x: usize, y: usize) -> Option<usize> {
        let result = if let Some(tile) = self.get_tile_mut(x, y) {
            match tile.get_type() {
                TileType::Flag | TileType::Open => None,
                _ => {
                    tile.set_type(TileType::Open);
                    tile.dirty = true;
                    Some(tile.get_num())
                }
            }
        } else {
            None
        };

        if let Some(num) = result {
            self.opened += 1;

            if num == usize::MAX {
                self.status = MapStatus::Lose;
            } else if self.opened + self.mines == self.width * self.height {
                self.status = MapStatus::Win;
            }
            if num == 0 {
                let (width, height) = self.get_size();
                Self::for_adjacent(width, height, x, y, |x, y| {
                    self.open_tile(x, y);
                });
            }
        }

        result
    }

    fn close_tile(&mut self, x: usize, y: usize) {
        let result = if let Some(tile) = self.get_tile_mut(x, y) {
            tile.set_type(TileType::Unknown);
            tile.dirty = true;
            true
        } else {
            false
        };
        if result {
            self.opened -= 1;
        }
    }

    fn mark_tile(&mut self, x: usize, y: usize) {
        let mut flags = self.flags;
        if let Some(tile) = self.get_tile_mut(x, y) {
            match tile.get_type() {
                TileType::Unknown => {
                    tile.set_type(TileType::Flag);
                    flags += 1;
                }
                TileType::Flag => {
                    tile.set_type(TileType::Question);
                    flags -= 1;
                }
                TileType::Question => {
                    tile.set_type(TileType::Unknown);
                }
                TileType::Open => {}
            }
            tile.set_dirty(true);
        }
        self.flags = flags;
    }

    fn open_all_tile(&mut self, x: usize, y: usize) {
        let (width, height) = self.get_size();
        let mut flags = 0;
        Self::for_adjacent(width, height, x, y, |x, y| {
            if self.get_tile(x, y).unwrap().is_flag() {
                flags += 1;
            }
        });
        if self.get_tile(x, y).unwrap().get_num() <= flags {
            Self::for_adjacent(width, height, x, y, |x, y| {
                if !self.get_tile(x, y).unwrap().is_flag() {
                    self.open_tile(x, y);
                }
            });
        }
    }

    fn test_tile(&mut self, x: usize, y: usize) -> bool {
        match self.mode {
            GameMode::Classic => true,
            GameMode::Flagger => {
                if !Self::is_adjacent_or_same(self.cursor.0, self.cursor.1, x, y) {
                    false
                } else {
                    true
                }
            }
        }
    }

    pub fn open(&mut self, x: usize, y: usize) -> Option<usize> {
        if !self.test_tile(x, y) {
            return None;
        }

        let result = self.open_tile(x, y);
        if x < self.width && y < self.height {
            self.move_cursor(x, y);
        }

        result
    }

    pub fn start(&mut self, x: usize, y: usize, time: f32) -> Option<usize> {
        while self.get_tile(x, y).unwrap().get_num() != 0 {
            self.generate();
        }
        self.begin_time = time;
        self.time = time;
        self.is_started = true;
        self.open(x, y)
    }

    pub fn open_all(&mut self, x: usize, y: usize) {
        if !self.test_tile(x, y) {
            return;
        }

        self.open_all_tile(x, y);
    }

    pub fn mark(&mut self, x: usize, y: usize) {
        if !self.test_tile(x, y) {
            return;
        }

        self.mark_tile(x, y);
    }

    fn diff(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    pub fn close_if_too_far(&mut self, x: usize, y: usize) {
        if Self::diff(self.cursor.0, x) + Self::diff(self.cursor.1, y) >= 4 {
            self.close_tile(x, y);
        }
    }

    pub fn try_close_far(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                if self.get_tile(i, j).unwrap().is_open() {
                    self.close_if_too_far(i, j);
                }
            }
        }
    }
}
