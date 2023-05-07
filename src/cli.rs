use clap::{Args, Parser, Subcommand};
use std::{
    fs::File,
    io::{BufReader, Read},
};
use url::Url;
use xml::EventReader;

use crate::models::{channel::Channel, item::Item};

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
    url: Option<String>,
}

#[derive(Args)]
struct WriteCommand {}

fn is_valid_url(url: String) -> bool {
    match Url::parse(url.as_str()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Read(read_command)) => {
            let url = read_command.url.unwrap();
            let file = File::open(url).unwrap();
            let file = BufReader::new(file);
            let mut reader = EventReader::new(file);

            match read_command.items {
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

            match read_command.item {
                Some(index) => {
                    let item = Item::read_index(&mut reader, index as i8).unwrap();
                    println!("{item}");
                    return;
                }
                _ => {}
            }

            let channel: Channel;
            if read_command.verbose {
                channel = Channel::read_all(&mut reader).unwrap();
                println!("{channel}");
            } else {
                channel = Channel::read_required(&mut reader).unwrap();
                println!("{channel}");
            }
        }
        Some(Command::Write(read_command)) => {}
        None => todo!(),
    }
}
