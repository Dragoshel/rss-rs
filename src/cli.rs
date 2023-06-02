use clap::Parser;

use crate::app::App;
use crate::mongo::run as mongo_run;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[arg(long)]
	mongo: bool,
}

pub fn run() {
	let cli = Cli::parse();
    let mut app = App::new();

	if cli.mongo {
		mongo_run();
		return;
	}


	app.init().unwrap();
    app.run().unwrap();
}
