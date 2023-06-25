mod contents_menu;
mod delete_feed_popup;
mod feeds_menu;
mod stories_menu;
mod subscribe_popup;

pub use contents_menu::ContentsMenu;
pub use delete_feed_popup::DeleteFeedPopup;
pub use feeds_menu::FeedsMenu;
pub use stories_menu::StoriesMenu;
pub use subscribe_popup::SubscribePopup;

use std::io::Stdout;

use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;
use tui::{backend::CrosstermBackend, style::Color};

use crossterm::event::KeyEvent;

use crate::models::{Feed, Story};

pub enum MenuState {
    Feeds,
    Stories(Option<Feed>),
    Contents(Option<Story>),
    Exit,
}

pub trait Menu {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>);
    fn transition(&mut self, key_event: KeyEvent) -> MenuState;
    fn observer(&mut self);
    fn reload(&mut self) -> crate::error::Result<()>;
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
        Color::Gray => Color::Rgb(125, 131, 142),
        Color::DarkGray => Color::Rgb(32, 32, 40),
        _ => color,
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
