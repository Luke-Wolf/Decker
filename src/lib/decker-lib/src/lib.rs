use std::{
    fmt::{self, Display},
    fs,
    path::PathBuf,
};

use anyhow::Result;
use clap::ValueEnum;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use crate::Decker;
    use anyhow::Result;
    use rusqlite::Connection;

    #[test]
    fn create_tables() -> Result<()> {
        let connection = Connection::open_in_memory()?;
        Decker::create_tables(&connection)?;
        Ok(())
    }
}

///List of Supported Games
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, ValueEnum)]
pub enum Games {
    OpenMW,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameRow {
    id: usize,
    game: Games,
    location: PathBuf,
}

pub trait Game: fmt::Debug {}

pub trait Mod {}

impl Display for Games {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Games::OpenMW => {
                write!(f, "OpenMW")
            }
        }
    }
}

pub struct Decker {
    connection: Connection,
}

impl Decker {
    fn create_tables(connection: &Connection) -> Result<()> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS
                Games(
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    location TEXT NOT NULL
                );",
            [],
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS
                Mods(
                    id INTEGER PRIMARY KEY,
                    game_id INTEGER,
                    name TEXT NOT NULL,
                    location TEXT NOT NULL,
                    enabled INTEGER NOT NULL,
                    FOREIGN KEY(game_id) REFERENCES Games(id)
                );",
            [],
        )?;
        Ok(())
    }
    pub fn new() -> Result<Self> {
        let path = if let Some(mut path) = dirs::home_dir() {
            path.push(".decker");

            // Ensure that the path exists
            fs::create_dir_all(&path)?;

            path.push("decker.db");
            path
        } else {
            //dump the file in the current directory
            let mut path = PathBuf::new();
            path.push("decker.db");
            path
        };
        let connection = Connection::open(path)?;
        Decker::create_tables(&connection)?;
        Ok(Decker { connection })
    }
    pub fn add_game(self, game: Games, path: PathBuf) -> Result<()> {
        todo!()
        // self.connection.execute(
        //     "INSERT INTO Games(name, location) values(?, ?)",
        //     (&game.to_string(), &path.to_string_lossy()),
        // )?;
        // Ok(())
    }
    pub fn add_mod(self, game: Games, game_mod: String, path: PathBuf) {
        todo!()
    }
    pub fn remove_game(self, game: Games) {
        todo!()
    }
    pub fn remove_mod(self, game: Games, game_mod: String) {
        todo!()
    }
    pub fn enable_game(self, game: Games) {
        todo!()
    }
    pub fn enable_mod(self, game: Games, game_mod: String) {
        todo!()
    }
    pub fn disable_game(self, game: Games) {
        todo!()
    }
    pub fn disable_mod(self, game: Games, game_mod: String) {
        todo!()
    }
}
