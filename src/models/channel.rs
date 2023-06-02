use std::io::Cursor;
use std::{fmt::Display, io::BufReader};

use xml::{reader::XmlEvent, EventReader};

use serde::{Deserialize, Serialize};

use crate::util::{read_text, fetch_http};
use crate::models::item::Item;


#[allow(unused)]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Channel {
    // REQUIRED channel elements
    pub title: String,
    pub link: String,
    pub description: String,

    // OPTIONAL channel elements
    pub rss_link: Option<String>,
    language: Option<String>,
    copyright: Option<String>,
    managing_editor: Option<String>,
    web_master: Option<String>,
    pub_date: Option<String>,
    last_build_date: Option<String>,
    category: Option<String>,
    generator: Option<String>,
    docs: Option<String>,
    cloud: Option<String>,
    ttl: Option<String>,

    // Composite channel elements
    pub items: Option<Vec<Item>>,
    image: Option<String>,
    rating: Option<String>,
    text_input: Option<String>,
    skip_hours: Option<String>,
    skip_days: Option<String>,
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}\nLink: {}\nDescription: {}\n",
            self.title, self.link, self.description
        )?;
        if let Some(language) = &self.language {
            write!(f, "Language: {language}\n")?;
        }
        if let Some(copyright) = &self.copyright {
            write!(f, "Copyright: {copyright}\n")?;
        }
        if let Some(managing_editor) = &self.managing_editor {
            write!(f, "ManaginEditor: {managing_editor}\n")?;
        }
        if let Some(web_master) = &self.web_master {
            write!(f, "WebMaster: {web_master}\n")?;
        }
        if let Some(pub_date) = &self.pub_date {
            write!(f, "PubDate: {pub_date}\n")?;
        }
        if let Some(last_build_date) = &self.last_build_date {
            write!(f, "LastBuildDate: {last_build_date}\n")?;
        }
        if let Some(category) = &self.category {
            write!(f, "Category: {category}\n")?;
        }
        if let Some(generator) = &self.generator {
            write!(f, "Generator: {generator}\n")?;
        }
        if let Some(docs) = &self.docs {
            write!(f, "Docs: {docs}\n")?;
        }
        if let Some(cloud) = &self.cloud {
            write!(f, "Cloud: {cloud}\n")?;
        }
        if let Some(ttl) = &self.ttl {
            write!(f, "Ttl: {ttl}\n")?;
        }
        Ok(())
    }
}

impl Channel {
	pub fn fetch_required(url: &str) -> crate::Result<Channel> {
        let body = fetch_http(url)?;
        let reader = BufReader::new(body);
        let mut reader = EventReader::new(reader);
		
		Self::read_required(&mut reader)
	}

    pub fn read_required(
        reader: &mut EventReader<BufReader<Cursor<String>>>,
    ) -> crate::Result<Channel> {
        let mut channel = Channel::default();
        loop {
            match reader.next() {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    // Currently ignoring any namespaces because
                    // I don't have any schema validation
                    if name.prefix.is_some() {
                        break;
                    }

                    match name.local_name.as_str() {
                        // Required
                        "title" => {
                            channel.title = match read_text(reader)? {
                                Some(title) => title,
                                None => String::new(),
                            }
                        }
                        "link" => {
                            channel.link = match read_text(reader)? {
                                Some(link) => link,
                                None => String::new(),
                            }
                        }
                        "description" => {
                            channel.description = match read_text(reader)? {
                                Some(description) => description,
                                None => String::new(),
                            }
                        }

                        // Currently skipping over item
                        "item" => reader.skip()?,
                        // Currently skipping over image
                        "image" => reader.skip()?,
                        // Currently skipping over rating
                        "rating" => reader.skip()?,
                        // Currently skipping over text_input
                        "text_input" => reader.skip()?,
                        // Currently skipping over skip_hours
                        "skip_hours" => reader.skip()?,
                        // Currently skipping over skip_days
                        "skip_days" => reader.skip()?,
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndDocument) => break,
                Err(error) => return Err(crate::Error::from(error)),
                _ => {}
            }
        }
        Ok(channel)
    }
}

// pub fn read_all(reader: &mut EventReader<BufReader<Cursor<String>>>) -> Result<Channel, Error> {
//     let mut channel = Channel::default();

//     loop {
//         match reader.next() {
//             Ok(XmlEvent::StartElement { name, .. }) => {
//                 // Currently ignoring any namespaces because
//                 // I don't have any schema validation
//                 if name.prefix.is_some() {
//                     break;
//                 }

//                 let current_tag_name = name.local_name.as_str();
//                 match current_tag_name {
//                     // Required
//                     "title" => channel.title = read_text(reader).unwrap(),
//                     "link" => channel.link = read_text(reader).unwrap(),
//                     "description" => channel.description = read_text(reader).unwrap(),
//                     "language" => channel.language = Some(read_text(reader).unwrap()),
//                     "copyright" => channel.copyright = Some(read_text(reader).unwrap()),
//                     "managingEditor" => {
//                         channel.managing_editor = Some(read_text(reader).unwrap())
//                     }
//                     "webMaster" => channel.web_master = Some(read_text(reader).unwrap()),
//                     "pubDate" => channel.pub_date = Some(read_text(reader).unwrap()),
//                     "lastBuildDate" => {
//                         channel.last_build_date = Some(read_text(reader).unwrap())
//                     }
//                     "category" => channel.category = Some(read_text(reader).unwrap()),
//                     "generator" => channel.generator = Some(read_text(reader).unwrap()),
//                     "docs" => channel.docs = Some(read_text(reader).unwrap()),
//                     "cloud" => channel.cloud = Some(read_text(reader).unwrap()),
//                     "ttl" => channel.ttl = Some(read_text(reader).unwrap()),

//                     // Currently skipping over item
//                     "item" => skip_current(reader, current_tag_name).unwrap(),
//                     // Currently skipping over image
//                     "image" => skip_current(reader, current_tag_name).unwrap(),
//                     // Currently skipping over rating
//                     "rating" => skip_current(reader, current_tag_name).unwrap(),
//                     // Currently skipping over text_input
//                     "text_input" => skip_current(reader, current_tag_name).unwrap(),
//                     // Currently skipping over skip_hours
//                     "skip_hours" => skip_current(reader, current_tag_name).unwrap(),
//                     // Currently skipping over skip_days
//                     "skip_days" => skip_current(reader, current_tag_name).unwrap(),
//                     _ => {}
//                 }
//             }
//             Ok(XmlEvent::EndDocument) => break,
//             _ => {}
//         }
//     }
//     Ok(channel)
// }
