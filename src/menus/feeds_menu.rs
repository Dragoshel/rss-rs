use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crossterm::event::{KeyCode, KeyEvent};

use mongodb::sync::Database;

use std::io::Stdout;

use rss::Channel;

use crate::{delete_feed, get_feeds, get_stories};

use super::{FeedsPopupMenu, Menu, MenuState, one_dark};

pub struct FeedsMenu<'a> {
    title: &'a str,
    feeds: Vec<Channel>,
    state: ListState,

    popup_menu: FeedsPopupMenu<'a>,

    database: &'a Database,
}

impl<'a> FeedsMenu<'a> {
    pub fn new(title: &'a str, database: &'a Database) -> crate::Result<Self> {
        Ok(FeedsMenu {
            title,
            feeds: get_feeds(database)?,
            state: ListState::default(),

            popup_menu: FeedsPopupMenu::new("Search for a Feed Online", database),

            database,
        })
    }

    fn feeds(&self) -> &[Channel] {
        &self.feeds
    }

    fn set_feeds(&mut self, feeds: impl Into<Vec<Channel>>) {
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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(80)])
            .split(f.size());

        // COMMANDS BOX
        let block = Block::default().title("Commands").borders(Borders::ALL);

        f.render_widget(block, chunks[0]);

        let subscribe_spans = Spans::from(vec![
            Span::styled("s     ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("subscribe to a new feed"),
        ]);

        let help_chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .margin(2)
            .split(chunks[0]);

        let delete_spans = Spans::from(vec![
            Span::styled("d     ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("delete a feed"),
        ]);

        let enter_spans = Spans::from(vec![
            Span::styled("ENTER ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("load news"),
        ]);

        let arrows_spans = Spans::from(vec![
            Span::styled("↑ ↓   ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("navigate UP and DOWN"),
        ]);

        let quit_spans = Spans::from(vec![
            Span::styled("ESC   ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("quit"),
        ]);

        let paragraph = Paragraph::new(vec![
            subscribe_spans,
            delete_spans,
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
            .margin(2)
            .split(chunks[1]);

        let channels: Vec<ListItem> = self
            .feeds
            .iter()
            .map(|c| ListItem::new(c.title()))
            .collect();

        let list = List::new(channels)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(one_dark(Color::LightBlue)))
            .highlight_symbol("> ");

        f.render_stateful_widget(list, feeds_chunks[0], &mut self.state);
        // FEEDS LIST

        // POPUP
        if self.popup_menu.popped {
            self.popup_menu.draw(f);
        }
        // POPUP
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        if self.popup_menu.popped {
            self.popup_menu.transition(key_event);
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

                KeyCode::Char('s') => {
                    self.popup_menu.popped = true;
                }

                KeyCode::Char('d') => {
                    if let Some(selected_state) = self.state.selected() {
                        let selected_feed = self.feeds.get(selected_state);

                        if let Some(selected) = selected_feed {
                            delete_feed(selected.title(), self.database).unwrap();
                        }
                    }
                    self.refresh().unwrap();
                }

                KeyCode::Enter => {
                    if let Some(selected_state) = self.state.selected() {
                        let selected_feed = self.feeds.get(selected_state);

                        if let Some(selected) = selected_feed {
							let stories = get_stories(selected.title(), self.database).unwrap();
                            return MenuState::Stories(stories);
                        }
                    }
                }

                _ => {}
            }
        }

        MenuState::Feeds
    }

    fn refresh(&mut self) -> crate::Result<()> {
        self.feeds = get_feeds(self.database)?;

        Ok(())
    }

    fn state(&mut self) -> MenuState {
        MenuState::Feeds
    }
}
