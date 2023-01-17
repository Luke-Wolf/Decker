use decker_lib::*;

use std::{
    fmt::{self, Display},
    path::PathBuf,
};

use clap::{Parser, Subcommand, ValueEnum};

///List of Supported Games
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Games {
    OpenMW,
}

impl Display for Games {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Games::OpenMW => {
                write!(f, "OpenMW")
            }
        }
    }
}

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

    // let path = if let Some(mut path) = dirs::home_dir() {
    //     path.push(".decker");

    //     // Ensure that the path exists
    //     fs::create_dir_all(&path)?;

    //     path.push("decker.db");
    //     path
    // } else {
    //     //dump the file in the current directory
    //     let mut path = PathBuf::new();
    //     path.push("decker.db");
    //     path
    // };
    // let connection = Connection::open(path)?;

    // let decker = Decker::new();
    // match decker {
    //     Ok(decker) => {
    //         println!("Got Decker");
    //     }
    //     Err(e) => {
    //         println!("Got Error {e:?}");
    //     }
    // }
}
