// Raw terminal mode requires a carriage return which isn't inserted by `writeln!()`
// https://doc.rust-lang.org/std/macro.writeln.html
#![allow(clippy::write_with_newline)]

use std::io;
use std::io::Write;
use termion::{
    event::{Event, Key},
    raw::IntoRawMode,
    screen::AlternateScreen,
    input::TermRead,
    color,
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Mark {
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum CellState {
    Marked(Mark),
    Empty,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Empty
    }
}

impl CellState {
    fn is_marked(self) -> bool {
        if let CellState::Marked(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct Cell {
    state: CellState,
    winning: bool,
}

#[derive(Debug)]
struct Board {
    size: usize,
    cells: Vec<Vec<Cell>>,
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

impl Board {
    fn cell_at_mut(&mut self, coord: Coord) -> &mut Cell {
        &mut self.cells[coord.y][coord.x]
    }

    fn cell_at(&self, coord: Coord) -> &Cell {
        &self.cells[coord.y][coord.x]
    }
}

type WinCoords = Vec<Coord>;

impl Board {
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

    fn is_win(&self, coord: Coord, mark: Mark) -> Option<WinCoords> {
        self.check_column(coord.x, mark)
            .or_else(|| self.check_row(coord.y, mark))
            .or_else(|| self.check_diagonals(mark))
    }
}


#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    const fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }

    fn mv(&mut self, direction: Direction, grid_size: usize) {
        use Direction::*;

        let upper_bound = grid_size - 1;

        match direction {
            Up => self.y = if self.y == 0 { 0 } else { self.y - 1 },
            Down => {
                self.y = if self.y == upper_bound {
                    upper_bound
                } else {
                    self.y + 1
                }
            }
            Left => self.x = if self.x == 0 { 0 } else { self.x - 1 },
            Right => {
                self.x = if self.x == upper_bound {
                    upper_bound
                } else {
                    self.x + 1
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

#[derive(Default, Debug)]
struct GameState {
    board: Board,
    cursor: Coord,
    mark: Mark,
    winner: Option<Mark>,
}

const CELL_WIDTH: usize = 9;
const CELL_HEIGHT: usize = 3;
const CELL_CENTER: Coord = Coord::new(4, 1);

impl GameState {
    fn draw(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        for (y, row) in self.board.cells.iter().enumerate() {
            for cy in 0..CELL_HEIGHT {
                for (x, cell) in row.iter().enumerate() {
                    let draw_background = if self.winner.is_some() {
                        cell.winning
                    } else {
                        self.cursor == (x, y)
                    };

                    if draw_background {
                        match self.mark {
                            Mark::Nought => write!(out, "{}", color::Bg(color::Green))?,
                            Mark::Cross => write!(out, "{}", color::Bg(color::Blue))?,
                        }
                    }

                    for cx in 0..CELL_WIDTH {
                        if CELL_CENTER == (cx, cy) {
                            let marker = match cell.state {
                                CellState::Marked(Mark::Cross) => "\u{2715}",
                                CellState::Marked(Mark::Nought) => "\u{25EF}",
                                CellState::Empty => " ",
                            };

                            write!(out, "{}", marker)?;
                        } else {
                            write!(out, " ")?;
                        }
                    }

                    if draw_background {
                        write!(out, "{}", color::Bg(color::Reset))?;
                    }

                    if x < self.board.size - 1 {
                        write!(
                            out,
                            "{}  {}",
                            color::Bg(color::White),
                            color::Bg(color::Reset)
                        )?;
                    }
                }

                write!(out, "\r\n")?;
            }

            // Write out bottom border for entire row
            if y < self.board.size - 1 {
                let border =
                    " ".repeat((CELL_WIDTH * self.board.size) + ((self.board.size - 1) * 2));
                write!(
                    out,
                    "{}{}{}\r\n",
                    color::Bg(color::White),
                    border,
                    color::Bg(color::Reset)
                )?;
            } else {
                write!(out, "\r\n")?;
            }
        }

        Ok(())
    }

    fn play(&mut self) {
        let cell = self.board.cell_at_mut(self.cursor);
        if cell.state.is_marked() {
            return;
        }

        cell.state = CellState::Marked(self.mark);

        if let Some(coords) = self.board.is_win(self.cursor, self.mark) {
            self.winner = Some(self.mark);
            for coord in coords.iter() {
                let cell = self.board.cell_at_mut(*coord);
                cell.winning = true;
            }
        } else {
            self.mark = !self.mark;
        }
    }
}

impl<I: io::Read, O: io::Write> Runnable<I, O> for GameState {
    fn run(&mut self, input: &mut I, screen: &mut O) -> Result<Transition<I, O>, io::Error> {
        self.draw(screen)?;
        screen.flush()?;

        for event_result in input.events() {
            let evt = event_result?;
            match evt {
                Event::Key(Key::Char('q')) => {
                    return Ok(Transition::Done);
                }
                Event::Key(Key::Up) => self.cursor.mv(Direction::Up, self.board.size),
                Event::Key(Key::Down) => self.cursor.mv(Direction::Down, self.board.size),
                Event::Key(Key::Left) => self.cursor.mv(Direction::Left, self.board.size),
                Event::Key(Key::Right) => self.cursor.mv(Direction::Right, self.board.size),
                Event::Key(Key::Char('\n')) => self.play(),
                _ => {}
            }

            write!(
                screen,
                "{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1)
            )?;
            self.draw(screen)?;
            screen.flush()?;

            if self.winner.is_some() {
                std::thread::sleep(std::time::Duration::from_secs(5));
                break;
            }
        }

        Ok(Transition::Next(Box::new(GameState::default())))
    }
}

enum Transition<I, O> {
    Next(Box<dyn Runnable<I, O>>),
    Done,
}

trait Runnable<I: io::Read, O: io::Write> {
    fn run(&mut self, input: &mut I, screen: &mut O) -> Result<Transition<I, O>, io::Error>;
}

fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut screen = AlternateScreen::from(io::stdout().into_raw_mode()?);

    write!(screen, "{}", termion::cursor::Hide,)?;

    let mut current_state: Box<
        dyn Runnable<io::Stdin, AlternateScreen<termion::raw::RawTerminal<io::Stdout>>>,
    > = Box::new(GameState::default());

    loop {
        write!(
            screen,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )?;

        match current_state.run(&mut stdin, &mut screen)? {
            Transition::Done => {
                break;
            }
            Transition::Next(runnable) => {
                current_state = runnable;
            }
        }
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();

    Ok(())
}
