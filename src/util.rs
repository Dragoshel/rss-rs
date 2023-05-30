use std::fs::File;
use std::io::{BufReader, Cursor};

use tui::layout::{Layout, Direction, Constraint, Rect};
use xml::{reader::XmlEvent, EventReader};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ERROR: could not fetch data")]
    Reqwest(#[from] reqwest::Error),

    #[error("ERROR: could not parse xml document")]
    XmlReader(#[from] xml::reader::Error),

	#[error("ERROR: could not interact with Mongo Db")]
	MongoDb(#[from] mongodb::error::Error)
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn read_text(
    reader: &mut EventReader<BufReader<Cursor<String>>>,
) -> xml::reader::Result<Option<String>> {
    loop {
        match reader.next() {
            Ok(XmlEvent::CData(text)) => return Ok(Some(text)),
            Ok(XmlEvent::Characters(text)) => return Ok(Some(text)),
            Ok(XmlEvent::Whitespace(_)) => {}
            Err(error) => return Err(error),
            _ => return Ok(None),
        }
    }
}

pub fn skip_to(
    reader: &mut EventReader<BufReader<Cursor<String>>>,
    tag_name: &str,
) -> xml::reader::Result<Option<()>> {
    loop {
        match reader.next()? {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name == tag_name {
                    return Ok(Some(()));
                }
            }
            XmlEvent::EndDocument => return Ok(None),
            _ => {}
        }
    }
}

pub fn fetch_file(url: &str) -> xml::reader::Result<File> {
    let file = File::open(url)?;
    Ok(file)
}

pub fn fetch_http(url: &str) -> reqwest::Result<Cursor<String>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(Cursor::new(body))
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
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