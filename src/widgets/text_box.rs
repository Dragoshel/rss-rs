use crossterm::event::{KeyEvent, KeyCode};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

#[derive(Default)]
pub struct TextBox<'a> {
	pub title: &'a str,
	pub text: String,
	pub scroll: u16,
}

pub struct RenderTextBox<'a> {
	pub title: &'a str,
	pub text: String,
	pub scroll: u16,
}

impl<'a> TextBox<'a> {
	pub fn new(title: &'a str, text: String) -> Self {
		TextBox {
			title,
			text,
			scroll: 0,
		}
	}

	pub fn as_render(&self) -> RenderTextBox {
		RenderTextBox {
			title: self.title,
			text: self.text.to_string(),
			scroll: self.scroll,
		}
	}

	pub fn navigate(&mut self, key: KeyEvent) {
		match key.code {
			KeyCode::Char('j') => self.scroll = self.scroll + 1,
			KeyCode::Char('k') => {
				if self.scroll < 1 {
					self.scroll = 0;
				} else {
					self.scroll = self.scroll - 1
				}
			}
			_ => {}
		}
	}
}

impl<'a> Widget for RenderTextBox<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let text = Text::from(self.text);
		let block = Block::default()
			.title(text.height().to_string())
			.borders(Borders::ALL);
		let paragraph = Paragraph::new(text)
			.block(block)
			.wrap(Wrap { trim: true })
			.scroll((self.scroll, 0));

		paragraph.render(area, buf);
	}
}
