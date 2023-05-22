use tui::style::{Style, Color, Modifier};
use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::{ListItem, ListState, List, Block, Borders, StatefulWidget};

use crossterm::event::{KeyCode, KeyEvent};

#[derive(Default)]
pub struct NavList<'a> {
	pub title: &'a str,
	pub items: Vec<String>,
	pub state: ListState
}

pub struct RenderNavList<'a> {
	title: &'a str,
	items: Vec<ListItem<'a>>,
}

impl<'a> NavList<'a> {
	pub fn new(title: &'a str, items: Vec<String>) -> NavList {
		NavList {
			title,
			items,
			state: ListState::default()
		}
	}

	pub fn as_render(&self) -> RenderNavList<'a> {
		let items: Vec<ListItem> = self.items
			.iter()
			.map(|i| ListItem::new(i.to_string()))
			.collect();

		RenderNavList {
			title: self.title,
			items
		}
	}
	
	pub fn navigate(&mut self, key: KeyEvent) {
		match key.code {
			KeyCode::Char('j') => {
				self.next();	
			},
			KeyCode::Char('k') => {
				self.previous();
			},
			KeyCode::Down => {
				self.next();	
			},
			KeyCode::Up => {
				self.previous();
			},
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

impl<'a> StatefulWidget for RenderNavList<'a> {
	type State = ListState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		let block = Block::default()
			.title(self.title)
			.borders(Borders::ALL);
		let list = List::new(self.items)
			.block(block)
			.style(Style::default().fg(Color::White))
			.highlight_style(Style::default().add_modifier(Modifier::ITALIC))
			.highlight_symbol(">>");

		StatefulWidget::render(list, area, buf, state);
	}
}
