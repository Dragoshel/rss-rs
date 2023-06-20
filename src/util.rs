use std::io::{Cursor, BufReader};

use rss::Channel;

pub fn fetch_http(url: &str) -> reqwest::Result<BufReader<Cursor<String>>> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(BufReader::new(Cursor::new(body)))
}

pub fn fetch_feed(url: &str) -> crate::Result<Channel> {
	let reader = fetch_http(url)?;
	let channel = Channel::read_from(reader)?;
	Ok(channel)
}