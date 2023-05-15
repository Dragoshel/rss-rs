use crate::util::get_text;
use crate::{models::item::Item, util::skip_current};
use std::io::Read;
use std::{fmt::Display, fs::File, io::BufReader};
use xml::{reader::Error, reader::XmlEvent, EventReader};

#[derive(Default, Debug)]
pub struct Channel {
    // REQUIRED channel elements
    pub title: String,
    pub link: String,
    pub description: String,

    // OPTIONAL channel elements
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
    items: Option<Vec<Item>>,
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
    pub fn read_all(reader: &mut EventReader<BufReader<Box<dyn Read>>>) -> Result<Channel, Error> {
        let mut channel = Channel::default();

        loop {
            match reader.next() {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    // Currently ignoring any namespaces because
                    // I don't have any schema validation
                    if name.prefix.is_some() {
                        break;
                    }

                    let current_tag_name = name.local_name.as_str();
                    match current_tag_name {
                        // Required
                        "title" => channel.title = get_text(reader).unwrap(),
                        "link" => channel.link = get_text(reader).unwrap(),
                        "description" => channel.description = get_text(reader).unwrap(),
                        "language" => channel.language = Some(get_text(reader).unwrap()),
                        "copyright" => channel.copyright = Some(get_text(reader).unwrap()),
                        "managingEditor" => channel.managing_editor = Some(get_text(reader).unwrap()),
                        "webMaster" => channel.web_master = Some(get_text(reader).unwrap()),
                        "pubDate" => channel.pub_date = Some(get_text(reader).unwrap()),
                        "lastBuildDate" => channel.last_build_date = Some(get_text(reader).unwrap()),
                        "category" => channel.category = Some(get_text(reader).unwrap()),
                        "generator" => channel.generator = Some(get_text(reader).unwrap()),
                        "docs" => channel.docs = Some(get_text(reader).unwrap()),
                        "cloud" => channel.cloud = Some(get_text(reader).unwrap()),
                        "ttl" => channel.ttl = Some(get_text(reader).unwrap()),

                        // Currently skipping over item
                        "item" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over image
                        "image" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over rating
                        "rating" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over text_input
                        "text_input" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over skip_hours
                        "skip_hours" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over skip_days
                        "skip_days" => skip_current(reader, current_tag_name).unwrap(),
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndDocument) => break,
                _ => {}
            }
        }
        Ok(channel)
    }

    pub fn read_required(reader: &mut EventReader<BufReader<Box<dyn Read>>>) -> Result<Channel, Error> {
        let mut channel = Channel::default();

        loop {
            match reader.next() {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    // Currently ignoring any namespaces because
                    // I don't have any schema validation
                    if name.prefix.is_some() {
                        break;
                    }

                    let current_tag_name = name.local_name.as_str();
                    match current_tag_name {
                        // Required
                        "title" => channel.title = get_text(reader).unwrap(),
                        "link" => channel.link = get_text(reader).unwrap(),
                        "description" => channel.description = get_text(reader).unwrap(),

                        // Currently skipping over item
                        "item" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over image
                        "image" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over rating
                        "rating" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over text_input
                        "text_input" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over skip_hours
                        "skip_hours" => skip_current(reader, current_tag_name).unwrap(),
                        // Currently skipping over skip_days
                        "skip_days" => skip_current(reader, current_tag_name).unwrap(),
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndDocument) => break,
                _ => {}
            }
        }
        Ok(channel)
    }
}
