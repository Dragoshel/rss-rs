
mod feeds;
mod stories;
mod contents;

use std::io::Stdout;

pub use self::feeds::FeedsMenu;
pub use self::stories::StoriesMenu;
pub use self::contents::ContentsMenu;

use tui::Frame;
use tui::backend::CrosstermBackend;

use crossterm::event::KeyEvent;

pub enum MenuState {
	Feeds,
	Stories(Option<String>),
	Contents(Option<String>),
	Exit
}

pub trait Menu {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>);
	fn transition(&mut self, key_event: KeyEvent) -> MenuState;
	fn handle_key_event(&mut self, key_event: KeyEvent);
	fn get_state(&mut self) -> MenuState;
}