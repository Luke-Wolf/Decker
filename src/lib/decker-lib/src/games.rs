use super::*;

struct OpenMW {}

impl GameType for OpenMW {
    fn name(&self) -> String {
        String::from("OpenMW")
    }

    fn enable_tool(&self, tool: &str) -> anyhow::Result<()> {
        todo!()
    }

    fn enable_mod(&self, game_mod: Box<Mod>) -> anyhow::Result<()> {
        todo!()
    }

    fn disable_mod(&self, game_mod: Box<Mod>) -> anyhow::Result<()> {
        todo!()
    }

    fn activate_mods(&self, mods: Vec<Box<Mod>>) -> anyhow::Result<()> {
        todo!()
    }

    fn deactivate_mods(&self) -> anyhow::Result<()> {
        todo!()
    }
}

struct Gamebryo {}

impl GameType for Gamebryo {
    fn name(&self) -> String {
        String::from("Gamebryo")
    }

    fn enable_tool(&self, tool: &str) -> anyhow::Result<()> {
        todo!()
    }

    fn enable_mod(&self, game_mod: Box<Mod>) -> anyhow::Result<()> {
        todo!()
    }

    fn disable_mod(&self, game_mod: Box<Mod>) -> anyhow::Result<()> {
        todo!()
    }

    fn activate_mods(&self, mods: Vec<Box<Mod>>) -> anyhow::Result<()> {
        todo!()
    }

    fn deactivate_mods(&self) -> anyhow::Result<()> {
        todo!()
    }
}

struct LooseFiles {}

impl GameType for LooseFiles {
    fn name(&self) -> String {
        String::from("LooseFiles")
    }

    fn enable_tool(&self, tool: &str) -> anyhow::Result<()> {
        todo!()
    }

    fn enable_mod(&self, game_mod: Box<Mod>) -> anyhow::Result<()> {
        todo!()
    }

    fn disable_mod(&self, game_mod: Box<Mod>) -> anyhow::Result<()> {
        todo!()
    }

    fn activate_mods(&self, mods: Vec<Box<Mod>>) -> anyhow::Result<()> {
        todo!()
    }

    fn deactivate_mods(&self) -> anyhow::Result<()> {
        todo!()
    }
}
