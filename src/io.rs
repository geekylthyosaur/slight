use std::path::Path;

pub struct IO;

impl IO {
    pub fn scan(path: &Path) -> Option<Vec<String>> {
        todo!("Read dirs from given path")
    }

    pub fn read_number(path: &Path) -> Result<usize, todo!()> {
        todo!("Read & parse number from file")
    }

    pub fn dir(path: &Path) -> Option<&str> {
        todo!("Return name of directory");
    }

    pub fn parent_dir(path: &Path) -> Option<&str> {
        todo!("Return name of parent directory")
    }
}
