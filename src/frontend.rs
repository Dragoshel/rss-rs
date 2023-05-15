use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::models::channel::Channel;
use crate::widgets::{channel_widget::ChannelWidget, input_widget::InputWidget};

pub fn spawn(channel: Channel) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input_buffer = String::new();
    // draw in the terminal
    loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Length(5), Constraint::Length(3)].as_ref())
                    .split(f.size());

                let channel_block = ChannelWidget::new();
                let input_block = InputWidget::new(&input_buffer);
                f.render_widget(input_block, chunks[0]);
                f.render_widget(channel_block, chunks[1]);
            })
            .unwrap();
        if poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char(key) => {
                        input_buffer.push(key);
                    }
                    _ => {}
                }
            }
        } else {
            // Timeout expired and no `Event` is available
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
