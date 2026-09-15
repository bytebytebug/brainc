use std::env;
use std::path::PathBuf;

pub struct Engine {
    project_root: PathBuf,
}

impl Engine {

    fn new(path: PathBuf) -> Self {
        Self {
            project_root: path,
        }
    }

    pub fn create() -> Self {
        let dir = env::current_dir().unwrap();

        Self::new(dir)
    }

    pub fn project_root(&self) -> String {
        self.project_root.to_string_lossy().into_owned()
    }
}

