mod contents_menu;
mod feeds_menu;
mod stories_menu;
mod feeds_popup_menu;

pub use contents_menu::ContentsMenu;
pub use feeds_menu::FeedsMenu;
pub use stories_menu::StoriesMenu;
pub use feeds_popup_menu::FeedsPopupMenu;

use std::io::Stdout;

use tui::layout::{Rect, Direction, Constraint, Layout};
use tui::{backend::CrosstermBackend, style::Color};
use tui::Frame;

use crossterm::event::KeyEvent;

use crate::models::{Story, Feed};

pub enum MenuState {
	Feeds,
	Stories(Option<Feed>),
	Contents(Option<Story>),
	Exit,
}

pub trait Menu {
	fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>);
	fn transition(&mut self, key_event: KeyEvent) -> MenuState;
	fn refresh(&mut self) -> crate::error::Result<()>;
	fn state(&mut self) -> MenuState;
}

pub fn one_dark(color: Color) -> Color {
	match color {
		Color::Black => Color::Rgb(40, 44, 52),
		Color::Red => Color::Rgb(224, 108, 117),
		Color::Green => Color::Rgb(152, 195, 121),
		Color::Yellow => Color::Rgb(229, 192, 123),
		Color::Blue => Color::Rgb(97, 175, 239),
		Color::Magenta => Color::Rgb(198, 120, 221),
		Color::LightBlue => Color::Rgb(86, 182, 194),
		Color::White => Color::Rgb(171, 178, 191),
		_ => color
	}
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}