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
    let mut app = App::new();

	app.init().unwrap();
    app.run().unwrap();
}
