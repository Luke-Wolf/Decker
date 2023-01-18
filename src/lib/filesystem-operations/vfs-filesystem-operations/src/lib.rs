use anyhow::Result;
use decker_lib::FileSystemOperation;
use vfs::{FileSystem, PhysicalFS, VfsPath};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

struct VFSFileOperations {}

impl FileSystemOperation for VFSFileOperations {
    fn create_mod_folder(&self, path: &std::path::Path) -> Result<()> {
        let path = VfsPath::new(PhysicalFS::new(path));
        path.create_dir_all()?;
        Ok(())
    }

    fn delete_mod_folder(&self, path: &std::path::Path) -> Result<()> {
        let path = VfsPath::new(PhysicalFS::new(path));
        path.remove_dir_all()?;
        Ok(())
    }

    fn install_mod_to_mod_folder(
        &self,
        mod_path: &std::path::Path,
        mod_folder_path: &std::path::Path,
    ) -> Result<()> {
        let mod_path = VfsPath::new(PhysicalFS::new(mod_path));
        let mod_folder_path = VfsPath::new(PhysicalFS::new(mod_folder_path));
        if mod_path.exists()? {
            if !mod_folder_path.exists()? {
                mod_folder_path.create_dir_all()?;
            }
            mod_path.copy_dir(&mod_folder_path)?;
        }
        Ok(())
    }

    fn remove_mod_from_mod_folder(
        &self,
        game_mod: &decker_lib::Mod,
        mod_folder_path: &std::path::Path,
    ) -> Result<()> {
        let mod_folder_path = VfsPath::new(PhysicalFS::new(mod_folder_path));
        let mod_path = mod_folder_path.join(&game_mod.name)?;
        mod_path.remove_dir_all()?;
        Ok(())
    }

    fn enable_mods(
        &self,
        mod_paths: Vec<&std::path::Path>,
        target_path: &std::path::Path,
    ) -> Result<()> {
        todo!()
    }

    fn disable_mods(
        &self,
        mod_paths: Vec<&std::path::Path>,
        target_path: &std::path::Path,
    ) -> Result<()> {
        todo!()
    }
}
