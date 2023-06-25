mod feed;
mod story;

pub use feed::Feed;
pub use story::Story;

use mongodb::bson::Document;
use mongodb::results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult};
use mongodb::sync::Database;

pub fn insert_one_feed(feed: &Feed, db: &Database) -> mongodb::error::Result<InsertOneResult> {
    db.collection::<Feed>("feeds").insert_one(feed, None)
}

pub fn insert_many_feed(
    feeds: Vec<&Feed>,
    db: &Database,
) -> mongodb::error::Result<InsertManyResult> {
    db.collection::<Feed>("feeds").insert_many(feeds, None)
}

pub fn find_one_feed(
    filter: Option<Document>,
    db: &Database,
) -> mongodb::error::Result<Option<Feed>> {
    let feed = db.collection::<Feed>("feeds").find_one(filter, None)?;
    Ok(feed)
}

pub fn find_many_feed(
    filter: Option<Document>,
    db: &Database,
) -> mongodb::error::Result<Vec<Feed>> {
    let mut cursor = db.collection("feeds").find(filter, None)?;
    let mut feeds: Vec<Feed> = Vec::new();

    while cursor.advance()? {
        let feed = cursor.deserialize_current()?;
        feeds.push(feed);
    }
    Ok(feeds)
}

pub fn delete_one_feed(query: Document, db: &Database) -> mongodb::error::Result<DeleteResult> {
    db.collection::<Feed>("feeds").delete_one(query, None)
}

pub fn update_one_feed(
    query: Document,
    update: Document,
    db: &Database,
) -> mongodb::error::Result<UpdateResult> {
    db.collection::<Feed>("feeds")
        .update_one(query, update, None)
}
