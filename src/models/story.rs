use serde::{Deserialize, Serialize};

use mongodb::bson::oid::ObjectId;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Story {
    #[serde(rename = "_id")]
    id: ObjectId,
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    #[serde(rename = "pubDate")]
    pub_date: Option<String>,
    author: Option<String>,
    content: Option<String>,
    pub read: bool,
    pub scroll: usize,
}

impl From<rss::Item> for Story {
    fn from(item: rss::Item) -> Self {
        Self {
            id: ObjectId::new(),
            title: item.title,
            link: item.link,
            description: item.description,
            pub_date: item.pub_date,
            author: item.author,
            content: item.content,
            read: false,
            scroll: 0,
        }
    }
}

impl Story {
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<Option<String>>,
    {
        self.title = title.into();
    }

    pub fn link(&self) -> Option<&str> {
        self.link.as_deref()
    }

    pub fn set_link<V>(&mut self, link: V)
    where
        V: Into<Option<String>>,
    {
        self.link = link.into();
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description<V>(&mut self, description: V)
    where
        V: Into<Option<String>>,
    {
        self.description = description.into();
    }

    pub fn pub_date(&self) -> Option<&str> {
        self.pub_date.as_deref()
    }

    pub fn set_pub_date<V>(&mut self, pub_date: V)
    where
        V: Into<Option<String>>,
    {
        self.pub_date = pub_date.into();
    }

    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    pub fn set_author<V>(&mut self, author: V)
    where
        V: Into<Option<String>>,
    {
        self.author = author.into();
    }

    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    pub fn set_content<V>(&mut self, content: V)
    where
        V: Into<Option<String>>,
    {
        self.content = content.into();
    }
}
