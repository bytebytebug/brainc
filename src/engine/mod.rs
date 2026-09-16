use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use crate::parser;

pub enum PartError {
    BadFormat(PathBuf),
}

pub enum EngineInitError {
    RootNotFound,
    PartAtRoot,
    LoadInvalidPart(Vec<PartError>),
}

pub struct Engine {
    project_root: PathBuf,
    project_parts: Vec<PathBuf>,
}

impl Engine {

    fn new(path: PathBuf) -> Result<Self, EngineInitError> {
        if path.join("part.brain").is_file() {
            return Err(EngineInitError::PartAtRoot);
        }

        let parts: Vec<PathBuf> = find_parts(path.clone());

        let parts_errors = verify_parts(parts.clone());
        if !parts_errors.is_empty() {
            return Err(EngineInitError::LoadInvalidPart(parts_errors));
        }

        Ok(Self {
            project_root: path,
            project_parts: parts,
        })
    }

    pub fn create() -> Result<Self, EngineInitError> {
        let mut dir: PathBuf = env::current_dir().unwrap();

        loop {
            if dir.join("root.brain").is_file() {
                return Self::new(dir)
            }
            
            match dir.parent() {
                Some(parent) => {
                    dir = parent.to_owned();
                    continue;
                },
                None => {
                    return Err(EngineInitError::RootNotFound)
                }
            }
        }
    }

    pub fn project_parts(&self) -> Vec<String> {
        self.project_parts
            .iter()
            .map(|part| part.to_string_lossy().into_owned())
            .collect()
    }

    pub fn project_root(&self) -> String {
        self.project_root.to_string_lossy().into_owned()
    }

    pub fn compile_parts(&self) {
        for part in self.project_parts.clone() {
            let content = fs::read_to_string(part.clone().join("part.brain")).unwrap();
            
            let brain_file = parser::parse(content);
            
            let mut cmd = Command::new(brain_file.compile.first().unwrap());
            cmd.current_dir(part.clone());
            for (i, v) in brain_file.compile.iter().enumerate() {
                if i != 0 {
                    cmd.arg(v);
                }
            }
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
            match cmd.status() {
                Ok(res) => {
                    println!("{}", res.to_string());
                }
                Err(error) => {
                    println!("{}", error.to_string())
                }
            }
        }
    }
}

fn find_parts(path: PathBuf) -> Vec<PathBuf> {
    let mut parts: Vec<PathBuf> = vec![];

    if path.join("part.brain").is_file() {
        return vec![path];
    }

    for dir in list_dirs(path) {
        parts.extend(find_parts(dir));
    }

    parts
}

fn list_dirs(path: PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();

            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

fn verify_parts(parts: Vec<PathBuf>) -> Vec<PartError> {
    let mut errors: Vec<PartError> = vec![];
   
    for part in parts {
        if let Some(err) = find_error_in_part(part.join("part.brain")) {
            errors.extend(vec![err])
        }
    }

    errors
}

fn find_error_in_part(path: PathBuf) -> Option<PartError> {
    match fs::read_to_string(path.clone()) {
        Ok(content) => {
            if parser::is_format_valid(content) {
                None
            } else {
                Some(PartError::BadFormat(path.clone()))
            }
        }
        Err(error) => {
            panic!("{}", format!("An error occurred while reading the file: {}", error.to_string()))
        }
    }
}

