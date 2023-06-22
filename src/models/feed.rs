use serde::{Deserialize, Serialize};

use mongodb::bson::oid::ObjectId;

use super::Story;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Feed {
    #[serde(rename = "_id")]
    id: ObjectId,
    title: String,
    link: String,
    description: String,
    stories: Vec<Story>,
}

impl From<rss::Channel> for Feed {
    fn from(channel: rss::Channel) -> Self {
        let stories: Vec<Story> = channel
            .items()
            .iter()
            .map(|i| Story::from(i.to_owned()))
            .collect();

        Self {
            id: ObjectId::new(),
            title: channel.title,
            link: channel.link,
            description: channel.description,
            stories,
        }
    }
}

impl Feed {
	pub fn id(&self) -> String {
		self.id.to_string()
	}

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn set_title<V>(&mut self, title: V)
    where
        V: Into<String>,
    {
        self.title = title.into();
    }

    pub fn link(&self) -> &str {
        self.link.as_str()
    }

    pub fn set_link<V>(&mut self, link: V)
    where
        V: Into<String>,
    {
        self.link = link.into();
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn set_description<V>(&mut self, description: V)
    where
        V: Into<String>,
    {
        self.description = description.into();
    }

    pub fn stories(&self) -> &[Story] {
        &self.stories
    }

    pub fn set_stories<V>(&mut self, stories: V)
    where
        V: Into<Vec<Story>>,
    {
        self.stories = stories.into();
    }
}
