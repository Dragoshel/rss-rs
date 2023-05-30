mod cli;
mod models;
mod util;
mod app;
mod menus;

pub use self::util::Error;
pub use self::util::Result;

use cli::run;

fn main() {
	run();
}
