mod games;

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use anyhow::Result;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct Game {
    pub name: String,
    pub location: PathBuf,
    pub game_type: GameTypeOption,
    pub mod_folder: PathBuf,
}

pub trait GameType {
    fn name(&self) -> String;
    fn enable_tool(&self, tool: &str) -> Result<()>;
    fn enable_mod(&self, game_mod: Box<Mod>) -> Result<()>;
    fn disable_mod(&self, game_mod: Box<Mod>) -> Result<()>;
    fn activate_mods(&self, mods: Vec<Box<Mod>>) -> Result<()>;
    fn deactivate_mods(&self) -> Result<()>;
}

impl PartialEq for Box<dyn GameType> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Debug for Box<dyn GameType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Box").field(&self.name()).finish()
    }
}

#[derive(Debug)]
pub enum GameTypeOption {
    Name(String),
    GameType(Box<dyn GameType>),
}

impl PartialEq for GameTypeOption {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Name(l0), Self::Name(r0)) => l0 == r0,
            (Self::GameType(l0), Self::GameType(r0)) => l0 == r0,
            _ => false,
        }
    }
}

pub struct Mod {
    pub name: String,
    pub description: String,
    pub url: String,
    pub location: PathBuf,
    pub enabled: bool,
    pub dependencies: Vec<Box<Mod>>,
}

pub struct Decker {
    game_repository: Box<dyn GameRepository>,
    mod_repository: Box<dyn ModRepository>,
    filesystem_operation: Box<dyn FileSystemOperation>,
}

impl Decker {
    pub fn new(
        game_repository: Box<dyn GameRepository>,
        mod_repository: Box<dyn ModRepository>,
        filesystem_operation: Box<dyn FileSystemOperation>,
    ) -> Result<Self> {
        Ok(Self {
            game_repository,
            mod_repository,
            filesystem_operation,
        })
    }

    pub fn add_game(&self, game: &Box<Game>) -> Result<()> {
        self.game_repository.add_game(&game)?;
        self.filesystem_operation
            .create_mod_folder(&game.mod_folder)?;
        Ok(())
    }

    pub fn remove_game(&self, game: &Box<Game>) -> Result<()> {
        self.game_repository.remove_game(&game)?;
        self.filesystem_operation
            .delete_mod_folder(&game.mod_folder)?;
        Ok(())
    }

    pub fn get_game(&self, game_name: &str) -> Option<Game> {
        todo!()
    }

    pub fn available_games(&self) -> Result<Vec<Box<Game>>> {
        todo!()
    }

    pub fn update_game_location(&self, game: &Box<Game>, path: &Path) -> Result<()> {
        todo!()
    }

    pub fn update_mod_folder_location(&self, game: &Box<Game>, path: &Path) -> Result<()> {
        todo!()
    }

    pub fn add_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()> {
        self.mod_repository.add_mod(&game, &game_mod)?;
        self.filesystem_operation
            .install_mod_to_mod_folder(&game_mod.location, &game.mod_folder)?;
        Ok(())
    }
    pub fn remove_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()> {
        self.mod_repository.remove_mod(&game, &game_mod)
    }
    pub fn enable_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()> {
        todo!()
    }
    pub fn disable_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()> {
        todo!()
    }
    pub fn available_mods(&self, game: &Box<Game>) -> Result<Vec<Box<Mod>>> {
        todo!()
    }
    pub fn enabled_mods(&self, game: &Box<Game>) -> Result<Vec<Box<Mod>>> {
        todo!()
    }
}
pub trait ModRepository {
    fn add_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()>;
    fn remove_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()>;
    fn enable_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()>;
    fn disable_mod(&self, game: &Box<Game>, game_mod: &Box<Mod>) -> Result<()>;
    fn available_mods(&self, game: &Box<Game>) -> Result<Vec<Box<Mod>>>;
    fn enabled_mods(&self, game: &Box<Game>) -> Result<Vec<Box<Mod>>>;
}

pub trait GameRepository {
    fn add_game(&self, game: &Box<Game>) -> Result<()>;
    fn remove_game(&self, game: &Box<Game>) -> Result<()>;
    fn get_game(&self, game_name: &str) -> Result<Option<Box<Game>>>;
    fn update_game_location(&self, game: &Box<Game>, path: &Path) -> Result<()>;
    fn update_mod_folder_location(&self, game: &Box<Game>, path: &Path) -> Result<()>;
    fn games(&self) -> Result<Vec<Box<Game>>>;
}

pub trait FileSystemOperation {
    fn create_mod_folder(&self, path: &Path) -> Result<()>;
    fn delete_mod_folder(&self, path: &Path) -> Result<()>;
    fn install_mod_to_mod_folder(&self, mod_path: &Path, mod_folder_path: &Path) -> Result<()>;
    fn remove_mod_from_mod_folder(&self, game_mod: &Mod, mod_folder_path: &Path) -> Result<()>;

    fn enable_mods(&self, mod_paths: Vec<&Path>, target_path: &Path) -> Result<()>;
    fn disable_mods(&self, mod_paths: Vec<&Path>, target_path: &Path) -> Result<()>;
}
