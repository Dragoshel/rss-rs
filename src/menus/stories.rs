use crossterm::event::{Event, KeyCode};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;

use crate::widgets::NavList;

use super::{Menu, MenuState};

pub struct StoriesMenu<'a> {
	nav_list: NavList<'a>
}

impl<'a> StoriesMenu<'a> {
	pub fn new(items: Vec<String>) -> StoriesMenu<'a> {
		StoriesMenu {
			nav_list: NavList::new("Stories", items)
		}
	}
}

impl<'a> Menu for StoriesMenu<'a> {
	fn transition(&mut self, event: Event) -> Option<MenuState> {
        if let Event::Key(key) = event {
            self.nav_list.navigate(key);

            match key.code {
                KeyCode::Char('q') => return None,
                KeyCode::Enter => return Some(MenuState::FeedsMenu),
                _ => {}
            }
        }

		Some(MenuState::StoriesMenu)
    }	

    fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        f.render_stateful_widget(
            self.nav_list.renderable(),
            chunks[0],
            &mut self.nav_list.state,
        );
    }
}
