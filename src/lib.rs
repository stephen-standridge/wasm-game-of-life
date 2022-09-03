extern crate fixedbitset;
use fixedbitset::FixedBitSet;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height-1, 0, 1].iter().cloned() {
            for delta_col in [self.width-1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    //we're at self
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn spaceship(&mut self, x: u32,y: u32) -> bool {
        match(x, y) {
            (x, 10) if x < 5 => return true,
            (x, 11) if x == 0 || x == 5 => return true,
            (0, 12) => return true,
            (1, 13) => return true,
            (5, 13) => return true,
            (_, _) => return false,
        }
    }

    fn random(&mut self, mut x: f64) -> f64 {
        x = ((x*34.0)+1.0)*x;
        x = x - (x * (1.0 / 289.0)).floor() * 289.0;
        x = x/289.0;
        x
    }

    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let size = (width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Universe {
            width,
            height,
            cells,
        }
    }    

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width*self.height) as usize;
        for i in 0..size {
            self.cells.set(i, false);
        }  
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width*height) as usize;
        for i in 0..size {
            self.cells.set(i, false);
        }        
    }

    pub fn create_random_board(&mut self) {
        let mut next = self.cells.clone();
        let size = (self.width*self.height) as usize;

        for i in 0..size {
            let mut x = i as f64;
            x = self.random(x);
            next.set(i, x < 0.25 || x > 0.75);
        }

        self.cells = next;
    }

    pub fn create_spaceship(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];

                let is_spaceship: bool = self.spaceship(col, row);
                next.set(idx, match(cell, is_spaceship) {
                    (_, true) => true,
                    (_, false) => false,
                });
            }
        }
        self.cells = next;
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(idx, match(cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.  
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                    // All other cells remain in the same state.                    
                });
            }
        }
        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

use std::fmt;
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}