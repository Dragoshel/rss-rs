use clap::{Args, Parser, Subcommand};
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
};
use url::Url;
use xml::EventReader;

use crate::models::{Item, Channel};
use crate::app::spawn;

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

fn get(url_or_local_path: &str) -> Box<dyn Read> {
    if let Ok(_) = Url::parse(url_or_local_path) {
        let body = reqwest::blocking::get(url_or_local_path)
            .unwrap()
            .text()
            .unwrap();
        let cursor = Cursor::new(body);

        Box::new(cursor)
    } else {
        let file = File::open(url_or_local_path).unwrap();
        Box::new(file)
    }
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Read(command)) => {
            let url = command.url.as_str();
            let reader = BufReader::new(get(url));
            let mut reader = EventReader::new(reader);

            match command.items {
                Some(count) => {
                    let items = Item::read_count(&mut reader, count as i8).unwrap();

                    for item in items {
                        println!("---------------");
                        println!("{item}\n");
                    }

                    return;
                }
                _ => {}
            }

            match command.item {
                Some(index) => {
                    let item = Item::read_index(&mut reader, index as i8).unwrap();
                    println!("{item}");
                    return;
                }
                _ => {}
            }

            let channel: Channel;
            if command.verbose {
                channel = Channel::read_all(&mut reader).unwrap();
                println!("{channel}");
            } else {
                channel = Channel::read_required(&mut reader).unwrap();
                println!("{channel}");
            }
        }
        Some(Command::Write(_command)) => todo!(),
        None => {
            spawn().unwrap();
        }
    }
}
