use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    resource: Resource,
}

#[derive(Debug, Subcommand)]
enum Resource {
    Issue {
        #[clap(subcommand)]
        command: Command,
    },
}

#[derive(Debug, Subcommand)]
enum Command {
    List,
    Add,
}

fn main() {
    let args = Args::parse();
    match args.resource {
        Resource::Issue { command } => match command {
            Command::Add => println!("Add!"),
            Command::List => println!("List!"),
        },
    }
}
