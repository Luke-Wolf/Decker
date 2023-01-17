mod games;

use std::path::{Path, PathBuf};

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

pub enum GameTypeOption {
    Name(String),
    GameType(Box<dyn GameType>),
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

    pub fn add_game(game: Box<Game>) -> Result<()> {
        todo!()
    }

    pub fn remove_game(game: Box<Game>) -> Result<()> {
        todo!()
    }

    pub fn available_games() -> Result<Vec<Box<Game>>> {
        todo!()
    }

    pub fn update_game_location(game: Box<Game>, path: &Path) -> Result<()> {
        todo!()
    }

    pub fn update_mod_folder_location(game: Box<Game>, path: &Path) -> Result<()> {
        todo!()
    }

    pub fn add_mod(game: Box<Game>, game_mod: Box<Mod>) -> Result<()> {
        todo!()
    }
    pub fn remove_mod(&self, game: Box<Game>, game_mod: String) -> Result<()> {
        todo!()
    }
    pub fn enable_mod(&self, game: Box<Game>, game_mod: String) -> Result<()> {
        todo!()
    }
    pub fn disable_mod(&self, game: Box<Game>, game_mod: String) -> Result<()> {
        todo!()
    }
    pub fn available_mods(&self, game: Box<Game>) -> Result<Vec<Box<Mod>>> {
        todo!()
    }
    pub fn enabled_mods(&self, game: Box<Game>) -> Result<Vec<Box<Mod>>> {
        todo!()
    }
}
pub trait ModRepository {
    fn add_mod(&self, game: Box<Game>, game_mod: String, path: &Path) -> Result<()>;
    fn remove_mod(&self, game: Box<Game>, game_mod: String) -> Result<()>;
    fn enable_mod(&self, game: Box<Game>, game_mod: String) -> Result<()>;
    fn disable_mod(&self, game: Box<Game>, game_mod: String) -> Result<()>;
    fn available_mods(&self, game: Box<Game>) -> Result<Vec<Box<Mod>>>;
    fn enabled_mods(&self, game: Box<Game>) -> Result<Vec<Box<Mod>>>;
}

pub trait GameRepository {
    fn add_game(&self, game: Box<Game>, path: &Path, mods_folder: &Path) -> Result<()>;
    fn remove_game(&self, game: Box<Game>) -> Result<()>;
    fn update_game_location(&self, game: Box<Game>, path: &Path) -> Result<()>;
    fn update_mod_folder_location(&self, game: Box<Game>, path: &Path) -> Result<()>;
    fn games(&self) -> Result<Vec<Box<Game>>>;
}

pub trait FileSystemOperation {
    fn enable_mods(&self, mod_paths: Vec<&Path>, target_path: &Path) -> Result<()>;
    fn disable_mods(&self, mod_paths: Vec<&Path>, target_path: &Path) -> Result<()>;
}
