mod main_menu;
mod game;
pub use main_menu::MainMenu;
pub use game::Game;

use std::io;

pub enum Transition<I, O> {
    Next(Box<dyn Runnable<I, O>>),
    Done,
}

pub trait Runnable<I: io::Read, O: io::Write> {
    fn run(&mut self, input: &mut I, screen: &mut O) -> Result<Transition<I, O>, io::Error>;
}
