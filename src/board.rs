use crate::{
    cell::{Cell, CellState},
    coord::Coord,
    mark::Mark
};

#[derive(Debug)]
pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Default for Board {
    fn default() -> Self {
        let size = 3;
        Board {
            size,
            cells: vec![vec![Cell::default(); size]; size],
        }
    }
}

type WinCoords = Vec<Coord>;

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            size,
            cells: vec![vec![Cell::default(); size]; size],
        }
    }

    pub fn cell_at_mut(&mut self, coord: Coord) -> &mut Cell {
        &mut self.cells[coord.y][coord.x]
    }

    pub fn cell_at(&self, coord: Coord) -> &Cell {
        &self.cells[coord.y][coord.x]
    }

    fn generate_diagonal_coords(&self) -> Vec<WinCoords> {
        let mut top_left_to_bottom_right = vec![Coord::default(); self.size];
        let mut top_right_to_bottom_left = vec![Coord::default(); self.size];

        for i in 0..self.size {
            top_left_to_bottom_right[i] = Coord::new(i, i);
            top_right_to_bottom_left[i] = Coord::new(self.size - 1 - i, i);
        }

        vec![top_left_to_bottom_right, top_right_to_bottom_left]
    }

    fn check_row(&self, y: usize, mark: Mark) -> Option<WinCoords> {
        let mut row_coords = vec![Coord::default(); self.size];

        #[allow(clippy::needless_range_loop)]
        for x in 0..self.size {
            let coord = Coord::new(x, y);
            if self.cell_at(coord).state != CellState::Marked(mark) {
                return None;
            }
            row_coords[x] = coord;
        }

        Some(row_coords)
    }

    fn check_column(&self, x: usize, mark: Mark) -> Option<WinCoords> {
        let mut col_coords = vec![Coord::default(); self.size];

        #[allow(clippy::needless_range_loop)]
        for y in 0..self.size {
            let coord = Coord::new(x, y);
            if self.cell_at(coord).state != CellState::Marked(mark) {
                return None;
            }
            col_coords[y] = coord;
        }

        Some(col_coords)
    }

    fn check_diagonals(&self, mark: Mark) -> Option<WinCoords> {
        for coords in self.generate_diagonal_coords() {
            if coords
                .iter()
                .all(|coord| self.cell_at(*coord).state == CellState::Marked(mark))
            {
                return Some(coords);
            }
        }
        None
    }

    pub fn is_win(&self, coord: Coord, mark: Mark) -> Option<WinCoords> {
        self.check_column(coord.x, mark)
            .or_else(|| self.check_row(coord.y, mark))
            .or_else(|| self.check_diagonals(mark))
    }
}
