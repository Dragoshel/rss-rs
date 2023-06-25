mod app;
mod cli;
mod error;
mod menus;
mod models;
mod util;

pub use app::App;
pub use cli::{Cli, Commands};

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    if cli.seed {
        cli.seed_database()
            .expect("Encountered error while seeding the database");
        println!("Succesfully Seeded");
    } else {
        cli.handle_args().unwrap();
    }
}
