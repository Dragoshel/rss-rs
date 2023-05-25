use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;

use crossterm::event::{KeyCode, KeyEvent};

use super::{Menu, MenuState};

use crate::models::Item;
use crate::widgets::ItemList;

#[derive(Default)]
pub struct StoriesMenu<'a> {
	item_list: ItemList<'a>,
}

impl<'a> StoriesMenu<'a> {
	pub fn new(stories: Vec<Item>) -> Self {
		StoriesMenu {
			item_list: ItemList::new("Latest Stories", stories),
		}
	}
}

impl<'a> Menu for StoriesMenu<'a> {
	fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(2)
			.constraints([Constraint::Percentage(100)].as_ref())
			.split(f.size());

		f.render_stateful_widget(
			self.item_list.as_render(),
			chunks[0],
			&mut self.item_list.state,
		);
	}

	fn transition(&mut self, key_event: KeyEvent) -> MenuState {
		match key_event.code {
			KeyCode::Char('q') => return MenuState::Exit,
			KeyCode::Char('h') => return MenuState::Feeds,
			KeyCode::Enter => {
				let selected = self.item_list.state.selected().unwrap();
				let selected = self.item_list.items.get(selected).unwrap();
				let mut dto_item = Item::default();
				dto_item.title = selected.title.clone();	
				dto_item.link = selected.link.clone();
				dto_item.description = selected.description.clone();

				return MenuState::Contents(Some(dto_item));
			}
			_ => {}
		}

		MenuState::Stories(None)
	}

	fn handle_key_event(&mut self, key_event: KeyEvent) {
		self.item_list.navigate(key_event);
	}

	fn state(&mut self) -> MenuState {
		MenuState::Stories(None)
	}
}
