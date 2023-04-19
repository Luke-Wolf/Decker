use decker_lib::FileSystemOperation;
use std::fs;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub struct SysMountFileSystemOperations {}

impl SysMountFileSystemOperations {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystemOperation for SysMountFileSystemOperations {
    fn create_mod_folder(&self, path: &std::path::Path) -> anyhow::Result<()> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    fn delete_mod_folder(&self, path: &std::path::Path) -> anyhow::Result<()> {
        fs::remove_dir_all(path)?;
        Ok(())
    }

    fn install_mod_to_mod_folder(
        &self,
        mod_path: &std::path::Path,
        mod_folder_path: &std::path::Path,
    ) -> anyhow::Result<()> {
        fs::copy(mod_path, mod_folder_path)?;
        Ok(())
    }

    fn remove_mod_from_mod_folder(
        &self,
        game_mod: &decker_lib::Mod,
        mod_folder_path: &std::path::Path,
    ) -> anyhow::Result<()> {
        fs::remove_dir_all(mod_folder_path.join(&game_mod.name))?;
        Ok(())
    }

    fn enable_mods(
        &self,
        mod_paths: Vec<&std::path::Path>,
        target_path: &std::path::Path,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn disable_mods(
        &self,
        mod_paths: Vec<&std::path::Path>,
        target_path: &std::path::Path,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
