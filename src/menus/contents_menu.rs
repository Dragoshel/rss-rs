use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

use crossterm::event::{KeyCode, KeyEvent};

use crate::models::Item;
use crate::widgets::{TextBox, HelpBox};

use super::{Menu, MenuState};

#[derive(Default)]
pub struct ContentsMenu<'a> {
	text_box: TextBox<'a>
}

impl<'a> ContentsMenu<'a> {
	pub fn new(item: Item) -> Self {
		ContentsMenu {
			text_box: TextBox::new("Contents", item.description.unwrap().to_string())
		}
	}
}

impl<'a> Menu for ContentsMenu<'a> {
	fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(2)
			.constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
			.split(f.size());

		let help = HelpBox {};

		f.render_widget(help.as_render(), chunks[0]);
		f.render_widget(self.text_box.as_render(), chunks[1]);
	}

	fn transition(&mut self, key_event: KeyEvent) -> MenuState {
		match key_event.code {
			KeyCode::Char('q') => return MenuState::Exit,
			KeyCode::Enter => return MenuState::Stories(None),
			KeyCode::Char('h') => return MenuState::Stories(None),
			_ => {}
		}

		MenuState::Contents(None)
	}

	fn handle_key_event(&mut self, key_event: KeyEvent) {
		self.text_box.navigate(key_event);
	}

	fn state(&mut self) -> MenuState {
		MenuState::Contents(None)
	}
}
