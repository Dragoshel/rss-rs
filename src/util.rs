use std::io::{BufReader, Read};

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
            Ok(XmlEvent::EndDocument) => return Ok(()),
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