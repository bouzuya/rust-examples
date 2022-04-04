// <https://docs.rs/clap/3.1.6/clap/trait.Subcommand.html#example>
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    Add,
    Remove,
}

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Add => println!("Add!"),
        Action::Remove => println!("Remove!!"),
    }
}
