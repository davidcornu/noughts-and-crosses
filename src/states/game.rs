use std::io;
use crate::{
    mark::Mark,
    coord::{Coord, Direction},
    board::Board,
    cell::CellState,
    states::{Transition, Runnable, MainMenu},
    clear
};
use termion::{
    color,
    event::{Event, Key},
    input::TermRead,
};

#[derive(Default, Debug)]
pub struct Game {
    board: Board,
    cursor: Coord,
    mark: Mark,
    winner: Option<Mark>,
}

const CELL_WIDTH: usize = 9;
const CELL_HEIGHT: usize = 3;
const CELL_CENTER: Coord = Coord::new(4, 1);

impl Game {
    pub fn new(board_size: usize) -> Self {
        Game {
            board: Board::new(board_size),
            cursor: Coord::default(),
            mark: Mark::default(),
            winner: None,
        }
    }

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

impl<I: io::Read, O: io::Write> Runnable<I, O> for Game {
    fn run(&mut self, input: &mut I, screen: &mut O) -> Result<Transition<I, O>, io::Error> {
        self.draw(screen)?;
        screen.flush()?;

        for event_result in input.events() {
            let evt = event_result?;
            match evt {
                Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => {
                    return Ok(Transition::Done);
                }
                Event::Key(Key::Up) => self.cursor.mv(Direction::Up, self.board.size),
                Event::Key(Key::Down) => self.cursor.mv(Direction::Down, self.board.size),
                Event::Key(Key::Left) => self.cursor.mv(Direction::Left, self.board.size),
                Event::Key(Key::Right) => self.cursor.mv(Direction::Right, self.board.size),
                Event::Key(Key::Char('\n')) => self.play(),
                _ => {}
            }

            clear(screen)?;
            self.draw(screen)?;
            screen.flush()?;

            if self.winner.is_some() {
                std::thread::sleep(std::time::Duration::from_secs(5));
                break;
            }
        }

        Ok(Transition::Next(Box::new(MainMenu::default())))
    }
}
