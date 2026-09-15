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

    pub fn create() -> Result<Self, ()> {
        let mut dir: PathBuf = env::current_dir().unwrap();

        loop {
            if dir.join("root.brain").is_file() {
                return Ok(Self::new(dir))
            }
            
            match dir.parent() {
                Some(parent) => {
                    dir = parent.to_owned();
                    continue;
                },
                None => {
                    return Err(())
                }
            }
        }
    }

    pub fn project_root(&self) -> String {
        self.project_root.to_string_lossy().into_owned()
    }
}

