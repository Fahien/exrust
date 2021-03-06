mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Wrap web-sys console log function in a println! style macro
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// With repr(8) each cell is represented as a single byte
/// Also by using 0 and 1 we can easily count cell's live neighbors with addition
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    /// Flips the cell state
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

// This annotation helps us define and work with opaque
// handles to JavaScript objects or Boxed Rust structures
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn count_live_neighbors(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        // Here we use deltas and modulo to avoid special casing the edges with ifs
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // Skip self
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;

                let idx = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[idx] as u8;
            }
        }

        count
    }

    /// Returns the dead and alive values of the universe
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

/// Public methods exported to JavaScript
#[wasm_bindgen]
impl Universe {
    /// Computes the next generation from the current one
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.count_live_neighbors(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: underpopulation
                    // Any live cell with fewer than two live neighbours dies
                    (Cell::Alive, x) if x < 2 => Cell::Dead,

                    // Rule 2: status quo
                    // Any live cell with two or three live neighbours lives on to the next generation
                    (Cell::Alive, x) if x >= 2 && x <= 3 => Cell::Alive,

                    // Rule 3: overpopulation
                    // Any live cell with more than three live neighbours dies
                    (Cell::Alive, x) if x > 3 => Cell::Dead,

                    // Rule 4: reproduction
                    // Any dead cell with exactly three life neighbours becomes a live cell
                    (Cell::Dead, 3) => Cell::Alive,

                    // Other cells retain their states
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    /// Initializes an universe with an interesting pattern of live cells
    pub fn new() -> Self {
        // Here we install a hook that in case of panic will display
        // informative error messages in the developer console
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let cells: Vec<Cell> = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            width: width as u32,
            height: height as u32,
            cells,
        }
    }

    pub fn render(&self) -> String {
        // Automatically provided by the Display trait
        self.to_string()
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Returns a pointer to the start of the cells array
    pub fn get_cells_ptr(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Set the width of the universe by resetting all cells to a dead state
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe by resetting all cells to a dead state
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    /// Flips the state of a cell at a given position
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '⬜' } else { '⬛' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
