use decker_lib::*;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author,version,about,long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[command(subcommand)]
        command: AddCommands,
    },
    Remove {
        #[command(subcommand)]
        command: SubCommands,
    },
    Enable {
        #[command(subcommand)]
        command: SubCommands,
    },
    Disable {
        #[command(subcommand)]
        command: SubCommands,
    },
}

#[derive(Subcommand)]
enum AddCommands {
    Game {
        /// The Name of the Game        
        #[arg(value_enum)]
        name: Games,
        /// The Path to the Game
        path: PathBuf,
    },
    Mod {
        #[arg(value_enum)]
        game: Games,
        game_mod: String,
        path: PathBuf,
    },
}

#[derive(Subcommand)]
enum SubCommands {
    Game {
        /// The Name of the Game
        #[arg(value_enum)]
        name: Games,
    },
    Mod {
        /// The Name of the Game
        #[arg(value_enum)]
        game: Games,
        /// The Name of the Mod
        game_mod: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let decker = Decker::new();
    match decker {
        Ok(decker) => {
            println!("Got Decker");
        }
        Err(e) => {
            println!("Got Error {e:?}");
        }
    }
}
