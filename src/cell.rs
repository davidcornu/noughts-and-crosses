use crate::mark::Mark;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellState {
    Marked(Mark),
    Empty,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Empty
    }
}

impl CellState {
    pub fn is_marked(self) -> bool {
        if let CellState::Marked(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Cell {
    pub state: CellState,
    pub winning: bool,
}
