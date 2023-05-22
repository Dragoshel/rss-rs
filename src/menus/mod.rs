mod feeds;
mod stories;

use std::io::Stdout;

pub use self::feeds::FeedsMenu;
pub use self::stories::StoriesMenu;

use tui::Frame;
use tui::backend::CrosstermBackend;

use crossterm::event::KeyEvent;

pub enum MenuState {
	Feeds,
	Stories,
	Exit
}

pub trait Menu {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>);
	fn transition(&mut self, key_event: KeyEvent) -> MenuState;
	fn handle_key_event(&mut self, key_event: KeyEvent);
	fn get_state(&mut self) -> MenuState;
}