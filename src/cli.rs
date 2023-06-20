use clap::Parser;
use mongodb::sync::Client;

use crate::app::App;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[arg(long)]
	mongo: bool,
}

pub fn run() {
	let cli = Cli::parse();

	let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
	let database = client.database("Rss-Rs");
    let mut app = App::new(&database).unwrap();
    app.run().unwrap();
}
