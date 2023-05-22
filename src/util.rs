use std::{io::{BufReader, Read, Cursor}, fs::File};

use reqwest::Url;
use xml::{reader::XmlEvent, EventReader};

pub fn get_text(reader: &mut EventReader<BufReader<Box<dyn Read>>>) -> Option<String> {
    loop {
        match reader.next() {
            Ok(XmlEvent::CData(text)) => return Some(text.to_string()),
            Ok(XmlEvent::Characters(text)) => return Some(text.to_string()),
            Ok(XmlEvent::Whitespace(_)) => continue,
            _ => break,
        }
    }
    None
}

pub fn skip_to(
    reader: &mut EventReader<BufReader<Box<dyn Read>>>,
    tag_name: &str,
) -> Result<(), String> {
    loop {
        match reader.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == tag_name {
                    return Ok(());
                }
            }
            Ok(XmlEvent::EndDocument) => return Err("reached end".to_string()),
            Err(error) => return Err(error.msg().to_string()),
            _ => {}
        }
    }
}

pub fn skip_current(
    reader: &mut EventReader<BufReader<Box<dyn Read>>>,
    current_tag_name: &str,
) -> Result<(), &'static str> {
    loop {
        match reader.next() {
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == current_tag_name {
                    return Ok(());
                }
            }
            _ => {}
        }
    }
}

pub fn get(url_or_local_path: String) -> Box<dyn Read> {
    if let Ok(_) = Url::parse(url_or_local_path.as_str()) {
        let body = reqwest::blocking::get(url_or_local_path)
            .unwrap()
            .text()
            .unwrap();
        let cursor = Cursor::new(body);

        Box::new(cursor)
    } else {
        let file = File::open(url_or_local_path).unwrap();
        Box::new(file)
    }
}
