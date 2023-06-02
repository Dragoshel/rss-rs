use std::io::Stdout;
use std::ops::Deref;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap};

use crossterm::event::{KeyCode, KeyEvent};

use crate::models::Channel;
use crate::mongo::{delete_feed, get_all_feeds, insert_feed};
use crate::util::{centered_rect, one_dark};

use super::{Menu, MenuState};

#[derive(Clone, Copy, Default)]
enum PopupChoice {
    #[default]
    Back,
    Subscribe,
}

#[derive(Default)]
pub struct FeedsMenu<'a> {
    title: &'a str,
    feeds: Vec<Channel>,
    state: ListState,

    popup_title: &'a str,
    popup_feed: Option<Channel>,
    popup_fetched: bool,
    popup_input: String,
    popup_choice: PopupChoice,
    popped: bool,
}

impl<'a> FeedsMenu<'a> {
    pub fn new(title: &'a str) -> Self {
        FeedsMenu {
            title,
            feeds: vec![],
            state: ListState::default(),

            popup_title: "Search for a Feed",
            popup_feed: None,
            popup_fetched: false,
            popup_input: String::new(),
            popup_choice: PopupChoice::Back,
            popped: false,
        }
    }

    pub fn init(&mut self) -> crate::Result<()> {
        self.feeds = get_all_feeds()?;
        Ok(())
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

    fn exit_popup(&mut self) {
        self.popped = false;
        self.popup_fetched = false;
        self.popup_input = String::new();
    }

    fn handle_popup_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.exit_popup();
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
                    // SUBSCRIBING TO URL
                    match self.popup_choice {
                        PopupChoice::Back => {}
                        PopupChoice::Subscribe => {
                            if let Some(channel) = &self.popup_feed {
                                insert_feed(channel).unwrap();
                            }
                            self.init().unwrap();
                        }
                    }
                    self.exit_popup();
                } else {
                    // FETCHING FEED BY URL
                    match Channel::fetch_required(&self.popup_input) {
                        Ok(mut channel) => {
                            channel.rss_link = Some(self.popup_input.to_string());
                            self.popup_feed = Some(channel);
                        }
                        Err(error) => {
                            // [TODO]
                            // CHANGE TO PROPER ERROR REPORTING
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
            .map(|c| ListItem::new(c.title.to_string()))
            .collect();

        let list = List::new(channels)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("> ");

        f.render_stateful_widget(list, feeds_chunks[0], &mut self.state);
        // FEEDS LIST

        // POPUP
        if self.popped {
            let popup_area = centered_rect(40, 30, f.size());

            let chunks = Layout::default()
                .constraints(vec![Constraint::Percentage(35), Constraint::Percentage(65)])
                .split(popup_area);

            let mut input_container = Block::default()
                .title(self.popup_title)
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Blue));

            let input_chunks = Layout::default()
                .constraints(vec![Constraint::Percentage(100)])
                .margin(1)
				.horizontal_margin(3)
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
                    .margin(1)
					.horizontal_margin(3)
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
                            .bg(one_dark(Color::Black)),
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

                KeyCode::Char('d') => {
                    if let Some(selected_state) = self.state.selected() {
                        let selected_feed = self.feeds.get(selected_state);

                        if let Some(selected) = selected_feed {
                            delete_feed(selected.title.deref()).unwrap();
                        }
                    }
                    self.init().unwrap();
                }

                KeyCode::Enter => {
                    if let Some(selected_state) = self.state.selected() {
                        let selected_feed = self.feeds.get(selected_state);

                        if let Some(selected) = selected_feed {
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
