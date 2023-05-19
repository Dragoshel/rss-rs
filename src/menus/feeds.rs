use crossterm::event::{Event, KeyCode};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};

use crate::widgets::NavList;

use super::{Menu, MenuState};

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
    fn transition(&mut self, event: Event) -> Option<MenuState> {
        if let Event::Key(key) = event {
            self.nav_list.navigate(key);

            match key.code {
                KeyCode::Char('q') => return None,
                KeyCode::Char(key) => {
                    self.input_buffer.push(key);
                }
                KeyCode::Enter => return Some(MenuState::StoriesMenu),
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                _ => {}
            }
        }
		
		Some(MenuState::FeedsMenu)
    }

    fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
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
            self.nav_list.renderable(),
            chunks[1],
            &mut self.nav_list.state,
        );
    }
}
