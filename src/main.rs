mod cli;
mod util;
mod app;
mod menus;
mod mongo;
mod error;

pub use self::error::Error;
pub use self::error::Result;

use cli::run;

fn main() {
	run();
}
