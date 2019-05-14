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

const BOARD_SIZE: usize = 3;

#[derive(Debug)]
struct Board {
    cells: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: [[Cell::default(); BOARD_SIZE]; BOARD_SIZE],
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

type WinCoords = [Coord; BOARD_SIZE];

impl Board {
    fn generate_diagonal_coords() -> [WinCoords; 2] {
        let mut top_left_to_bottom_right = [Coord::default(); BOARD_SIZE];
        let mut top_right_to_bottom_left = [Coord::default(); BOARD_SIZE];

        for i in 0..BOARD_SIZE {
            top_left_to_bottom_right[i] = Coord::new(i, i);
            top_right_to_bottom_left[i] = Coord::new(BOARD_SIZE - 1 - i, i);
        }

        [top_left_to_bottom_right, top_right_to_bottom_left]
    }

    fn check_row(&self, y: usize, mark: Mark) -> Option<WinCoords> {
        let mut row_coords = [Coord::default(); BOARD_SIZE];

        #[allow(clippy::needless_range_loop)]
        for x in 0..BOARD_SIZE {
            let coord = Coord::new(x, y);
            if self.cell_at(coord).state != CellState::Marked(mark) {
                return None;
            }
            row_coords[x] = coord;
        }

        Some(row_coords)
    }

    fn check_column(&self, x: usize, mark: Mark) -> Option<WinCoords> {
        let mut col_coords = [Coord::default(); BOARD_SIZE];

        #[allow(clippy::needless_range_loop)]
        for y in 0..BOARD_SIZE {
            let coord = Coord::new(x, y);
            if self.cell_at(coord).state != CellState::Marked(mark) {
                return None;
            }
            col_coords[y] = coord;
        }

        Some(col_coords)
    }

    fn check_diagonals(&self, mark: Mark) -> Option<WinCoords> {
        for coords in Board::generate_diagonal_coords().iter() {
            if coords
                .iter()
                .all(|coord| self.cell_at(*coord).state == CellState::Marked(mark))
            {
                return Some(*coords);
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

const MAX_XY: usize = BOARD_SIZE - 1;

#[derive(Default, Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    const fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }

    fn mv(&mut self, direction: Direction) {
        use Direction::*;

        match direction {
            Up => self.y = if self.y == 0 { 0 } else { self.y - 1 },
            Down => self.y = if self.y == MAX_XY { MAX_XY } else { self.y + 1 },
            Left => self.x = if self.x == 0 { 0 } else { self.x - 1 },
            Right => self.x = if self.x == MAX_XY { MAX_XY } else { self.x + 1 },
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

                    if x < BOARD_SIZE - 1 {
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
            if y < BOARD_SIZE - 1 {
                let border = " ".repeat((CELL_WIDTH * BOARD_SIZE) + ((BOARD_SIZE - 1) * 2));
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
                Event::Key(Key::Up) => self.cursor.mv(Direction::Up),
                Event::Key(Key::Down) => self.cursor.mv(Direction::Down),
                Event::Key(Key::Left) => self.cursor.mv(Direction::Left),
                Event::Key(Key::Right) => self.cursor.mv(Direction::Right),
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
