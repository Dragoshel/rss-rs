use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

pub struct HelpBox {}

pub struct RenderHelpBox {}

impl HelpBox {
	pub fn as_render(&self) -> RenderHelpBox {
		RenderHelpBox {}
	}
}

impl Widget for RenderHelpBox {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let block = Block::default().title("Help").borders(Borders::ALL);
		let spans = Spans::from(vec![
			Span::styled(
				"↑ ↓",
				Style::default()
					.fg(Color::Yellow)
					.add_modifier(Modifier::ITALIC),
			),
			Span::from(", "),
			Span::styled(
				"j k",
				Style::default()
					.fg(Color::Yellow)
					.add_modifier(Modifier::ITALIC),
			),
			Span::from(" to move up or down"),
		]);
		let paragraph = Paragraph::new(spans).block(block).wrap(Wrap { trim: true });

		paragraph.render(area, buf);
	}
}
