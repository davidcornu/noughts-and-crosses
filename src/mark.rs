#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mark {
    Nought,
    Cross,
}

impl Default for Mark {
    fn default() -> Self {
        Mark::Nought
    }
}

impl std::ops::Not for Mark {
    type Output = Mark;

    fn not(self) -> Self {
        match self {
            Mark::Cross => Mark::Nought,
            Mark::Nought => Mark::Cross,
        }
    }
}
