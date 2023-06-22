mod app;
mod error;
mod menus;
mod util;
mod cli;
mod models;

use error::{Error, Result};
use menus::*;
use mongodb::sync::Client;
use util::*;
use app::App;

fn main() {
	self::cli::handle().unwrap();
}