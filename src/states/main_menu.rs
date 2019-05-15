use std::io;
use termion::{
    style,
    event::{Event, Key},
    input::TermRead,
};
use crate::{
    ascii_box::AsciiBox,
    states::{Runnable, Transition, Game},
    clear
};

pub struct MainMenu {
    pub board_size: usize,
}

impl Default for MainMenu {
    fn default() -> Self {
        MainMenu { board_size: 3 }
    }
}

impl MainMenu {
    fn draw(&self, out: &mut impl io::Write) -> Result<(), io::Error> {
        write!(
            out,
            "{}Noughts & Crosses{}\r\n\r\n",
            style::Bold,
            style::Reset
        )?;

        let mut controls = AsciiBox::default();
        controls.add_line("\u{2191} Move cursor up");
        controls.add_line("\u{2193}             down");
        controls.add_line("\u{2190}             left");
        controls.add_line("\u{2192}             right");
        controls.add_line("\u{21B5} Play");
        controls.add_line("q Quit");
        controls.print(out, "Controls")?;

        write!(
            out,
            "\r\n\
             Board size: {} (use \u{2191} or \u{2193} to change)\r\n\
             Press \u{21B5} to start\r\n",
            self.board_size,
        )?;

        Ok(())
    }
}

const MIN_BOARD_SIZE: usize = 2;
const MAX_BOARD_SIZE: usize = 5;

impl<I: io::Read, O: io::Write> Runnable<I, O> for MainMenu {
    fn run(&mut self, input: &mut I, screen: &mut O) -> Result<Transition<I, O>, io::Error> {
        self.draw(screen)?;
        screen.flush()?;

        for event_result in input.events() {
            let evt = event_result?;

            match evt {
                Event::Key(Key::Up) => {
                    if self.board_size != MAX_BOARD_SIZE {
                        self.board_size += 1;
                    }
                }
                Event::Key(Key::Down) => {
                    if self.board_size != MIN_BOARD_SIZE {
                        self.board_size -= 1;
                    }
                }
                Event::Key(Key::Char('\n')) => break,
                Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => {
                    return Ok(Transition::Done);
                }
                _ => {}
            }

            clear(screen)?;
            self.draw(screen)?;
            screen.flush()?;
        }

        Ok(Transition::Next(Box::new(Game::new(self.board_size))))
    }
}
