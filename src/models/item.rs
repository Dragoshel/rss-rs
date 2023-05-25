use crate::util::{get_text, skip_to, get};
use std::{
	fmt::Display,
	io::{BufReader, Read},
};

use xml::{reader::XmlEvent, EventReader};

#[derive(Default, Debug)]
pub struct Item {
	// OPTIONAL item elements
	pub title: Option<String>,
	pub link: Option<String>,
	pub description: Option<String>,

	author: Option<String>,
	category: Option<String>,
	comments: Option<String>,
	enclosure: Option<String>,
	guid: Option<String>,
	pub_date: Option<String>,
	source: Option<String>,
}

impl Display for Item {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(title) = &self.title {
			write!(f, "Title: {}\n\n", title)?;
		}
		if let Some(link) = &self.link {
			write!(f, "Link: {}\n\n", link)?;
		}
		if let Some(description) = &self.description {
			write!(f, "Description: {}\n\n", description)?;
		}
		Ok(())
	}
}

impl Item {

	pub fn read_from_url_by_title(url: &String, title: String) -> xml::reader::Result<Item, String> {		
		let reader = BufReader::new(get(url.to_string()));
		let mut reader = EventReader::new(reader);

		loop {
			if let Ok(_) = skip_to(&mut reader, "item") {
				let item = Self::read(&mut reader).unwrap();
				if item.title.clone().unwrap() == title {
					return Ok(item);
				}
			} else { break; }
		}
		Err("no items".to_string())
	}

	pub fn read_from_url(url: String) -> Vec<Item> {
		let reader = BufReader::new(get(url));
		let mut reader = EventReader::new(reader);
		Self::read_all(&mut reader)
	}

	pub fn read_all(
		reader: &mut EventReader<BufReader<Box<dyn Read>>>
	) -> Vec<Item> {
		let mut items: Vec<Item> = Vec::new();

		loop {
			if let Ok(_) = skip_to(reader, "item") {
				let item = Item::read(reader).unwrap();
				items.push(item);
			} else {
				break;
			}
		}
		items
	}

	pub fn read_index(
		reader: &mut EventReader<BufReader<Box<dyn Read>>>,
		index: i8,
	) -> xml::reader::Result<Item> {
		skip_to(reader, "item").unwrap();

		for _ in 0..index {
			skip_to(reader, "item").unwrap();
		}
		Ok(Item::read(reader).unwrap())
	}

	pub fn read_count(
		reader: &mut EventReader<BufReader<Box<dyn Read>>>,
		count: i8
	) -> xml::reader::Result<Vec<Item>> {
		let mut items: Vec<Item> = Vec::new();

		skip_to(reader, "item").unwrap();

		for _ in 0..count {
			let item = Item::read(reader).unwrap();
			items.push(item);
			skip_to(reader, "item").unwrap();
		}

		Ok(items)
	}

	pub fn read(
		reader: &mut EventReader<BufReader<Box<dyn Read>>>,
	) -> xml::reader::Result<Item> {
		let mut item = Item::default();

		loop {
			match reader.next() {
				Ok(XmlEvent::StartElement { name, .. }) => {
					let tag_name = name.local_name.as_str();

					match tag_name {
						"title" => item.title = Some(get_text(reader).unwrap()),
						"link" => item.link = Some(get_text(reader).unwrap()),
						"description" => item.description = Some(get_text(reader).unwrap()),
						_ => {}
					}
				}
				Ok(XmlEvent::EndElement { name }) => {
					if name.local_name == "item" {
						break;
					}
				}
				_ => {}
			}
		}
		Ok(item)
	}
}
