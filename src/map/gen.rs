use crate::prelude::*;

impl Map {
    pub fn for_adjacent(
        width: usize,
        height: usize,
        x: usize,
        y: usize,
        mut visitor: impl FnMut(usize, usize),
    ) {
        if x > 0 {
            visitor(x - 1, y);
            if y > 0 {
                visitor(x - 1, y - 1);
            }
            if y + 1 < height {
                visitor(x - 1, y + 1);
            }
        }
        if x + 1 < width {
            visitor(x + 1, y);
            if y > 0 {
                visitor(x + 1, y - 1);
            }
            if y + 1 < height {
                visitor(x + 1, y + 1);
            }
        }
        if y > 0 {
            visitor(x, y - 1);
        }
        if y + 1 < height {
            visitor(x, y + 1);
        }
    }

    pub fn is_adjacent(x: usize, y: usize, q_x: usize, q_y: usize) -> bool {
        (x != q_x || y != q_y) && Self::is_adjacent_or_same(x, y, q_x, q_y)
    }

    pub fn is_adjacent_or_same(x: usize, y: usize, q_x: usize, q_y: usize) -> bool {
        (x == q_x || x + 1 == q_x || x == q_x + 1) && (y == q_y || y + 1 == q_y || y == q_y + 1)
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let mut mines = self.get_mines();
        let (width, height) = self.get_size();
        let mut rest = width * height;

        for y in 0..height {
            for x in 0..width {
                let tile = self.get_tile_mut(x, y).unwrap();
                // This evenly spreads the mines
                if rng.gen_range(0..rest) < mines {
                    tile.set_num(usize::MAX);
                    mines -= 1;
                } else {
                    tile.set_num(0);
                }
                tile.set_type(TileType::Unknown);
                tile.set_dirty(true);
                rest -= 1;
            }
        }

        for y in 0..height {
            for x in 0..width {
                if self.get_tile(x, y).unwrap().get_num() == usize::MAX {
                    continue;
                }

                let mut count = 0;
                Self::for_adjacent(width, height, x, y, |x, y| {
                    if self.get_tile(x, y).unwrap().get_num() == usize::MAX {
                        count += 1;
                    }
                });
                self.get_tile_mut(x, y).unwrap().set_num(count);
            }
        }
    }

    fn randomize_until_no_loop(&mut self) {
        loop {
            self.randomize();
            if !self.has_loop() {
                break;
            }
        }
    }

    pub fn generate(&mut self) {
        match self.mode {
            GameMode::Classic => {
                self.randomize();
            }
            GameMode::Flagger => {
                self.randomize_until_no_loop();
            }
        }
    }

    pub fn recover_tiles(&mut self) {
        let (width, height) = self.get_size();
        for y in 0..height {
            for x in 0..width {
                let tile = self.get_tile_mut(x, y).unwrap();
                tile.set_type(TileType::Unknown);
                tile.set_dirty(true);
            }
        }
    }

    fn has_loop(&self) -> bool {
        let mut checker = MapLoopCheck::new(self);
        checker.check()
    }
}

struct MapLoopCheck<'a> {
    map: &'a Map,
    width: usize,
    height: usize,
    vis: Box<[bool]>,
}

impl<'a> MapLoopCheck<'a> {
    fn new(map: &'a Map) -> Self {
        let (width, height) = map.get_size();
        let vis = vec![false; width * height].into_boxed_slice();
        Self {
            map,
            width,
            height,
            vis,
        }
    }

    fn search(&mut self, cur: usize) -> bool {
        if self.vis[cur] {
            true
        } else {
            let mut result = false;
            self.vis[cur] = true;
            Map::for_adjacent(
                self.width,
                self.height,
                cur % self.width,
                cur / self.width,
                |x, y| {
                    let pos = self.map.index_of(x, y);
                    if pos != cur && self.map.get_tile(x, y).unwrap().get_num() == usize::MAX {
                        result = result || self.search(pos);
                    }
                },
            );
            result
        }
    }

    fn check(&mut self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = self.map.index_of(x, y);
                if self.map.get_tile(x, y).unwrap().get_num() == usize::MAX {
                    if self.search(pos) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
