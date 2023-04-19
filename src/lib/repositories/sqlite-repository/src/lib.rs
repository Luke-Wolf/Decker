use std::path::Path;

use anyhow::Result;
use rusqlite::Connection;

use decker_lib::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqliteRepositoryError {
    #[error("game already existed in repo")]
    GameAlreadyExists,
    #[error("mod already existed in repo")]
    ModAlreadyExists,
    #[error("failed to delete game")]
    FailedToDeleteGame,
    #[error("failed to update game location")]
    FailedToUpdateGameLocation,
    #[error("failed to update mod folder location")]
    FailedToUpdateModFolderLocation,
    #[error("failed to delete mod")]
    FailedToDeleteMod,
    #[error("failed to enable mod")]
    FailedToEnableMod,
    #[error("failed to disable mod")]
    FailedToDisableMod,
}

#[derive(Debug)]
struct GameRow {
    _id: usize,
    game: String,
    game_type: String,
    location: String,
    mod_folder: String,
}

impl From<GameRow> for Game {
    fn from(value: GameRow) -> Self {
        Self {
            name: value.game,
            game_type: GameTypeOption::Name(value.game_type),
            location: value.location.into(),
            mod_folder: value.mod_folder.into(),
        }
    }
}

pub struct SqliteGameRepository {
    connection: Connection,
}

impl SqliteGameRepository {
    pub fn new(connection: Connection) -> Result<Self> {
        let mut repo = Self { connection };
        repo.create_tables()?;
        Ok(repo)
    }

    pub fn create_tables(&mut self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS
                Games(
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,                    
                    game_type TEXT NOT NULL,
                    location TEXT NOT NULL,
                    mod_folder TEXT NOT NULL
                );",
            (),
        )?;

        Ok(())
    }
}

impl GameRepository for SqliteGameRepository {
    fn add_game(&self, game: &Box<Game>) -> Result<()> {
        if self.connection.execute(
            "INSERT OR IGNORE INTO
                games (name, game_type, location, mod_folder) 
                VALUES(?1, ?2, ?3, ?4);",
            (
                &game.name,
                match &game.game_type {
                    GameTypeOption::GameType(game_type) => game_type.name(),
                    GameTypeOption::Name(name) => name.to_owned(),
                },
                game.location.to_str(),
                game.mod_folder.to_str(),
            ),
        )? == 1
        {
            Ok(())
        } else {
            Err(SqliteRepositoryError::GameAlreadyExists.into())
        }
    }

    fn get_game(&self, game_name: &str) -> Result<Option<Box<Game>>> {
        let mut stmt = self
            .connection
            .prepare_cached("SELECT * FROM Games WHERE name = ?1;")?;
        let game_iter = stmt.query_map([game_name], |row| {
            Ok(GameRow {
                _id: row.get(0)?,
                game: row.get(1)?,
                game_type: row.get(2)?,
                location: row.get(3)?,
                mod_folder: row.get(4)?,
            })
        })?;
        let result = game_iter.fold(Ok(None), |_, row| {
            if let Ok(game) = row {
                Ok(Some(Box::new(game.into())))
            } else {
                Ok(None)
            }
        });
        result
    }

    fn remove_game(&self, game: &Box<Game>) -> Result<()> {
        if self.connection.execute(
            "DELETE FROM games
                WHERE name='?1';",
            &[&game.name],
        )? == 1
        {
            Ok(())
        } else {
            Err(SqliteRepositoryError::FailedToDeleteGame.into())
        }
    }

    fn games(&self) -> Result<Vec<Box<Game>>> {
        let mut stmt = self.connection.prepare("SELECT * from Games")?;
        let game_iter = stmt.query_map([], |row| {
            Ok(GameRow {
                _id: row.get(0)?,
                game: row.get(1)?,
                game_type: row.get(2)?,
                location: row.get(3)?,
                mod_folder: row.get(4)?,
            })
        })?;
        let game_iter = game_iter.map(|row| Box::new(row.unwrap().into())).collect();
        Ok(game_iter)
    }

    fn update_game_location(&self, game: &Box<Game>, path: &Path) -> Result<()> {
        if self.connection.execute(
            "UPDATE games
                  SET location = '?1'
                  WHERE name='?2'",
            (path.to_str(), &game.name),
        )? == 1
        {
            Ok(())
        } else {
            Err(SqliteRepositoryError::FailedToUpdateGameLocation.into())
        }
    }

    fn update_mod_folder_location(&self, game: &Box<Game>, path: &Path) -> Result<()> {
        if self.connection.execute(
            "UPDATE games
                  SET mod_folder = '?1'
                  WHERE name='?2'",
            (path.to_str(), &game.name),
        )? == 1
        {
            Ok(())
        } else {
            Err(SqliteRepositoryError::FailedToUpdateModFolderLocation.into())
        }
    }
}

pub struct SqliteModRepository {
    connection: Connection,
}

struct ModRepositoryRow {
    _id: usize,
    _game_name: String,
    name: String,
    description: String,
    url: String,
    location: String,
    enabled: bool,
}

impl From<ModRepositoryRow> for Mod {
    fn from(value: ModRepositoryRow) -> Self {
        Self {
            name: value.name,
            description: value.description,
            url: value.url,
            location: value.location.into(),
            enabled: value.enabled,
            dependencies: vec![],
        }
    }
}

impl SqliteModRepository {
    pub fn new(connection: Connection) -> Result<Self> {
        let mut repo = Self { connection };
        repo.create_tables()?;
        Ok(repo)
    }

    pub fn create_tables(&mut self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS
                Mods(
                    id INTEGER PRIMARY KEY,
                    game_name TEXT NOT NULL,
                    name TEXT NOT NULL,
                    description TEXT,
                    url TEXT,                    
                    location TEXT NOT NULL,
                    enabled INTEGER NOT NULL                    
                );",
            (),
        )?;

