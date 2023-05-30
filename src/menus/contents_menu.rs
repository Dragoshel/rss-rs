use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

use crossterm::event::{KeyCode, KeyEvent};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::models::Item;

use super::{Menu, MenuState};

#[derive(Default)]
pub struct ContentsMenu<'a> {
    title: &'a str,
    text: String,
    scroll: usize,
}

impl<'a> ContentsMenu<'a> {
    pub fn new(item: Item) -> Self {
        ContentsMenu {
            title: "Contents of your story",
            text: item.description.unwrap_or_default(),
            scroll: 0,
        }
    }
}

impl<'a> Menu for ContentsMenu<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(f.size());

        let block = Block::default()
			.title("Help")
			.borders(Borders::ALL);
        let spans = Spans::from(vec![
            Span::styled(
                "↑ ↓",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC),
            ),
            Span::from(", "),
            Span::styled(
                "j k",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC),
            ),
            Span::from(" to move up or down"),
        ]);
        let help = Paragraph::new(spans)
			.block(block)
			.wrap(Wrap { trim: true });
		f.render_widget(help, chunks[0]);

        let block = Block::default()
			.borders(Borders::ALL)
			.title(self.title);
        let paragraph = Paragraph::new(self.text.to_string())
            .block(block)
            .wrap(Wrap { trim: true })
            .scroll((self.scroll as u16, 0));
		f.render_widget(paragraph, chunks[1]);
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        match key_event.code {
            KeyCode::Esc => {
                return MenuState::Exit;
            }

            KeyCode::Char('h') => {
                return MenuState::Stories(None);
            }

            KeyCode::Enter => {
                return MenuState::Stories(None);
            }

            _ => {}
        }

        MenuState::Contents(None)
    }

    fn state(&mut self) -> MenuState {
        MenuState::Contents(None)
    }
}
