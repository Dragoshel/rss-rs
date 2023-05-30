use clap::{Args, Parser, Subcommand};

use mongodb::sync::Client;

use crate::app::App;
use crate::menus::FeedsMenu;
use crate::models::Channel;

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
    match Cli::parse().command {
        None => {

			let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
			let database = client.database("Rss");
			let collection = database.collection::<Channel>("channels");
			let mut subscribed_channels:Vec<Channel> = Vec::new();

			let mut cursor = collection.find(None, None).unwrap();

			while cursor.advance().unwrap() {
				let channel = cursor.deserialize_current().unwrap();
				subscribed_channels.push(channel);
			}
	
            // let mut darknet_diaries = Channel::default();
            // darknet_diaries.title = String::from("Darknet Diaries");
            // darknet_diaries.link = String::from("https://feeds.megaphone.fm/darknetdiaries");

            // let mut its_foss = Channel::default();
            // its_foss.title = String::from("It's FOSS");
            // its_foss.link = String::from("https://itsfoss.com/rss/");

            // let mut security_latest = Channel::defautl();
            // security_latest.title = String::from("Security Latest");
            // security_latest.link = String::from("https://www.wired.com/feed/category/security/latest/rss");

            // let mut hacker_news = Channel::default();
            // hacker_news.title = String::from("Hacker News");
            // hacker_news.link = String::from("https://news.ycombinator.com/rss");

			// collection.insert_many(vec![darknet_diaries, its_foss, security_latest, hacker_news], None).unwrap();

            // let feeds_menu = FeedsMenu::new(subscribed_channels);

            let mut app = App::default();
            app.feeds_menu = FeedsMenu::new(subscribed_channels);
            // app.subscribed_channels = subscribed_channels;

            app.spawn().unwrap();
        }
        _ => {}
    }
}
