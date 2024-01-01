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

    pub fn randomize(&mut self) {
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

    pub fn randomize_until_no_loop(&mut self) {
        loop {
            self.randomize();
            if !self.has_loop() {
                break;
            }
        }
    }

    pub fn has_loop(&self) -> bool {
        let mut checker = MapLoopCheck::new(self);
        checker.check()
    }
}

// This is an implementation of Tarjan's algorithm.
// This check whether the mines form a loop and isolate some non-mine-blocks
struct MapLoopCheck<'a> {
    map: &'a Map,
    width: usize,
    height: usize,
    vis: Box<[bool]>,
    dfn: Box<[usize]>,
    low: Box<[usize]>,
    count: usize,
}

impl<'a> MapLoopCheck<'a> {
    fn new(map: &'a Map) -> Self {
        let (width, height) = map.get_size();
        let vis = vec![false; width * height].into_boxed_slice();
        let dfn = vec![usize::MAX; width * height].into_boxed_slice();
        let low = vec![usize::MAX; width * height].into_boxed_slice();
        Self {
            map,
            width,
            height,
            vis,
            dfn,
            low,
            count: 0,
        }
    }

    fn dfs(&mut self, x: usize, y: usize) -> usize {
        let i = self.map.index_of(x, y);
        if !self.vis[i] {
            self.vis[i] = true;
            self.dfn[i] = self.count;
            self.low[i] = self.count;
            self.count += 1;

            Map::for_adjacent(self.width, self.height, x, y, |x, y| {
                let j = self.map.index_of(x, y);
                if !self.vis[j] {
                    self.dfs(x, y);
                    self.low[i] = self.low[i].min(self.low[j]);
                } else if self.dfn[j] < self.dfn[i] {
                    self.low[i] = self.low[i].min(self.dfn[j]);
                }
            });
        }
        i
    }

    fn check(&mut self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                let i = self.dfs(x, y);
                if self.dfn[i] == self.low[i] {
                    return true;
                }
            }
        }

        false
    }
}
