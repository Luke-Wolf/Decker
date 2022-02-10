use decker_lib::*;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author,version,about,long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[clap(subcommand)]
        command: AddCommands,
    },
    Remove {
        #[clap(subcommand)]
        command: SubCommands,
    },
    Enable {
        #[clap(subcommand)]
        command: SubCommands,
    },
    Disable {
        #[clap(subcommand)]
        command: SubCommands,
    },
}

#[derive(Subcommand)]
enum AddCommands {
    Game {
        /// The Name of the Game
        #[clap(arg_enum)]
        name: Games,
        /// The Path to the Game
        path: PathBuf,
    },
    Mod {
        #[clap(arg_enum)]
        game: Games,
        game_mod: String,
        path: PathBuf,
    },
}

#[derive(Subcommand)]
enum SubCommands {
    Game {
        /// The Name of the Game
        #[clap(arg_enum)]
        name: Games,
    },
    Mod {
        /// The Name of the Game
        #[clap(arg_enum)]
        game: Games,
        /// The Name of the Mod
        game_mod: String,
    },
}

fn main() {
    let cli = Cli::parse();
}
