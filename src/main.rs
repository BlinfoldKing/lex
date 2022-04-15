pub mod ast;
pub mod definition;
pub mod evaluator;
pub mod grammar;
pub mod handler;
pub mod modules;
pub mod repl;
pub mod utils;

use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(short, long)]
    option: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// run source file
    Run {
        /// .lx source file
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// run interactive mode
    Repl,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Repl => repl::Repl::new().run(),
        Command::Run { ref path } => {
            let input = fs::read_to_string(path).unwrap();
            let res = evaluator::Engine::new().parse(&input).unwrap();

            println!("exited: {}", res)
        }
    }
}
