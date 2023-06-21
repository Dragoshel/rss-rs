mod app;
mod error;
mod menus;
mod mongo;
mod util;
mod cli;

use error::{Error, Result};
use menus::*;
use util::*;
use mongo::*;
use app::App;

fn main() {
	self::cli::handle().unwrap();
}