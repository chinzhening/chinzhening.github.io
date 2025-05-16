use serde::Deserialize;
use serde_yaml;

use std::path::PathBuf;

const DEFAULT_CONFIG_PATH: &str = "config.yml";

pub fn get_default_path() -> PathBuf {
    PathBuf::from(DEFAULT_CONFIG_PATH)
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub dir: Dirs,
    pub filename: Filenames,
    pub font: Font,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Dirs {
    pub posts: String,
    pub build: String,
    pub root: String,
    pub fonts: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Filenames {
    pub index: String,
    pub style: String,
    pub script: String,
    pub metadata: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Font {
    pub default: Option<String>,
}

impl TryFrom<&PathBuf> for Config {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let yml_bytes = std::fs::read(&path)?;
        let config: Config = serde_yaml::from_slice(&yml_bytes)?;
        Ok(config)
    }
}