use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};

use crossterm::event::{KeyCode, KeyEvent};

use crate::models::Story;

use super::{one_dark, Menu, MenuState};

pub struct ContentsMenu {
    story: Story,

    scroll: usize,
}

impl ContentsMenu {
    pub fn new() -> Self {
        ContentsMenu {
            story: Story::default(),
            scroll: 0,
        }
    }

    pub fn get_story(&self) -> &Story {
        &self.story
    }

    pub fn set_story(&mut self, story: Story) {
        self.story = story;
    }
}

impl Menu for ContentsMenu {
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

        let quit_spans = Spans::from(vec![
            Span::styled("ESC   ", Style::default().fg(one_dark(Color::Green))),
            Span::raw("quit"),
        ]);

        let paragraph = Paragraph::new(vec![back_spans, Spans::from(""), arrows_spans, quit_spans])
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, help_chunks[0]);
        // COMMANDS BOX

        // CONTENTS
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.story.title().unwrap());
        f.render_widget(block, chunks[1]);

        let contents_chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
            .margin(2)
            .split(chunks[1]);

        // META BOX
        let meta_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(contents_chunks[0]);

        let published_spans = Spans::from(vec![
            Span::styled("Published: ", Style::default().fg(one_dark(Color::Green))),
            Span::from(self.story.pub_date().unwrap_or_default()),
        ]);

        let paragraph = Paragraph::new(published_spans).wrap(Wrap { trim: true });

        f.render_widget(paragraph, meta_chunks[0]);

        let author_spans = Spans::from(vec![
            Span::styled("Author: ", Style::default().fg(one_dark(Color::Green))),
            Span::from(self.story.author().unwrap_or_default()),
        ]);

        let paragraph = Paragraph::new(author_spans).wrap(Wrap { trim: true });

        f.render_widget(paragraph, meta_chunks[2]);
        // META BOX

        let paragraph = Paragraph::new(self.story.content().unwrap_or_default())
            .wrap(Wrap { trim: true })
            .scroll((self.scroll as u16, 0));

        f.render_widget(paragraph, contents_chunks[1]);

        // CONTENTS
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        match key_event.code {
            KeyCode::Esc => {
                return MenuState::Exit;
            }

            KeyCode::Up => {
                if self.scroll > 0 {
                    self.scroll = self.scroll - 1;
                }
            }

            KeyCode::Down => {
                self.scroll = self.scroll + 1;
            }

            KeyCode::Left => {
                return MenuState::Stories(None);
            }
            _ => {}
        }

        MenuState::Contents(None)
    }

    fn refresh(&mut self) -> crate::Result<()> {
        todo!()
    }

    fn state(&mut self) -> MenuState {
        MenuState::Contents(None)
    }
}
