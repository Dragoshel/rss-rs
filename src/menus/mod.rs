mod feeds;
mod stories;

pub use self::feeds::FeedsMenu;
pub use self::stories::StoriesMenu;

use tui::Frame;
use tui::backend::Backend;
use crossterm::event::Event;

pub enum MenuState {
	FeedsMenu,
	StoriesMenu
}

pub trait Menu {
	fn transition(&mut self, event: Event) -> Option<MenuState>;
    fn ui<B: Backend>(&mut self, f: &mut Frame<B>);
}