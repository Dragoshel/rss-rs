use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Tabs, Wrap};
use tui::Frame;

use std::io::Stdout;

use crossterm::event::{KeyCode, KeyEvent};

use super::{centered_rect, one_dark, Menu, MenuState};

pub struct DeleteFeedPopup<'a> {
    title: &'a str,
    pub deleted: bool,
    pub popped: bool,
    choice: bool,
}

impl<'a> DeleteFeedPopup<'a> {
    pub fn new() -> Self {
        DeleteFeedPopup {
            title: "Confirm Deletion",
            deleted: false,
            popped: false,
            choice: false,
        }
    }
}

impl<'a> Menu for DeleteFeedPopup<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let popup_area = centered_rect(40, 15, f.size());

        let chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .split(popup_area);

        let popup_chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .margin(2)
            .split(chunks[0]);

        let block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .style(Style::default().bg(one_dark(Color::DarkGray)));
        f.render_widget(block, chunks[0]);

        let paragraph =
            Paragraph::new("Are you sure you want to delete the feed?").wrap(Wrap { trim: true });
        f.render_widget(paragraph, popup_chunks[0]);

        let tabs = Tabs::new(vec![Spans::from("Back"), Spans::from("Confirm")])
            .select(self.choice as usize)
            .style(Style::default().fg(one_dark(Color::Green)))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(one_dark(Color::Green))
                    .fg(one_dark(Color::DarkGray)),
            );
        f.render_widget(tabs, popup_chunks[1]);
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        match key_event.code {
            KeyCode::Esc => {
                self.reload().unwrap();
            }

            KeyCode::Left => {
                self.choice = false;
            }

            KeyCode::Right => {
                self.choice = true;
            }

            KeyCode::Enter => {
                self.deleted = self.choice;
                self.reload().unwrap();
            }

            _ => {}
        }
        // Fallback if none of the keys were pressed
        self.state()
    }

    fn reload(&mut self) -> crate::error::Result<()> {
        self.choice = false;
        self.popped = false;
        Ok(())
    }

    fn state(&mut self) -> MenuState {
        MenuState::Feeds
    }

    fn observer(&mut self) {}
}
