mod app;
mod error;
mod menus;
mod util;
mod cli;
mod models;

pub use app::App;
pub use cli::{Cli, Commands};

use clap::Parser;

fn main() {
	let cli = Cli::parse();

	if cli.seed_database {
		cli.seed_database().expect("Encountered error while seeding the database");
		println!("Succesfully Seeded");
	} else {
	    cli.handle_args().unwrap();
	}
}