use tui::backend::CrosstermBackend;
use tui::style::{Color, Style};
use tui::widgets::Block;
use tui::Terminal;

use std::io::{stdout, Stdout};
use std::time::Duration;

use crossterm::event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use mongodb::sync::Database;

use crate::{one_dark, ContentsMenu, FeedsMenu, Menu, MenuState, StoriesMenu};

pub struct App<'a> {
    pub feeds_menu: FeedsMenu<'a>,
    pub stories_menu: StoriesMenu<'a>,
    pub contents_menu: ContentsMenu,

    pub current_menu: MenuState,
}

impl<'a> App<'a> {
    pub fn new(db: &'a Database) -> crate::Result<Self> {
        let app = App {
            feeds_menu: FeedsMenu::new("Your Feeds", db)?,
            stories_menu: StoriesMenu::new("Your Stories", db),
            contents_menu: ContentsMenu::new(),

            current_menu: MenuState::Feeds,
        };

        Ok(app)
    }

    fn ui<M: Menu>(menu: &mut M, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> MenuState {
        terminal
            .draw(|f| {
                let background =
                    Block::default().style(Style::default().bg(one_dark(Color::Black)));
                f.render_widget(background, f.size());

                menu.draw(f);
            })
            .unwrap();

        if poll(Duration::from_millis(100)).unwrap() {
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
                MenuState::Feeds => Self::ui(&mut self.feeds_menu, &mut terminal),

                // TRANSITION FOR STORIES MENU
                MenuState::Stories(stories) => {
                    if let Some(stories) = stories {
                        self.stories_menu.set_stories(stories.clone());
                    }

                    Self::ui(&mut self.stories_menu, &mut terminal)
                }

                // TRANSITION FOR CONTENTS MENU
                MenuState::Contents(item) => {
                    if let Some(item) = item {
                        self.contents_menu.set_story(item.clone());
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
