use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keep_stacks: Option<Vec<String>>,
    pub path: Option<String>,
}

pub fn load_config() -> Option<Config> {
    let path: PathBuf = home_dir()?.join("homelabctl.toml");
    let content = fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}
