use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::style::{Style, Color};
use tui::widgets::Block;

use std::io::{stdout, Stdout};
use std::time::Duration;

use crossterm::event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use crate::menus::{ContentsMenu, FeedsMenu, Menu, MenuState, StoriesMenu};
use crate::models::Item;
use crate::util::one_dark;

pub struct App<'a> {
    pub feeds_menu: FeedsMenu<'a>,
    pub stories_menu: StoriesMenu<'a>,
    pub contents_menu: ContentsMenu,

    pub current_menu: MenuState,
    pub current_link: String,
}

impl<'a> App<'a> {
	pub fn new() -> Self {
        App {
            feeds_menu: FeedsMenu::new("Your Feeds"),
            stories_menu: StoriesMenu::default(),
            contents_menu: ContentsMenu::default(),

            current_menu: MenuState::Feeds,
            current_link: String::new(),
        }
	}

	pub fn init(&mut self) -> crate::Result<()> {
		self.feeds_menu.init()?;
		Ok(())
	}

    fn ui<M: Menu>(menu: &mut M, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> MenuState {
        terminal.draw(|f| {
			let background = Block::default()
				.style(Style::default()
				.bg(one_dark(Color::Black)));
			f.render_widget(background, f.size());

			menu.draw(f);
		}).unwrap();

        if poll(Duration::from_millis(500)).unwrap() {
            if let Ok(Event::Key(key_event)) = read() {
                return menu.transition(key_event);
            }
        }
        menu.state()
    }

    pub fn run(&mut self) -> crate::Result<()> {
        enable_raw_mode().unwrap();
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        loop {
            self.current_menu = match &self.current_menu {
            	// TRANSITION FOR FEEDS MENU
                MenuState::Feeds => {
					Self::ui(&mut self.feeds_menu, &mut terminal)
				}

            	// TRANSITION FOR STORIES MENU
                MenuState::Stories(selected_channel) => {
                    if let Some(channel) = selected_channel {
                        let rss_link = channel.clone().rss_link.unwrap();
                        let items = Item::fetch_all(rss_link.as_str())?;

                        self.stories_menu = StoriesMenu::new(items);
                        self.current_link = rss_link;
                    }

                    Self::ui(&mut self.stories_menu, &mut terminal)
                }

            	// TRANSITION FOR CONTENTS MENU
                MenuState::Contents(selected_item) => {
                    if let Some(item) = selected_item {
                        let title = item.title.clone().unwrap_or_default();
                        let item = Item::fetch_single_by_title(
                            self.current_link.as_str(),
                            title.as_str(),
                        )?;

                        if let Some(_) = item {
                            self.contents_menu = ContentsMenu::new(item.unwrap());
                        }
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
        )
        .unwrap();
        terminal.show_cursor().unwrap();

        Ok(())
    }
}
