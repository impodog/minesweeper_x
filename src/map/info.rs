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

#[derive(Resource, Debug)]
pub struct Map {
    width: usize,
    height: usize,
    mines: usize,
    scale: f32,
    tiles: Box<[Tile]>,
    pub status: MapStatus,

    flags: usize,
    opened: usize,
}

impl Map {
    pub fn new(width: usize, height: usize, mines: usize, scale: f32) -> Self {
        let tiles = vec![Tile::default(); width * height].into_boxed_slice();
        Self {
            width,
            height,
            scale,
            mines,
            tiles,
            status: MapStatus::Play,
            flags: 0,
            opened: 0,
        }
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

    pub fn open(&mut self, x: usize, y: usize) -> Option<usize> {
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
                    self.open(x, y);
                });
            }
        }

        result
    }

    pub fn start(&mut self, x: usize, y: usize) -> Option<usize> {
        while self.get_tile(x, y).unwrap().get_num() != 0 {
            self.randomize();
            println!("Number: {}", self.get_tile(x, y).unwrap().get_num());
        }
        self.open(x, y)
    }

    pub fn open_all(&mut self, x: usize, y: usize) {
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
                    self.open(x, y);
                }
            });
        }
    }

    pub fn mark(&mut self, x: usize, y: usize) {
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
            tile.dirty = true;
        }
        self.flags = flags;
    }
}
