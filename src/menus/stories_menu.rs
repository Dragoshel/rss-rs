use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crossterm::event::{KeyCode, KeyEvent};

use mongodb::bson::doc;
use mongodb::sync::Database;

use crate::models::{find_one_feed, update_one_feed, Feed};

use super::{one_dark, Menu, MenuState};

pub struct StoriesMenu<'a> {
    title: &'a str,
    feed: Feed,
    state: ListState,

    db: &'a Database,
}

impl<'a> StoriesMenu<'a> {
    pub fn new(db: &'a Database) -> Self {
        StoriesMenu {
            title: "Your Stories",
            feed: Feed::default(),
            state: ListState::default(),

            db,
        }
    }

    pub fn feed(&self) -> &Feed {
        &self.feed
    }

    pub fn set_feed(&mut self, feed: impl Into<Feed>) {
        self.feed = feed.into()
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.feed.stories().len() - 1 {
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
                    self.feed.stories().len() - 1
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
        let background = Block::default().style(Style::default().bg(one_dark(Color::Black)));
        f.render_widget(background, f.size());

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
            .feed
            .stories()
            .iter()
            .map(|s| {
                let title = s.title().unwrap_or_default();
                let color = if s.read {
                    Color::DarkGray
                } else {
                    Color::White
                };
                ListItem::new(title).style(Style::default().fg(color))
            })
            .collect();

        let list = List::new(items)
            .highlight_style(Style::default().fg(one_dark(Color::LightBlue)))
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
                if let Some(selected) = self.state.selected() {
                    if let Some(story) = self.feed.stories().get(selected) {
                        // Function for setting a story as read
                        update_one_feed(
                            doc! {
                                "stories._id": story.id
                            },
                            doc! {
                                "$set": {
                                    "stories.$.read": true
                                }
                            },
                            self.db,
                        )
                        .unwrap();

                        return MenuState::Contents(Some(story.clone()));
                    }
                }
            }
            _ => {}
        }
        // Fallback if none of the keys were pressed
        self.state()
    }

    fn refresh(&mut self) -> crate::error::Result<()> {
        self.feed = find_one_feed(Some(doc! {"_id": self.feed.id}), self.db)?.unwrap();
        Ok(())
    }

    fn state(&mut self) -> MenuState {
        MenuState::Stories(None)
    }
}
