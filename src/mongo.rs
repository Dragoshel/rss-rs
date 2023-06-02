use mongodb::results::DeleteResult;
use mongodb::{sync::Client, results::InsertOneResult};
use mongodb::bson::doc;

use crate::models::{Channel, Item};

pub fn run() {
	match get_all_feeds() {
		Ok(channels) => {
			channels.iter().for_each(|c| {
				println!("{c}");
			});
		}
		Err(error) => {
			println!("{error}");
		}
	}
}

pub fn delete_feed(title: &str) -> mongodb::error::Result<DeleteResult> {
	let client = Client::with_uri_str("mongodb://localhost:27017")?;
	let database = client.database("Rss-Rs");
	let feeds = database.collection::<Channel>("feeds");

	feeds.delete_one(doc! {
		"title": title
	}, None)
}

pub fn insert_feed(channel: &Channel) -> mongodb::error::Result<InsertOneResult> {
	let client = Client::with_uri_str("mongodb://localhost:27017")?;
	let database = client.database("Rss-Rs");
	let feeds = database.collection::<Channel>("feeds");

	feeds.insert_one(channel, None)
}

pub fn get_all_feeds() -> mongodb::error::Result<Vec<Channel>> {
	let client = Client::with_uri_str("mongodb://localhost:27017")?;
	let database = client.database("Rss-Rs");
	let feeds = database.collection("feeds");

	let mut cursor = feeds.find(None, None)?;
	let mut channels:Vec<Channel> = Vec::new();
	while cursor.advance()? {
		let channel = cursor.deserialize_current()?;
		channels.push(channel);
	}

	Ok(channels)
}