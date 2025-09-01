use std::path::PathBuf;

pub struct Env {
    pub dirpath: PathBuf,
    pub variables: Vec<String>,
}

impl Env {
    pub fn new(dirpath: PathBuf) -> Env {
        Env {
            dirpath,
            variables: vec![],
        }
    }
}
