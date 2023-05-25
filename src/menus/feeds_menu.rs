use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};

use crossterm::event::{KeyCode, KeyEvent};

use crate::models::Channel;
use crate::widgets::ChannelList;

use super::{Menu, MenuState};

#[derive(Default)]
pub struct FeedsMenu<'a> {
    channel_list: ChannelList<'a>,
    input_buffer: String,
}

impl<'a> FeedsMenu<'a> {
    pub fn new(channels: Vec<Channel>) -> Self {
        FeedsMenu {
            channel_list: ChannelList::new("Your Feed", channels),
            input_buffer: String::new(),
        }
    }
}

impl<'a> Menu for FeedsMenu<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        let block = Block::default()
            .title("Get feed manually")
            .borders(Borders::ALL);
        let text = Text::from(self.input_buffer.as_ref());
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, chunks[0]);

        f.render_stateful_widget(
            self.channel_list.as_render(),
            chunks[1],
            &mut self.channel_list.state,
        );
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        match key_event.code {
            KeyCode::Char('q') => return MenuState::Exit,
            KeyCode::Char(key) => {
                self.input_buffer.push(key);
            }
            KeyCode::Enter => {
                let selected = self.channel_list.state.selected().unwrap();
                let selected = self.channel_list.channels.get(selected).unwrap();
                let mut dto_channel = Channel::default();
                dto_channel.title = selected.title.clone();
                dto_channel.link = selected.link.clone();
                dto_channel.description = selected.description.clone();

                return MenuState::Stories(Some(dto_channel));
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            _ => {}
        }

        MenuState::Feeds
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.channel_list.navigate(key_event);
    }

    fn state(&mut self) -> MenuState {
        MenuState::Feeds
    }
}
