use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget};

use crossterm::event::{KeyCode, KeyEvent};

use crate::models::Item;

#[derive(Default)]
pub struct ItemList<'a> {
    pub title: &'a str,
    pub items: Vec<Item>,
    pub state: ListState,
}

pub struct RenderItemList<'a> {
    title: &'a str,
    items: Vec<ListItem<'a>>,
}

impl<'a> ItemList<'a> {
    pub fn new(title: &'a str, items: Vec<Item>) -> Self {
        ItemList {
            title,
            items,
            state: ListState::default(),
        }
    }

    pub fn as_render(&self) -> RenderItemList<'a> {
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|i| ListItem::new(i.title.clone().unwrap().to_string()))
            .collect();

        RenderItemList {
            title: self.title,
            items,
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
                if i >= self.items.len() - 1 {
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
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl<'a> StatefulWidget for RenderItemList<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().title(self.title).borders(Borders::ALL);
        let list = List::new(self.items)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        StatefulWidget::render(list, area, buf, state);
    }
}
