use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap};

use crossterm::event::{KeyCode, KeyEvent};

use crate::models::Channel;
use crate::util::centered_rect;

use super::{Menu, MenuState};

#[derive(Default)]
enum PopupState {
    #[default]
    Editing,
    Choosing,
}

#[derive(Clone, Copy, Default)]
enum PopupChoice {
    #[default]
    Back = 0,
    Subscribe = 1,
}

#[derive(Default)]
pub struct FeedsMenu<'a> {
    title: &'a str,
    feeds: Vec<Channel>,
    state: ListState,

    popup_title: &'a str,
    popup_state: PopupState,
    popup_feed: Option<Channel>,
    popup_fetched: bool,
    popup_input: String,
    popup_choice: PopupChoice,
    popped: bool,
}

impl<'a> FeedsMenu<'a> {
    pub fn new(feeds: Vec<Channel>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        FeedsMenu {
            title: "Your Subscribed Feeds",
            feeds,
            state,

            popup_title: "Search for a Feed",
            popup_state: PopupState::Editing,
            popup_feed: None,
            popup_fetched: false,
            popup_input: String::new(),
            popup_choice: PopupChoice::Back,
            popped: false,
        }
    }

    fn clear(&mut self) {
        self.popped = false;
        self.popup_fetched = false;
        self.popup_input = String::new();
    }

    fn next(&mut self) {
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

    fn handle_popup_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.clear();
            }

            KeyCode::Char(c) => {
                self.popup_input.push(c);
                self.popup_fetched = false;
            }

            KeyCode::Backspace => {
                self.popup_input.pop();
                self.popup_fetched = false;
            }

            KeyCode::Left => {
                self.popup_choice = PopupChoice::Back;
            }

            KeyCode::Right => {
                self.popup_choice = PopupChoice::Subscribe;
            }

            KeyCode::Enter => {
                if self.popup_fetched {
                    match self.popup_choice {
                        PopupChoice::Back => {}
                        PopupChoice::Subscribe => {
                            // Insert into database MONOGO DBBB
                        }
                    }
                    self.clear();
                } else {
                    match Channel::fetch_required(&self.popup_input) {
                        Ok(channel) => {
                            self.popup_feed = Some(channel);
                        }
                        Err(error) => {
                            let mut error_channel = Channel::default();
                            error_channel.title = error.to_string();
                            self.popup_feed = Some(error_channel);
                        }
                    }
                    self.popup_fetched = true;
                }
            }

            _ => {}
        }
    }
}

impl<'a> Menu for FeedsMenu<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(100)])
            .split(f.size());

        // FEEDS MENU
        let channels: Vec<ListItem> = self
            .feeds
            .iter()
            .map(|c| ListItem::new(c.title.to_string()))
            .collect();
        let block = Block::default().title(self.title).borders(Borders::ALL);
        let list = List::new(channels)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        f.render_stateful_widget(list, chunks[0], &mut self.state);
        // FEEDS MENU

        // POPUP
        if self.popped {
            let popup_area = centered_rect(50, 50, chunks[0]);
            let chunks = Layout::default()
                .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(popup_area);

            let mut input_container = Block::default()
                .title(self.title)
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Blue));
            let input_chunks = Layout::default()
                .constraints(vec![Constraint::Percentage(100)])
                .margin(1)
                .split(chunks[0]);
            let input_block = Block::default().borders(Borders::ALL);
            let input = Paragraph::new(self.popup_input.to_string())
                .wrap(Wrap { trim: true })
                .block(input_block);
            f.render_widget(input, input_chunks[0]);

            if self.popup_fetched {
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
                    .margin(2)
                    .split(chunks[1]);

                if let Some(channel) = &self.popup_feed {
                    let title = Paragraph::new(channel.title.to_string()).wrap(Wrap { trim: true });
                    f.render_widget(title, channel_chunks[0]);

                    let description =
                        Paragraph::new(channel.description.to_string()).wrap(Wrap { trim: true });
                    f.render_widget(description, channel_chunks[1]);
                }

                let tabs = Tabs::new(vec![Spans::from("Back"), Spans::from("Subscribe")])
                    .select(self.popup_choice as usize)
                    .style(Style::default().fg(Color::Yellow))
                    .highlight_style(
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .bg(Color::Black),
                    );
                f.render_widget(tabs, channel_chunks[2]);
            }
            f.render_widget(input_container, chunks[0]);
        }
        // POPUP
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        if self.popped {
            self.handle_popup_events(key_event);
        } else {
            // FEEDS MENU
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
                    self.popped = true;
                }

                KeyCode::Enter => {
                    if let Some(selected_state) = self.state.selected() {
                        let selected_story = self.feeds.get(selected_state);

                        if let Some(selected) = selected_story {
                            return MenuState::Stories(Some(selected.clone()));
                        }
                    }
                }

                _ => {}
            }
            // FEEDS MENU
        }

        MenuState::Feeds
    }

    fn state(&mut self) -> MenuState {
        MenuState::Feeds
    }
}
