mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

#[no_mangle]
pub extern "C" fn __export_memory() -> u32 {
    // Return the base address of the WASM linear memory
    // This is often 0, but it's good practice to get it dynamically
    0
}

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

impl Default for Universe {
    fn default() -> Self {
        Universe::new(0)
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: die from loneliness (underpopulation)
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: live if with sweet spot of company
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: die with too many neighbors (overpopulation)
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: dead cells become alive if have
                    // exactly 3 neighbors (reproduction)
                    (Cell::Dead, 3) => Cell::Alive,
                    // o.w. remain in same state
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new(init_state: u32) -> Universe {
        let width: u32 = 64;
        let height: u32 = 64;

        let helper = |row: u32, col: u32| (row * width + col) as usize;

        let cells = match init_state {
            // gosper's glider gun
            0 => {
                let mut _cells: Vec<Cell> = (0..width * height).map(|_| Cell::Dead).collect();
                [
                    (6, 1),
                    (6, 2),
                    (7, 1),
                    (7, 2),
                    (6, 11),
                    (7, 11),
                    (8, 11),
                    (5, 12),
                    (9, 12),
                    (4, 13),
                    (10, 13),
                    (4, 14),
                    (10, 14),
                    (7, 15),
                    (5, 16),
                    (9, 16),
                    (6, 17),
                    (7, 17),
                    (8, 17),
                    (7, 18),
                    (4, 21),
                    (5, 21),
                    (6, 21),
                    (4, 22),
                    (5, 22),
                    (6, 22),
                    (3, 23),
                    (7, 23),
                    (2, 25),
                    (3, 25),
                    (7, 25),
                    (8, 25),
                    (4, 35),
                    (4, 36),
                    (5, 35),
                    (5, 36),
                ]
                .map(|(row, col)| _cells[helper(row, col)] = Cell::Alive);
                _cells
            }
            // random
            1 => (0..width * height)
                .map(|_| {
                    if js_sys::Math::random() < 0.5 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                })
                .collect(),
            // default preset 1 config from tutorial
            _ => (0..width * height)
                .map(|i| {
                    if i % 2 == 0 || i % 7 == 0 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                })
                .collect(),
        };

        Universe {
            width,
            height,
            cells,
        }
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

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
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
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
