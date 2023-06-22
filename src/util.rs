use std::io::{Cursor, BufReader};

use rss::Channel;

use crate::models::Feed;

pub fn fetch_http(url: &str) -> reqwest::Result<BufReader<Cursor<String>>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(BufReader::new(Cursor::new(body)))
}

pub fn fetch_feed(url: &str) -> crate::Result<Feed> {
	let reader = fetch_http(url)?;
	let channel = Channel::read_from(reader)?;
	let feed = Feed::from(channel);
	Ok(feed)
}