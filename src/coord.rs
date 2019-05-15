#[derive(Default, Debug, Copy, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub const fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }

    pub fn mv(&mut self, direction: Direction, grid_size: usize) {
        use Direction::*;

        let upper_bound = grid_size - 1;

        match direction {
            Up => {
                if self.y > 0 {
                    self.y -= 1
                }
            }
            Down => {
                if self.y < upper_bound {
                    self.y += 1
                }
            }
            Left => {
                if self.x > 0 {
                    self.x -= 1
                }
            }
            Right => {
                if self.x < upper_bound {
                    self.x += 1
                }
            }
        };
    }
}

impl PartialEq<(usize, usize)> for Coord {
    fn eq(&self, other: &(usize, usize)) -> bool {
        (self.x, self.y) == *other
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
