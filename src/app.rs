use tui::backend::CrosstermBackend;
use tui::Terminal;

use crossterm::event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use std::io::{stdout, Stdout};
use std::time::Duration;

use mongodb::sync::Database;

use crate::menus::{ContentsMenu, FeedsMenu, Menu, MenuState, StoriesMenu};

pub struct App<'a> {
    pub feeds_menu: FeedsMenu<'a>,
    pub stories_menu: StoriesMenu<'a>,
    pub contents_menu: ContentsMenu,

    pub current_menu: MenuState,
}

impl<'a> App<'a> {
    pub fn new(db: &'a Database) -> Self {
        App {
            feeds_menu: FeedsMenu::new(db),
            stories_menu: StoriesMenu::new(db),
            contents_menu: ContentsMenu::new(),

            current_menu: MenuState::Feeds,
        }
    }

    pub fn load(&mut self) -> crate::error::Result<()> {
        self.feeds_menu.reload()
    }

    fn ui<M: Menu>(
        menu: &mut M,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> MenuState {
		// EXTRA RUNNING IN THE BACKGROUND ex. POPUP
		menu.observer();

		// RENDERING MENU
        terminal.draw(|f| menu.draw(f)).unwrap();

		// READING KEYBOARD at 100 millis
        if let Ok(_) = poll(Duration::from_millis(100)) {
            if let Ok(Event::Key(key_event)) = read() {
                return menu.transition(key_event);
            }
        }

        menu.state()
    }

    pub fn run(&mut self) -> crate::error::Result<()> {
        enable_raw_mode().unwrap();
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        loop {
            self.current_menu = match &self.current_menu {
                // TRANSITION FOR FEEDS MENU
                MenuState::Feeds => Self::ui(&mut self.feeds_menu, &mut terminal),

                // TRANSITION FOR STORIES MENU
                MenuState::Stories(feed) => {
                    if let Some(feed) = feed.to_owned() {
                        self.stories_menu.set_feed(feed);
                        self.feeds_menu.reload()?;
                    }
                    Self::ui(&mut self.stories_menu, &mut terminal)
                }

                // TRANSITION FOR CONTENTS MENU
                MenuState::Contents(story) => {
                    if let Some(story) = story.to_owned() {
                        self.contents_menu.set_story(story);
                        self.stories_menu.reload()?;
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
