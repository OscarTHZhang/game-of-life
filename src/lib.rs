mod utils;

extern crate js_sys;

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
    cells: Vec<Cell>,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        };
    }
}

#[wasm_bindgen]
impl Universe {

    // get the index in the cells array given row and column
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // get the number of neighbor in the universe that will be alive in the next state
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 { // omit the origin
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

    // tick will compute the next generation from the current one
    // public to expose to javascript
    pub fn tick(&mut self) {
        let mut next = self.cells.clone(); // declare next state

        for r in 0..self.height {
            for c in 0..self.width {
                let index = self.get_index(r, c);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(r, c);
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[index] = next_cell; // assign the next state for the cell
            }
        }
        self.cells = next; // update the cells array to the next cells array
    }

    // init function for Universe
    pub fn new() -> Universe {
        let width = 64;
        let height = 32;

        let cells = (0..width * height)
            .map(|i| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();
        
        Universe {
            width,
            height,
            cells,
        }
    }

    // public getters
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    // change the cell from alive to dead or vice versa
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let index = self.get_index(row, col);
        self.cells[index].toggle();
    }

}