        Ok(())
    }
}

impl ModRepository for SqliteModRepository {
    fn add_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()> {
        if self.connection.execute(
            "INSERT OR IGNORE INTO
                mods (game_name, name, location, enabled) 
                VALUES(?1, ?2, ?3, 0);",
            (&game.name, &game_mod.name, &game_mod.location.to_str()),
        )? == 1
        {
            Ok(())
        } else {
            Err(SqliteRepositoryError::ModAlreadyExists.into())
        }
    }

    fn remove_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()> {
        if self.connection.execute(
            "DELETE FROM mods
                WHERE game_name='?1' AND name='?2';",
            &[&game.name, &game_mod.name],
        )? == 1
        {
            Ok(())
        } else {
            Err(SqliteRepositoryError::FailedToDeleteGame.into())
        }
    }

    fn enable_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()> {
        if self.connection.execute(
            "UPDATE mods
                  SET enabled = 1
                  WHERE game_name='?1' AND name = '?2';",
            (&game.name, &game_mod.name),
        )? == 1
        {
            Ok(())
        } else {
            Err(SqliteRepositoryError::FailedToEnableMod.into())
        }
    }

    fn disable_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()> {
        if self.connection.execute(
            "UPDATE mods
                  SET enabled = 0
                  WHERE game_name='?1' AND name = '?2';",
            (&game.name, &game_mod.name),
        )? == 1
        {
            Ok(())
        } else {
            Err(SqliteRepositoryError::FailedToUpdateModFolderLocation.into())
        }
    }

    fn available_mods(&self, game: &Box<Game>) -> Result<Vec<Box<Mod>>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * from Mods where game_name = ?1;")?;
        let game_iter = stmt.query_map([&game.name], |row| {
            Ok(ModRepositoryRow {
                _id: row.get(0)?,
                _game_name: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                url: row.get(4)?,
                location: row.get(5)?,
                enabled: row.get(6)?,
            })
        })?;
        let mod_iter = game_iter.map(|row| Box::new(row.unwrap().into())).collect();
        Ok(mod_iter)
    }

    fn enabled_mods(&self, game: &Box<Game>) -> Result<Vec<Box<Mod>>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * from Mods where game_name = ?1 AN, PathBufD enabled = 1;")?;
        let game_iter = stmt.query_map([&game.name], |row| {
            Ok(ModRepositoryRow {
                _id: row.get(0)?,
                _game_name: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                url: row.get(4)?,
                location: row.get(5)?,
                enabled: row.get(6)?,
            })
        })?;
        let mod_iter = game_iter.map(|row| Box::new(row.unwrap().into())).collect();
        Ok(mod_iter)
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    use anyhow::Result;
    use rusqlite::Connection;

    fn create_game_repo() -> Result<SqliteGameRepository> {
        let connection = Connection::open_in_memory()?;
        SqliteGameRepository::new(connection)
    }

    #[test]
    fn create_game_tables() -> Result<()> {
        create_game_repo()?;
        Ok(())
    }

    #[test]
    fn add_game() -> Result<()> {
        let game_repo = create_game_repo()?;
        let game = Box::new(Game {
            name: "OpenMW".into(),
            location: "/games/openMW".into(),
            game_type: GameTypeOption::Name("OpenMW".into()),
            mod_folder: "/mods/openMW".into(),
        });
        let size = game_repo.games()?.len();
        game_repo.add_game(&game)?;
        assert_eq!(size + 1, game_repo.games()?.len());

        if let Some(retrieved_game) = game_repo.get_game("OpenMW")? {
            assert_eq!(retrieved_game, game);
        } else {
            panic!("could not get game from repo");
        }
        Ok(())
    }

    #[test]
    fn remove_game() -> Result<()> {
        let game_repo = create_game_repo()?;
        let game = Box::new(Game {
            name: "OpenMW".into(),
            location: "/games/openMW".into(),
            game_type: GameTypeOption::Name("OpenMW".into()),
            mod_folder: "/mods/openMW".into(),
        });
        game_repo.add_game(&game)?;

        if let Some(retrieved_game) = game_repo.get_game("OpenMW")? {
            assert_eq!(retrieved_game, game)
        } else {
            panic!("could not get game from repo")
        }
        game_repo.remove_game(&game)?;
        if !game_repo.get_game("OpenMW")?.is_none() {
            panic!("game not removed from repo")
        }

        Ok(())
    }

    #[test]
    fn get_game() -> Result<()> {
        let game_repo = create_game_repo()?;
        let game = Box::new(Game {
            name: "OpenMW".into(),
            location: "/games/openMW".into(),
            game_type: GameTypeOption::Name("OpenMW".into()),
            mod_folder: "/mods/openMW".into(),
        });
        let size = game_repo.games()?.len();
        game_repo.add_game(&game)?;
        assert_eq!(size + 1, game_repo.games()?.len());

        if let Some(retrieved_game) = game_repo.get_game("OpenMW")? {
            assert_eq!(retrieved_game, game);
        } else {
            panic!("could not get game from repo");
        }
        Ok(())
    }

    #[test]
    fn get_games() -> Result<()> {
        todo!()
    }

    #[test]
    fn update_game_location() -> Result<()> {
        todo!()
    }

    #[test]
    fn update_mod_folder_location() -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod mod_tests {
    use super::*;

    use anyhow::Result;
    use rusqlite::Connection;

    fn create_mod_repo() -> Result<SqliteModRepository> {
        let connection = Connection::open_in_memory()?;
        SqliteModRepository::new(connection)
    }

    #[test]
    fn create_mod_tables() -> Result<()> {
        create_mod_repo()?;
        Ok(())
    }
}
