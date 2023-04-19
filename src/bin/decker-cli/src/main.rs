use decker_lib::*;
use rusqlite::Connection;
use sqlite_repository::{SqliteGameRepository, SqliteModRepository};
use sys_mount_filesystem_operations::SysMountFileSystemOperations;

use std::{
    fmt::{self, Display},
    fs,
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
    let mut decker_path = dirs::home_dir().unwrap();
    decker_path.push(".decker");
    fs::create_dir_all(&decker_path).unwrap();
    decker_path.push("decker.sqlite");

    let sqlite_connection = Connection::open(&decker_path).unwrap();
    let game_repository = Box::new(SqliteGameRepository::new(sqlite_connection).unwrap());

    let sqlite_connection = Connection::open(&decker_path).unwrap();
    let mod_repository = Box::new(SqliteModRepository::new(sqlite_connection).unwrap());

    let filesystem_operation = Box::new(SysMountFileSystemOperations::new());

    let decker = Decker::new(game_repository, mod_repository, filesystem_operation).unwrap();

    match cli.command {
        Commands::Add { command } => match command {
            AddCommands::Game { name, path } => {
                let game = Box::new(Game {
                    name: "OpenMW".into(),
                    location: path,
                    game_type: GameTypeOption::Name("OpenMW".into()),
                    mod_folder: "~/Decker/Mods/OpenMW".into(),
                });
                decker.add_game(&game);
            }
            AddCommands::Mod {
                game,
                game_mod,
                path,
            } => todo!(),
        },
        Commands::Remove { command } => match command {
            SubCommands::Game { name } => {                
                let game = Box::new(Game {
                    name: "OpenMW".into(),
                    location: "".into(),
                    game_type: GameTypeOption::Name("OpenMW".into()),
                    mod_folder: "~/Decker/Mods/OpenMW".into(),
                });
                decker.remove_game(&game);
            }
            SubCommands::Mod { game, game_mod } => todo!(),
        },
        Commands::Disable { command } => match command {
            SubCommands::Game { name } => {
                todo!()
            }
            SubCommands::Mod { game, game_mod } => todo!(),
        },
        Commands::Enable { command } => match command {
            SubCommands::Game { name } => todo!(),
            SubCommands::Mod { game, game_mod } => todo!(),
        },
    }
}
