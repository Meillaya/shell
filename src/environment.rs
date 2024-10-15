use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ShellEnv {
    pub cwd: PathBuf,
    pub path: String,
}

impl ShellEnv {
    pub fn new() -> Self {
        Self {
            cwd: env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            path: env::var("PATH").unwrap_or_default(),
        }
    }

    pub fn update_cwd(&mut self) {
        self.cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
}
