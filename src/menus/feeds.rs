use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};

use crossterm::event::{KeyCode, KeyEvent};

use crate::widgets::NavList;

use super::{Menu, MenuState};

#[derive(Default)]
pub struct FeedsMenu<'a> {
    input_buffer: String,
    nav_list: NavList<'a>,
}

impl<'a> FeedsMenu<'a> {
    pub fn new(subscribed_channels: Vec<(String, String)>) -> FeedsMenu<'a> {
		let items:Vec<String> = subscribed_channels.iter().map(|f| f.0.to_string()).collect();
        FeedsMenu {
            input_buffer: String::new(),
            nav_list: NavList::new("Feeds", items),
        }
    }
}

impl<'a> Menu for FeedsMenu<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        // Input Widget
        let block = Block::default()
            .title("Get feed manually")
            .borders(Borders::ALL);
        let text = Text::from(self.input_buffer.as_ref());
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, chunks[0]);

        f.render_stateful_widget(
            self.nav_list.as_render(),
            chunks[1],
            &mut self.nav_list.state,
        );
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        match key_event.code {
            KeyCode::Char('q') => return MenuState::Exit,
            KeyCode::Char(key) => {
                self.input_buffer.push(key);
            }
            KeyCode::Enter => return MenuState::Stories,
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            _ => {}
        }
		
		MenuState::Feeds
    }

	fn handle_key_event(&mut self, key_event: KeyEvent) {
		self.nav_list.navigate(key_event);
	}


	fn get_state(&mut self) -> MenuState {
	    MenuState::Feeds
	}
}
