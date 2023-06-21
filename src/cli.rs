use mongodb::sync::Client;

use clap::{Parser, Subcommand};

use crate::util::fetch_feed;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Read {
        #[arg(short, long)]
        feed: bool,

        #[arg(short, long)]
        story: Option<usize>,

        #[arg(long)]
        story_all: bool,

        #[arg(short, long)]
        url: String,

	    #[arg(short, long, action = clap::ArgAction::Count)]
	    verbose: u8,
    },
}

pub fn handle() -> crate::Result<()> {
    let cli = Cli::parse();;

    match &cli.command {
        Some(Commands::Read { feed, story, story_all, url, verbose }) => {
			let fetched_feed = fetch_feed(url)?;	

			if *feed == true {
				println!("{fetched_feed:?}");
			} else if *story_all == true {
				let stories = fetched_feed.items();
				println!("{stories:?}");
			} else {
				let story = story.unwrap_or_default();
				let story = fetched_feed.items().get(story).unwrap();
				println!("{story:?}");
			}
	
			Ok(())
        }
        None => {
            let client = Client::with_uri_str("mongodb://localhost:27017")?;
            let database = client.database("main");
            let mut app = crate::App::new(&database)?;

            app.run()
        }
    }
}
