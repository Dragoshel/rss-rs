use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, List, ListItem, ListState};

use crossterm::event::{KeyCode, KeyEvent};

use super::{Menu, MenuState};

use crate::models::Item;

#[derive(Default)]
pub struct StoriesMenu<'a> {
    title: &'a str,
    stories: Vec<Item>,
    state: ListState,
}

impl<'a> StoriesMenu<'a> {
    pub fn new(stories: Vec<Item>) -> Self {
        StoriesMenu {
            title: "Latest Stories",
            stories,
            state: ListState::default(),
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.stories.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.stories.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl<'a> Menu for StoriesMenu<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(vec![Constraint::Percentage(100)])
            .split(f.size());

        let items: Vec<ListItem> = self
            .stories
            .iter()
            .map(|s| ListItem::new(s.title.clone().unwrap_or_default()))
            .collect();
        let block = Block::default().title(self.title).borders(Borders::ALL);
        let list = List::new(items)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        f.render_stateful_widget(list, chunks[0], &mut self.state);
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        match key_event.code {
            KeyCode::Esc => {
                return MenuState::Exit;
            }

            KeyCode::Up => {
            	self.previous();
			}

            KeyCode::Down => {
            	self.next();
			}

			KeyCode::Left => {
				return MenuState::Feeds;
			}

            KeyCode::Enter => {
                if let Some(selected_state) = self.state.selected() {
                    let selected_story = self.stories.get(selected_state);

                    if let Some(selected) = selected_story {
                        return MenuState::Contents(Some(selected.clone()));
                    }
                }
            }
            _ => {}
        }

        MenuState::Stories(None)
    }

    fn state(&mut self) -> MenuState {
        MenuState::Stories(None)
    }
}
