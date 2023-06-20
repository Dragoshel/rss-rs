use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use mongodb::results::DeleteResult;
use mongodb::results::InsertOneResult;
use mongodb::sync::Database;

use rss::Channel;
use rss::Item;

pub fn delete_feed(title: &str, database: &Database) -> mongodb::error::Result<DeleteResult> {
    let feeds = database.collection::<Channel>("feeds");

    feeds.delete_one(
        doc! {
            "title": title
        },
        None,
    )
}

pub fn insert_feed(
    channel: &Channel,
    database: &Database,
) -> mongodb::error::Result<InsertOneResult> {
    let feeds = database.collection::<Channel>("feeds");

    feeds.insert_one(channel, None)
}

pub fn get_story(title: &str, database: &Database) -> mongodb::error::Result<Option<Item>> {
    let feeds = database.collection::<Channel>("feeds");

    let channel = feeds.find_one(
        doc! {
            "items.title": title
        },
        FindOneOptions::builder()
            .projection(doc! {
                "items.$": 1
            })
            .build(),
    )?;

    if let Some(channel) = channel {
        let item = channel.items.first().cloned();
        return Ok(item);
    }

    Ok(None)
}

pub fn get_stories(
    feed_title: &str,
    database: &Database,
) -> mongodb::error::Result<Option<Vec<Item>>> {
    let feeds = database.collection::<Channel>("feeds");

    let channel = feeds.find_one(
        doc! {
            "title": feed_title
        },
        None,
    )?;

    if let Some(channel) = channel {
        return Ok(Some(channel.items));
    }

    Ok(None)
}

pub fn get_feed(title: &str, database: &Database) -> mongodb::error::Result<Option<Channel>> {
    let feeds = database.collection::<Channel>("feeds");

    let channel = feeds.find_one(
        doc! {
            "title": title
        },
        None,
    )?;

    Ok(channel)
}

pub fn get_feeds(database: &Database) -> mongodb::error::Result<Vec<Channel>> {
    let feeds = database.collection("feeds");
    let mut cursor = feeds.find(None, None)?;
    let mut channels: Vec<Channel> = Vec::new();

    while cursor.advance()? {
        let channel = cursor.deserialize_current()?;
        channels.push(channel);
    }

    Ok(channels)
}
