use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

use std::io::Stdout;

use crossterm::event::{KeyCode, KeyEvent};

use mongodb::sync::Database;

use crate::{util::fetch_feed, models::{Feed, insert_one_feed}};

use super::{Menu, MenuState, centered_rect, one_dark};

pub struct FeedsPopupMenu<'a> {
    title: &'a str,
    feed: Option<Feed>,

    pub popped: bool,
    fetched: bool,

    choice: usize,
    input: String,

    db: &'a Database,
}

impl<'a> FeedsPopupMenu<'a> {
    pub fn new(title: &'a str, db: &'a Database) -> Self {
        FeedsPopupMenu {
            title,
            popped: false,
            fetched: false,
            feed: None,
            choice: 0,
            input: String::new(),

            db,
        }
    }

    pub fn feed(&self) -> Option<&Feed> {
        self.feed.as_ref()
    }

    pub fn set_feed(&mut self, feed: impl Into<Option<Feed>>) {
        self.feed = feed.into();
    }

    fn input(&self) -> &str {
        self.input.as_str()
    }

    fn set_input(&mut self, input: impl Into<String>) {
        self.input = input.into();
    }
}

impl<'a> Menu for FeedsPopupMenu<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let popup_area = centered_rect(40, 30, f.size());

        let chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(popup_area);

        let mut input_container = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Blue));

        let input_chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .margin(1)
            .horizontal_margin(3)
            .split(chunks[0]);

        let input_block = Block::default().borders(Borders::ALL);

        let input = Paragraph::new(self.input.to_string())
            .wrap(Wrap { trim: true })
            .block(input_block);

        f.render_widget(input, input_chunks[0]);

        if self.fetched {
            input_container =
                input_container.borders(Borders::TOP | Borders::RIGHT | Borders::LEFT);

            let channel_container = Block::default()
                .borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT)
                .style(Style::default().bg(Color::Blue));

            f.render_widget(channel_container, chunks[1]);

            let channel_chunks = Layout::default()
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ])
                .margin(1)
                .horizontal_margin(3)
                .split(chunks[1]);

            if let Some(channel) = &self.feed {
                let title = Paragraph::new(channel.title()).wrap(Wrap { trim: true });
                f.render_widget(title, channel_chunks[0]);

                let description = Paragraph::new(channel.description()).wrap(Wrap { trim: true });
                f.render_widget(description, channel_chunks[1]);
            }

            let tabs = Tabs::new(vec![Spans::from("Back"), Spans::from("Subscribe")])
                .select(self.choice as usize)
                .style(Style::default().fg(Color::Yellow))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(one_dark(Color::Black)),
                );
            f.render_widget(tabs, channel_chunks[2]);
        }
        f.render_widget(input_container, chunks[0]);
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        match key_event.code {
            KeyCode::Esc => {
                self.refresh();
            }

            KeyCode::Char(c) => {
                self.input.push(c);
                self.fetched = false;
            }

            KeyCode::Backspace => {
                self.input.pop();
                self.fetched = false;
            }

            KeyCode::Left => {
                self.choice = 0;
            }

            KeyCode::Right => {
                self.choice = 1;
            }

            KeyCode::Enter => {
                if self.fetched {
                    // SUBSCRIBING TO URL
                    match self.choice {
                        1 => {
                            if let Some(feed) = self.feed() {
								insert_one_feed(feed, self.db).unwrap();
                            }
                        }
                        _ => {}
                    }

                    self.refresh();
                } else {
                    // FETCHING FEED BY URL
                    match fetch_feed(&self.input) {
                        Ok(feed) => {
                            self.feed = Some(feed);
                        }
                        Err(error) => {
							self.feed = None;
                        }
                    }

                    self.fetched = true;
                }
            }

            _ => {}
        }

        MenuState::Feeds
    }

    fn refresh(&mut self) -> crate::Result<()> {
        self.popped = false;
        self.fetched = false;
        self.feed = None;
        self.choice = 0;
        self.input = String::new();

        Ok(())
    }

    fn state(&mut self) -> MenuState {
        MenuState::Feeds
    }
}
