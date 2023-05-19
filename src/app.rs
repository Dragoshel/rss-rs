use tui::Terminal;
use tui::backend::CrosstermBackend;

use std::io::{stdout, Result};
use std::time::Duration;

use crossterm::event::{poll, read, DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use crate::menus::{FeedsMenu, StoriesMenu, Menu, MenuState};


struct App {
	current_menu: MenuState
}

pub fn spawn() -> Result<()> {
	let mut app = App {
		current_menu: MenuState::FeedsMenu
	};
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
	
	let subscribed_channels = vec![
		("Darknet Diaries".to_string(), "https://feeds.megaphone.fm/darknetdiaries".to_string()),
		("It's FOSS".to_string(), "https://itsfoss.com/rss/".to_string()),
		("Security Latest".to_string(), "https://www.wired.com/feed/category/security/latest/rss".to_string()),
		("Hacker News".to_string(), "https://news.ycombinator.com/rss".to_string())
	];
	let items = vec![
		"133: I'm the Real Connor".to_string(),
		"132: Sam the Vendor".to_string(),
		"131: Welcome to Video".to_string(),
		"130: Jason's Pen Test".to_string()
	];

    let mut feeds_menu = FeedsMenu::new(subscribed_channels);
    let mut stories_menu = StoriesMenu::new(items);

    loop {
		match &app.current_menu {
			MenuState::FeedsMenu => {
        		terminal.draw(|f| feeds_menu.ui(f))?;

		        if poll(Duration::from_millis(500)).unwrap() {
					let event = read().unwrap();
					
					if let Some(next_menu) = feeds_menu.transition(event) {
						app.current_menu = next_menu;
					} else {
						break;
					}
				}
			},
			MenuState::StoriesMenu => {
        		terminal.draw(|f| stories_menu.ui(f))?;

		        if poll(Duration::from_millis(500)).unwrap() {
					let event = read().unwrap();
					if let Some(next_menu) = stories_menu.transition(event) {
						app.current_menu = next_menu;
					} else {
						break;
					}
		        }
			}
		}		

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
