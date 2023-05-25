use tui::backend::CrosstermBackend;
use tui::Terminal;

use std::io::{stdout, Result, Stdout};
use std::time::Duration;

use crossterm::event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
	disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use crate::menus::{ContentsMenu, FeedsMenu, Menu, MenuState, StoriesMenu};
use crate::models::Item;

pub struct App<'a> {
	pub feeds_menu: FeedsMenu<'a>,
	pub stories_menu: StoriesMenu<'a>,
	pub contents_menu: ContentsMenu<'a>,

	pub current_menu: MenuState,
	pub current_link: String,
}

impl<'a> Default for App<'a> {
	fn default() -> Self {
		App {
			feeds_menu: FeedsMenu::default(),
			stories_menu: StoriesMenu::default(),
			contents_menu: ContentsMenu::default(),

			current_menu: MenuState::Feeds,
			current_link: String::new()
		}
	}
}

impl<'a> App<'a> {
	fn ui<M: Menu>(menu: &mut M, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> MenuState {
		terminal.draw(|f| menu.draw(f)).unwrap();

		if poll(Duration::from_millis(500)).unwrap() {
			let event = read().unwrap();
			if let Event::Key(key_event) = event {
				menu.handle_key_event(key_event);

				return menu.transition(key_event);
			}
		}

		menu.state()
	}

	pub fn spawn(&mut self) -> Result<()> {
		enable_raw_mode()?;
		let mut stdout = stdout();
		execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
		let backend = CrosstermBackend::new(stdout);
		let mut terminal = Terminal::new(backend)?;

		loop {
			self.current_menu = match &self.current_menu {
				MenuState::Feeds => Self::ui(&mut self.feeds_menu, &mut terminal),

				MenuState::Stories(selected_channel) => {
					if let Some(channel) = selected_channel {
						let items = Item::read_from_url(channel.link.to_string());
						self.stories_menu = StoriesMenu::new(items);
						self.current_link = channel.link.to_string();
					}

					Self::ui(&mut self.stories_menu, &mut terminal)
				}

				MenuState::Contents(selected_item) => {
					if let Some(item) = selected_item {
						let item = Item::read_from_url_by_title(&self.current_link, item.title.clone().unwrap()).unwrap();
						self.contents_menu = ContentsMenu::new(item);
					}

					Self::ui(&mut self.contents_menu, &mut terminal)
				}

				MenuState::Exit => {
					break;
				}
			};
		}

		// restore terminal
		disable_raw_mode()?;
		execute!(
			terminal.backend_mut(),
			LeaveAlternateScreen,
			DisableMouseCapture
		)?;
		terminal.show_cursor()?;

		Ok(())
	}
}
