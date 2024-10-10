use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ShellEnv {
    pub cwd: PathBuf,
    // Additional environment variables can be added here
}

impl ShellEnv {
    pub fn new() -> Self {
        Self {
            cwd: env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
        }
    }

    pub fn update_cwd(&mut self) {
        self.cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    }
}
