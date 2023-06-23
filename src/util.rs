use std::io::{Cursor, BufReader};

use rss::Channel;

use crate::models::Feed;

pub fn fetch_http(url: impl Into<String>) -> reqwest::Result<BufReader<Cursor<String>>> {
    let body = reqwest::blocking::get(url.into())?.text()?;
    Ok(BufReader::new(Cursor::new(body)))
}

pub fn fetch_feed(url: impl Into<String>) -> crate::error::Result<Feed> {
	let channel = fetch_http(url)?;
	let channel = Channel::read_from(channel)?;
	let feed = Feed::from(channel);
	Ok(feed)
}