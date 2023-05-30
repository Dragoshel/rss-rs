mod contents_menu;
mod feeds_menu;
mod stories_menu;

pub use self::contents_menu::ContentsMenu;
pub use self::feeds_menu::FeedsMenu;
pub use self::stories_menu::StoriesMenu;

use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::Frame;

use crossterm::event::KeyEvent;

use crate::models::{Channel, Item};

pub enum MenuState {
	Feeds,
	Stories(Option<Channel>),
	Contents(Option<Item>),
	Exit,
}

pub trait Menu {
	fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>);
	fn transition(&mut self, key_event: KeyEvent) -> MenuState;
	fn state(&mut self) -> MenuState;
}
