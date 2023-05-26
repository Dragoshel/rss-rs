use std::fs::File;
use std::io::{BufReader, Cursor};

use xml::{reader::XmlEvent, EventReader};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed fetching data")]
    Reqwest(#[from] reqwest::Error),
    #[error("failed while trying to parse xml")]
    XmlReader(#[from] xml::reader::Error),
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

// [TODO]
// Use the library skip instead
pub fn skip_to(
    reader: &mut EventReader<BufReader<Cursor<String>>>,
    tag_name: &str,
) -> xml::reader::Result<Option<()>> {
    loop {
        match reader.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == tag_name {
                    return Ok(Some(()));
                }
            }
            Ok(XmlEvent::EndDocument) => return Ok(None),
            Err(error) => return Err(error),
            _ => {}
        }
    }
}

pub fn skip_current(
    reader: &mut EventReader<BufReader<Cursor<String>>>,
    tag_name: &str,
) -> xml::reader::Result<()> {
    loop {
        match reader.next() {
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == tag_name {
                    return Ok(());
                }
            }
            Err(error) => return Err(error),
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
