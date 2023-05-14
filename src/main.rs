mod commands;
use clap::{ValueEnum, Parser};


#[derive(ValueEnum, Clone, Debug)]
enum Command {
    Switch,
    Create
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    command: Command,
}

fn main() {
    println!("Init");
    let args = Args::parse();

    
    match args.command {
        Command::Create => commands::branch::create(),
        Command::Switch => commands::branch::switch(),
    } 

    println!("Finished");

}
