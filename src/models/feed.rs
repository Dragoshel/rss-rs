use serde::{Deserialize, Serialize};

use mongodb::bson::oid::ObjectId;

use super::Story;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Feed {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    title: String,
    link: String,
    rss_link: String,
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
            rss_link: String::new(),
            description: channel.description,
            stories,
        }
    }
}

impl Feed {
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn link(&self) -> &str {
        self.link.as_str()
    }

    pub fn set_link(&mut self, link: impl Into<String>) {
        self.link = link.into();
    }

    pub fn rss_link(&self) -> &str {
        self.rss_link.as_str()
    }

    pub fn set_rss_link(&mut self, rss_link: impl Into<String>) {
        self.rss_link = rss_link.into();
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn stories(&self) -> &[Story] {
        &self.stories
    }

    pub fn set_stories(&mut self, stories: impl Into<Vec<Story>>) {
        self.stories = stories.into();
    }

	pub fn stories_len(&self) -> usize {
		self.stories.len()
	}
}
