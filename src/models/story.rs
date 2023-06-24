use std::io::{BufReader, Cursor};

use serde::{Deserialize, Serialize};

use mongodb::bson::oid::ObjectId;

use html2text::render::text_renderer::PlainDecorator;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Story {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    #[serde(rename = "pubDate")]
    pub_date: Option<String>,
    author: Option<String>,
    creator: Option<String>,
    content: Option<String>,
    pub read: bool,
    pub scroll: usize,
}

impl From<rss::Item> for Story {
    fn from(item: rss::Item) -> Self {
        let creator = if let Some(dc_ext) = item.dublin_core_ext() {
            dc_ext.creators.first().cloned()
        } else {
            None
        };

		let html = BufReader::new(Cursor::new(item.content().unwrap_or_default()));
		let html = html2text::parse(html);
		let html = html.render(200, PlainDecorator::new()).into_string();

        let story = Self {
            id: ObjectId::new(),
            title: item.title,
            link: item.link,
            description: item.description,
            pub_date: item.pub_date,
            author: item.author,
            creator,
            content: Some(html),
            read: false,
            scroll: 0,
        };
        story
    }
}

impl Story {
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn set_title(&mut self, title: impl Into<Option<String>>) {
        self.title = title.into();
    }

    pub fn link(&self) -> Option<&str> {
        self.link.as_deref()
    }

    pub fn set_link(&mut self, link: impl Into<Option<String>>) {
        self.link = link.into();
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, description: impl Into<Option<String>>) {
        self.description = description.into();
    }

    pub fn pub_date(&self) -> Option<&str> {
        self.pub_date.as_deref()
    }

    pub fn set_pub_date(&mut self, pub_date: impl Into<Option<String>>) {
        self.pub_date = pub_date.into();
    }

    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    pub fn set_author(&mut self, author: impl Into<Option<String>>) {
        self.author = author.into();
    }

    pub fn creator(&self) -> Option<&str> {
        self.creator.as_deref()
    }

    pub fn set_creator(&mut self, creator: impl Into<Option<String>>) {
        self.creator = creator.into();
    }

    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    pub fn set_content(&mut self, content: impl Into<Option<String>>) {
        self.content = content.into();
    }
}
