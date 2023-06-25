use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Tabs, Wrap};
use tui::Frame;

use std::io::Stdout;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{models::Feed, util::fetch_feed};

use super::{centered_rect, one_dark, Menu, MenuState};

pub struct SubscribePopup<'a> {
    title: &'a str,
    feed: Option<Feed>,

    pub popped: bool,
    pub subscribed: bool,
    fetched: bool,

    choice: bool,
    input: String,
}

impl<'a> SubscribePopup<'a> {
    pub fn new() -> Self {
        SubscribePopup {
            title: "Search for a Feed Online",
            popped: false,
            subscribed: false,
            fetched: false,
            feed: None,
            choice: false,
            input: String::new(),
        }
    }

    pub fn feed(&self) -> Option<&Feed> {
        self.feed.as_ref()
    }

    pub fn set_feed(&mut self, feed: impl Into<Option<Feed>>) {
        self.feed = feed.into();
    }
}

impl<'a> Menu for SubscribePopup<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let popup_area = centered_rect(40, 30, f.size());

        let chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(popup_area);

        let mut input_container = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .style(Style::default().bg(one_dark(Color::DarkGray)));

        let input_chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .margin(1)
            .horizontal_margin(3)
            .split(chunks[0]);

        let input_block = Block::default().borders(Borders::ALL);

        let input = Paragraph::new(self.input.to_string())
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(one_dark(Color::Green)))
            .block(input_block);

        f.render_widget(input, input_chunks[0]);

        if self.fetched {
            input_container =
                input_container.borders(Borders::TOP | Borders::RIGHT | Borders::LEFT);

            let feed_container = Block::default()
                .borders(Borders::BOTTOM | Borders::RIGHT | Borders::LEFT)
                .style(Style::default().bg(one_dark(Color::DarkGray)));

            f.render_widget(feed_container, chunks[1]);

            let feed_chunks = Layout::default()
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ])
                .margin(1)
                .horizontal_margin(3)
                .split(chunks[1]);

            if let Some(feed) = &self.feed {
                let story_count = feed.stories().len().to_string();
                let story_count = format!(" ({})", story_count);
                let title_spans = Spans::from(vec![
                    Span::styled("[Title:] ", Style::default().fg(one_dark(Color::Gray))),
                    Span::raw(feed.title()),
                    Span::styled(story_count, Style::default().fg(one_dark(Color::Gray))),
                ]);
                let title = Paragraph::new(title_spans).wrap(Wrap { trim: true });
                f.render_widget(title, feed_chunks[0]);

                let description_spans = Spans::from(vec![
                    Span::styled(
                        "[Description:] ",
                        Style::default().fg(one_dark(Color::Gray)),
                    ),
                    Span::raw(feed.description()),
                ]);
                let description = Paragraph::new(description_spans).wrap(Wrap { trim: true });
                f.render_widget(description, feed_chunks[1]);
            }

            let tabs = Tabs::new(vec![Spans::from("Back"), Spans::from("Subscribe")])
                .select(self.choice as usize)
                .style(Style::default().fg(one_dark(Color::Green)))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(one_dark(Color::Green))
                        .fg(one_dark(Color::DarkGray)),
                );
            f.render_widget(tabs, feed_chunks[2]);
        }
        f.render_widget(input_container, chunks[0]);
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        match key_event.code {
            KeyCode::Esc => {
                self.reload().unwrap();
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
                self.choice = false;
            }

            KeyCode::Right => {
                self.choice = true;
            }

            KeyCode::Enter => {
                if !self.fetched {
                    self.feed = match fetch_feed(&self.input) {
                        Ok(feed) => Some(feed),
                        Err(error) => {
                            let mut feed_error = Feed::default();
                            feed_error.set_description(error.to_string());
                            Some(feed_error)
                        }
                    };
                    self.fetched = true;
                } else {
                    self.subscribed = self.choice;
                    self.reload().unwrap();
                }
            }

            _ => {}
        }
        // Fallback if none of the keys were pressed
        self.state()
    }

    fn reload(&mut self) -> crate::error::Result<()> {
        self.popped = false;
        self.fetched = false;
        self.choice = false;
        self.input = String::new();
        Ok(())
    }

    fn state(&mut self) -> MenuState {
        MenuState::Feeds
    }

    fn observer(&mut self) {}
}
