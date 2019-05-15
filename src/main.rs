// Raw terminal mode requires a carriage return which isn't inserted by `writeln!()`
// https://doc.rust-lang.org/std/macro.writeln.html
#![allow(clippy::write_with_newline)]

mod ascii_box;
mod mark;
mod cell;
mod coord;
mod board;
mod states;

use states::{MainMenu, Runnable, Transition};

use std::io;
use std::io::Write;
use termion::{
    raw::IntoRawMode,
    screen::AlternateScreen,
};

pub fn clear(screen: &mut impl io::Write) -> Result<(), io::Error> {
    write!(
        screen,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
}

fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut screen = AlternateScreen::from(io::stdout().into_raw_mode()?);

    write!(screen, "{}", termion::cursor::Hide,)?;

    let mut current_state: Box<
        dyn Runnable<io::Stdin, AlternateScreen<termion::raw::RawTerminal<io::Stdout>>>,
    > = Box::new(MainMenu::default());

    loop {
        clear(&mut screen)?;

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
