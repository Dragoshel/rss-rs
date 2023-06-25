use mongodb::sync::Client;

use clap::{Parser, Subcommand};

use crate::{models::insert_many_feed, util::fetch_feed};

#[derive(Parser)]
#[command(author = "Dragoş Ionescu")]
#[command(version)]
#[command(about = "RSS feed reader in your terminal.")]
#[command(
    long_about = "An RSS news feed reader with a beautiful TUI. It also has a CLI for handy quick commands."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, value_name = "URL")]
    pub database: String,

    #[arg(long)]
    pub seed_database: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Commands that read a feed(s) from the internet and gives quick output
    Read {
        /// Print the contents of a feed
        #[arg(short, long)]
        feed: bool,

        /// Print the contents of a story. Default story is the first one, if no number is provided
        #[arg(short, long, value_name = "NUMBER")]
        story: Option<usize>,

        /// Print the contents of all stories from a feed
        #[arg(long)]
        story_all: bool,

        /// A valid RSS feed URL
        #[arg(short, long)]
        url: String,

        /// Verbosity level for printing contents
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },
}

impl Cli {
    pub fn seed_database(self) -> crate::error::Result<()> {
        let client = Client::with_uri_str(self.database)?;
        let db = client.database("main");
        db.drop(None)?;

        let its_foss = fetch_feed("https://itsfoss.com/rss/")?;
        let darknet_diaries = fetch_feed("https://feeds.megaphone.fm/darknetdiaries")?;
        let wired_security = fetch_feed("https://www.wired.com/feed/category/security/latest/rss")?;

        insert_many_feed(vec![&its_foss, &darknet_diaries, &wired_security], &db)?;
        Ok(())
    }

    pub fn handle_args(self) -> crate::error::Result<()> {
        if let Some(Commands::Read {
            feed,
            story,
            story_all,
            url,
            verbose,
        }) = self.command
        {
            let fetched_feed = fetch_feed(url.as_str())?;
            if feed {
                println!("{fetched_feed:?}");
            } else if story_all {
                let stories = fetched_feed.stories();
                println!("{stories:?}");
            } else {
                let story = story.unwrap_or_default();
                let story = fetched_feed.stories().get(story).unwrap();
                println!("{story:?}");
            }
            Ok(())
        } else {
            let client = Client::with_uri_str(self.database)?;
            let db = client.database("main");
            let mut app = crate::App::new(&db);

            app.load()?;
            app.run()
        }
    }
}
