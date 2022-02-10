use std::path::PathBuf;

use clap::ArgEnum;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

///List of Supported Games
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Games {
    OpenMW,
}

pub struct Decker {}

impl Decker {
    pub fn new() -> Self {
        Decker {}
    }
    pub fn add_game(game: Games, path: PathBuf) {
        todo!()
    }
    pub fn add_mod(game: Games, game_mod: String, path: PathBuf) {
        todo!()
    }
    pub fn remove_game(game: Games) {
        todo!()
    }
    pub fn remove_mod(game: Games, game_mod: String) {
        todo!()
    }
    pub fn enable_game(game: Games) {
        todo!()
    }
    pub fn enable_mod(game: Games, game_mod: String) {
        todo!()
    }
    pub fn disable_game(game: Games) {
        todo!()
    }
    pub fn disable_mod(game: Games, game_mod: String) {
        todo!()
    }
}

impl Default for Decker {
    fn default() -> Self {
        Self::new()
    }
}
