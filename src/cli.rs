use clap::{Args, Parser, Subcommand};
use std::io::BufReader;
use xml::EventReader;

use crate::{models::{Item, Channel}, menus::{FeedsMenu, StoriesMenu}, util::get};
use crate::app::App;

#[derive(Parser)]
#[command(name = "Rss-Rs")]
#[command(author = "Dragos I. <ionescu.dragos23@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Cli Rss client written in Rust")]
struct Cli {
	#[command(subcommand)]
	command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
	Read(ReadCommand),
	Write(WriteCommand),
}

#[derive(Args)]
struct ReadCommand {
	/// Reads the channel and outputs information verbose
	#[arg(long, short)]
	verbose: bool,

	/// Reads <count> items from top to bottom
	#[arg(long, value_name = "count")]
	items: Option<u8>,

	/// Reads the <index> item in the channel
	#[arg(long, value_name = "order")]
	item: Option<u8>,

	#[arg(group = "input")]
	url: String,
}

#[derive(Args)]
struct WriteCommand {}

pub fn run() {
	let cli = Cli::parse();

	match cli.command {
		Some(Command::Read(command)) => {
			let url = command.url;
			let reader = BufReader::new(get(url));
			let mut reader = EventReader::new(reader);

			match command.items {
				Some(count) => {
					let items = Item::read_count(&mut reader, count as i8).unwrap();

					for item in items {
						println!("---------------");
						println!("{item}\n");
					}

					return;
				}
				_ => {}
			}

			match command.item {
				Some(index) => {
					let item = Item::read_index(&mut reader, index as i8).unwrap();
					println!("{item}");
					return;
				}
				_ => {}
			}

			let channel: Channel;
			if command.verbose {
				channel = Channel::read_all(&mut reader).unwrap();
				println!("{channel}");
			} else {
				channel = Channel::read_required(&mut reader).unwrap();
				println!("{channel}");
			}
		}
		Some(Command::Write(_command)) => todo!(),
		None => {

			let mut darknet_diaries = Channel::default();
			darknet_diaries.title = String::from("Darknet Diaries");
			darknet_diaries.link = String::from("https://feeds.megaphone.fm/darknetdiaries");
			
			let mut its_foss = Channel::default();
			its_foss.title = String::from("It's FOSS");
			its_foss.link = String::from("https://itsfoss.com/rss/");

			let mut security_latest = Channel::default();
			security_latest.title = String::from("Security Latest");
			security_latest.link = String::from("https://www.wired.com/feed/category/security/latest/rss");

			let mut hacker_news = Channel::default();
			hacker_news.title = String::from("Hacker News");
			hacker_news.link = String::from("https://news.ycombinator.com/rss");

			let subscribed_channels = vec![darknet_diaries, its_foss, security_latest, hacker_news];

			let feeds_menu = FeedsMenu::new(subscribed_channels);
			let stories_menu = StoriesMenu::default();

			let mut app = App::default();
			app.feeds_menu = feeds_menu;
			app.stories_menu = stories_menu;
			// app.subscribed_channels = subscribed_channels;

			app.spawn().unwrap();
		}
	}
}
