use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use mongodb::sync::Database;
use mongodb::bson::doc;

use std::io::Stdout;

use crossterm::event::{KeyCode, KeyEvent};

use crate::models::{delete_one_feed, find_many_feed, insert_one_feed, Feed};

use super::{one_dark, DeleteFeedPopup, Menu, MenuState, SubscribePopup};

pub struct FeedsMenu<'a> {
    title: &'a str,
    feeds: Vec<Feed>,
    state: ListState,

    subscribe_popup: SubscribePopup<'a>,
    delete_feed_popup: DeleteFeedPopup<'a>,

    db: &'a Database,
}

impl<'a> FeedsMenu<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self {
            title: "Your Feeds",
            feeds: vec![],
            state: ListState::default(),

            subscribe_popup: SubscribePopup::new(),
            delete_feed_popup: DeleteFeedPopup::new(),

            db,
        }
    }

    pub fn feeds(&self) -> &[Feed] {
        &self.feeds
    }

    pub fn set_feeds(&mut self, feeds: impl Into<Vec<Feed>>) {
        self.feeds = feeds.into();
    }

    fn next(&mut self) {
        if self.feeds.is_empty() {
            self.state.select(None);
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.feeds.len() - 1 {
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
        if self.feeds.is_empty() {
            self.state.select(None);
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.feeds.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl<'a> Menu for FeedsMenu<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let background = Block::default().style(Style::default().bg(one_dark(Color::Black)));
        f.render_widget(background, f.size());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(80)])
            .split(f.size());

        // COMMANDS BOX
        let block = Block::default().title("Commands").borders(Borders::ALL);

        f.render_widget(block, chunks[0]);

        let subscribe_delete_spans = Spans::from(vec![
            Span::styled("S D   ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("Subscribe/Delete Feed"),
        ]);

        let help_chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .margin(2)
            .split(chunks[0]);

        let enter_spans = Spans::from(vec![
            Span::styled("ENTER ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("Load Stories"),
        ]);

        let arrows_spans = Spans::from(vec![
            Span::styled("↑ ↓   ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("Navigate Up and Down"),
        ]);

        let quit_spans = Spans::from(vec![
            Span::styled("ESC   ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("Quit"),
        ]);

        let paragraph = Paragraph::new(vec![
            subscribe_delete_spans,
            enter_spans,
            Spans::from(""),
            arrows_spans,
            quit_spans,
        ])
        .wrap(Wrap { trim: true });

        f.render_widget(paragraph, help_chunks[0]);
        // COMMANDS BOX

        // FEEDS LIST
        let block = Block::default().title(self.title).borders(Borders::ALL);

        f.render_widget(block, chunks[1]);

        let feeds_chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .margin(1)
            .split(chunks[1]);

        let feeds: Vec<ListItem> = self
            .feeds
            .iter()
            .map(|f| {
                let story_count = f.stories().len().to_string();
                let story_count = format!(" ({})", story_count);
                let title_spans = Spans::from(vec![
                    Span::raw(f.title()),
                    Span::styled(story_count, Style::default().fg(one_dark(Color::Gray))),
                ]);
                ListItem::new(title_spans)
            })
            .collect();

        let list = List::new(feeds)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(one_dark(Color::LightBlue))
                    .fg(one_dark(Color::Black)),
            );

        f.render_stateful_widget(list, feeds_chunks[0], &mut self.state);
        // FEEDS LIST

        // POPUP
        if self.subscribe_popup.popped {
            self.subscribe_popup.draw(f);
        } else if self.delete_feed_popup.popped {
            self.delete_feed_popup.draw(f);
        }
        // POPUP
    }

    fn observer(&mut self) {
        if self.subscribe_popup.subscribed {
            if let Some(feed) = self.subscribe_popup.feed() {
                insert_one_feed(feed, self.db).unwrap();
                self.reload().unwrap();
                self.subscribe_popup.subscribed = false;
            }
            self.delete_feed_popup.deleted = false;
        }

        if self.delete_feed_popup.deleted {
            if let Some(selected) = self.state.selected() {
                if let Some(feed) = self.feeds.get(selected) {
                    delete_one_feed(doc! {"_id": feed.id}, self.db).unwrap();
                    self.reload().unwrap();
                }
            }
            self.delete_feed_popup.deleted = false;
        }
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        if self.subscribe_popup.popped {
            self.subscribe_popup.transition(key_event);
        } else if self.delete_feed_popup.popped {
            self.delete_feed_popup.transition(key_event);
        } else {
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

                KeyCode::Enter => {
                    if let Some(selected) = self.state.selected() {
                        if let Some(feed) = self.feeds.get(selected) {
                            return MenuState::Stories(Some(feed.clone()));
                        }
                    }
                }

                KeyCode::Char('s') => {
                    self.subscribe_popup.popped = true;
                }

                KeyCode::Char('r') => {
                    self.reload().unwrap();
                }

                KeyCode::Char('d') => {
                    self.delete_feed_popup.popped = true;
                }

                _ => {}
            }
        }
        // Fallback if none of the keys were pressed
        self.state()
    }

    fn reload(&mut self) -> crate::error::Result<()> {
        self.feeds = find_many_feed(None, self.db)?;
        Ok(())
    }

    fn state(&mut self) -> MenuState {
        MenuState::Feeds
    }
}
