use tui::backend::CrosstermBackend;
use tui::Terminal;

use std::io::{stdout, Stdout};
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
            current_link: String::new(),
        }
    }
}

impl<'a> App<'a> {
    fn ui<M: Menu>(menu: &mut M, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> MenuState {
        terminal.draw(|f| menu.draw(f)).unwrap();

        if poll(Duration::from_millis(500)).unwrap() {
            let event = read();
            if let Ok(Event::Key(key_event)) = event {
                menu.handle_key_event(key_event);

                return menu.transition(key_event);
            }
        }
        menu.state()
    }

    pub fn spawn(&mut self) -> crate::Result<()> {
        enable_raw_mode().unwrap();
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        loop {
            self.current_menu = match &self.current_menu {
                MenuState::Feeds => Self::ui(&mut self.feeds_menu, &mut terminal),

                MenuState::Stories(selected_channel) => {
                    if let Some(channel) = selected_channel {
						let title = channel.title.to_string();
						let link = channel.link.to_string();
                        let items = Item::fetch_all(link.as_str())?;

                        self.stories_menu = StoriesMenu::new(items);
                        self.current_link = link;
                    }

                    Self::ui(&mut self.stories_menu, &mut terminal)
                }

                MenuState::Contents(selected_item) => {
                    if let Some(item) = selected_item {
// [TODO]						
// What happens if you select an
// Item which doesn't have a title?
						let title = if let Some(title) = item.title.clone() {
							title
						} else {
							String::new()
						};

// [TODO]						
// Check if item was found
                        let item = Item::fetch_single_by_title(self.current_link.as_str(), title.as_str())?;
                        self.contents_menu = ContentsMenu::new(item.unwrap());
                    }

                    Self::ui(&mut self.contents_menu, &mut terminal)
                }

                MenuState::Exit => {
                    break;
                }
            };
        }

        // restore terminal
        disable_raw_mode().unwrap();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ).unwrap();
        terminal.show_cursor().unwrap();

        Ok(())
    }
}
