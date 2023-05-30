use std::io::BufReader;
use std::{fmt::Display, io::Cursor};

use xml::reader::XmlEvent;
use xml::EventReader;

use serde::{Deserialize, Serialize};

use crate::util::{fetch_http, read_text, skip_to};

#[allow(unused)]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Item {
    // OPTIONAL item elements
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,

    author: Option<String>,
    category: Option<String>,
    comments: Option<String>,
    enclosure: Option<String>,
    guid: Option<String>,
    pub_date: Option<String>,
    source: Option<String>,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(title) = &self.title {
            write!(f, "Title: {}\n\n", title)?;
        }
        if let Some(link) = &self.link {
            write!(f, "Link: {}\n\n", link)?;
        }
        if let Some(description) = &self.description {
            write!(f, "Description: {}\n\n", description)?;
        }
        Ok(())
    }
}

impl Item {
    pub fn fetch_single_by_title(url: &str, title: &str) -> crate::Result<Option<Item>> {
        let body = fetch_http(url)?;
        let reader = BufReader::new(body);
        let mut reader = EventReader::new(reader);

        loop {
            if let Some(()) = skip_to(&mut reader, "item")? {
                let item = Self::read_single(&mut reader)?;
                if let Some(item_title) = item.title.clone() {
                    if item_title == title {
                        return Ok(Some(item));
                    }
                }
            } else {
                return Ok(None);
            }
        }
    }

    pub fn fetch_all(url: &str) -> crate::Result<Vec<Item>> {
        let body = fetch_http(url)?;
        let reader = BufReader::new(body);
        let mut reader = EventReader::new(reader);

        Self::read_all(&mut reader)
    }

    pub fn read_all(
        reader: &mut EventReader<BufReader<Cursor<String>>>,
    ) -> crate::Result<Vec<Item>> {
        let mut items: Vec<Item> = Vec::new();

        loop {
            match skip_to(reader, "item") {
                Ok(Some(())) => {
                    let item = Item::read_single(reader)?;
                    items.push(item);
                }
                Ok(None) => break,
                Err(error) => return Err(crate::Error::from(error)),
            }
        }
        Ok(items)
    }

    pub fn read_single(reader: &mut EventReader<BufReader<Cursor<String>>>) -> crate::Result<Item> {
        let mut item = Item::default();

        loop {
            match reader.next() {
                Ok(XmlEvent::StartElement { name, .. }) => match name.local_name.as_str() {
                    "title" => item.title = read_text(reader)?,
                    "link" => item.link = read_text(reader)?,
                    "description" => item.description = read_text(reader)?,
                    _ => {}
                },
                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name == "item" {
                        break;
                    }
                }
                _ => {}
            }
        }
        Ok(item)
    }
    // pub fn read_index(
    //     reader: &mut EventReader<BufReader<Cursor<String>>>,
    //     index: i8,
    // ) -> xml::reader::Result<Item> {
    //     skip_to(reader, "item").unwrap();

    //     for _ in 0..index {
    //         skip_to(reader, "item").unwrap();
    //     }
    //     Ok(Item::read(reader).unwrap())
    // }

    // pub fn read_count(
    //     reader: &mut EventReader<BufReader<Cursor<String>>>,
    //     count: i8,
    // ) -> xml::reader::Result<Vec<Item>> {
    //     let mut items: Vec<Item> = Vec::new();

    //     skip_to(reader, "item").unwrap();

    //     for _ in 0..count {
    //         let item = Item::read(reader).unwrap();
    //         items.push(item);
    //         skip_to(reader, "item").unwrap();
    //     }

    //     Ok(items)
    // }
}
