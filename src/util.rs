use std::io::{Cursor, BufReader};

use rss::Channel;

use crate::models::Feed;

pub fn fetch_http(url: &str) -> reqwest::Result<BufReader<Cursor<String>>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(BufReader::new(Cursor::new(body)))
}

pub fn fetch_feed(url: &str) -> crate::error::Result<Feed> {
	let channel = fetch_http(url)?;
	let channel = Channel::read_from(channel)?;
	let mut feed = Feed::from(channel);
	feed.set_rss_link(url);
	Ok(feed)
}