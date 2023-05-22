use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;

use crossterm::event::{KeyCode, KeyEvent};

use crate::widgets::NavList;

use super::{Menu, MenuState};

#[derive(Default)]
pub struct StoriesMenu<'a> {
    nav_list: NavList<'a>,
}

impl<'a> StoriesMenu<'a> {
    pub fn new(items: Vec<String>) -> StoriesMenu<'a> {
        StoriesMenu {
            nav_list: NavList::new("Stories", items),
        }
    }
}

impl<'a> Menu for StoriesMenu<'a> {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        f.render_stateful_widget(
            self.nav_list.as_render(),
            chunks[0],
            &mut self.nav_list.state,
        );
    }

    fn transition(&mut self, key_event: KeyEvent) -> MenuState {
        match key_event.code {
            KeyCode::Char('q') => return MenuState::Exit,
            KeyCode::Enter => return MenuState::Feeds,
            _ => {}
        }

        MenuState::Stories
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.nav_list.navigate(key_event);
    }

    fn get_state(&mut self) -> MenuState {
        MenuState::Stories
    }
}
