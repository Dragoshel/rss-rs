use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crossterm::event::{KeyCode, KeyEvent};

use super::{Menu, MenuState};

use crate::models::Item;
use crate::util::one_dark;

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
            .margin(5)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(80)])
            .split(f.size());

        // COMMANDS BOX
        let block = Block::default().title("Commands").borders(Borders::ALL);

        f.render_widget(block, chunks[0]);

        let help_chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .margin(2)
            .split(chunks[0]);

        let back_spans = Spans::from(vec![
            Span::styled("←     ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("go back"),
        ]);

        let arrows_spans = Spans::from(vec![
            Span::styled("↑ ↓   ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("navigate UP and DOWN"),
        ]);

        let enter_spans = Spans::from(vec![
            Span::styled("ENTER ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("load story"),
        ]);

        let quit_spans = Spans::from(vec![
            Span::styled("ESC   ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("quit"),
        ]);

        let paragraph = Paragraph::new(vec![
            back_spans,
            enter_spans,
            Spans::from(""),
            arrows_spans,
            quit_spans,
        ])
        .wrap(Wrap { trim: true });

        f.render_widget(paragraph, help_chunks[0]);
        // COMMANDS BOX

        // STORIES LIST
        let block = Block::default().title(self.title).borders(Borders::ALL);

        f.render_widget(block, chunks[1]);

        let stories_chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .margin(2)
            .split(chunks[1]);

        let items: Vec<ListItem> = self
            .stories
            .iter()
            .map(|s| ListItem::new(s.title.clone().unwrap_or_default()))
            .collect();

        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("> ");
        // STORIES LIST

        f.render_stateful_widget(list, stories_chunks[0], &mut self.state);
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
