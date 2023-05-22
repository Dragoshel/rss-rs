use std::io::Stdout;

use crossterm::event::{KeyEvent, KeyCode};
use tui::{backend::CrosstermBackend, Frame, layout::{Layout, Direction, Constraint}, widgets::{Block, Borders, Paragraph}, text::Text};

use crate::models::Item;

use super::{Menu, MenuState};

#[derive(Default)]
pub struct ContentsMenu {
	pub item: Item
}

impl Menu for ContentsMenu {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

		let block = Block::default()
			.title(self.item.title.clone().unwrap())
			.borders(Borders::ALL);
		let text = Text::from(self.item.description.clone().unwrap());
		let paragraph = Paragraph::new(text)
			.block(block);

		f.render_widget(
			paragraph,
			chunks[0]
		);
	}

	fn transition(&mut self, key_event: KeyEvent) -> MenuState {
		match key_event.code {
			KeyCode::Char('q') => return MenuState::Exit,
			KeyCode::Enter => return MenuState::Stories(None),
			KeyCode::Char('h') => {
				return MenuState::Stories(None)
			}
			_ => {}
		}

		MenuState::Contents(None)
	}

	fn handle_key_event(&mut self, key_event: KeyEvent) {
		match key_event.code {
			KeyCode::Char('j') => {
				// Go down
			},
			KeyCode::Char('k') => {
				// Go Up
			},
			_ => {}
		}
	}

	fn get_state(&mut self) -> MenuState {
		MenuState::Contents(None)
	}
}