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

pub mod decker_helpers {
    use decker_lib::*;

    pub trait ModAndGameRepos: ModRepository + GameRepository {}

    pub fn get_games(
        repository: Box<dyn ModAndGameRepos>,
        filesystem_ops: Box<dyn FileSystemOperation>,
    ) -> Vec<Box<Game>> {
        todo!();
    }
}
