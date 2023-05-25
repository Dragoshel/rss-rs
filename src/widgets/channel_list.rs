use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget};

use crossterm::event::{KeyCode, KeyEvent};

use crate::models::Channel;

#[derive(Default)]
pub struct ChannelList<'a> {
    pub title: &'a str,
    pub channels: Vec<Channel>,
    pub state: ListState,
}

pub struct RenderChannelList<'a> {
    title: &'a str,
    channels: Vec<ListItem<'a>>,
}

impl<'a> ChannelList<'a> {
    pub fn new(title: &'a str, channels: Vec<Channel>) -> Self {
        ChannelList {
            title,
            channels,
            state: ListState::default(),
        }
    }

    pub fn as_render(&self) -> RenderChannelList<'a> {
        let channels: Vec<ListItem> = self.channels
            .iter()
            .map(|c| ListItem::new(c.title.to_string()))
            .collect();

        RenderChannelList {
            title: self.title,
            channels,
        }
    }

    pub fn navigate(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('j') => {
                self.next();
            }
            KeyCode::Char('k') => {
                self.previous();
            }
            _ => {}
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.channels.len() - 1 {
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
                    self.channels.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl<'a> StatefulWidget for RenderChannelList<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().title(self.title).borders(Borders::ALL);
        let list = List::new(self.channels)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        StatefulWidget::render(list, area, buf, state);
    }
}
