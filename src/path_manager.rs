use std::fs;
use std::path::PathBuf;

use crate::config;
use crate::utility;

const DEFAULT_CONFIG_PATH: &str = "config.yml";

pub fn default_config_path() -> PathBuf {
    PathBuf::from(DEFAULT_CONFIG_PATH)
}

pub struct PathManager {
    config: config::Config,
    root: PathBuf,
}

impl PathManager {
    pub fn new(config: config::Config) -> Result<Self, Box<dyn std::error::Error>> {
        let root = PathBuf::from(config.get_root());
        let manager = PathManager { config, root };

        Ok(manager)
    }

    fn create_dirs_from(&self, dirs: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        for dir in dirs {
            let path = self.root.join(dir);
            if !path.exists() {
                println!("Creating directory at {}", path.display());
                fs::create_dir_all(&path)?;
            }
        }
        Ok(())
    }

    pub fn create_dirs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dirs = if self.root() != PathBuf::from(self.config.get_root()) { self.config.get_build_dirs() } else {self.config.get_dirs() };
        self.create_dirs_from(dirs)
    }

    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }

    pub fn to_build(&self) -> PathManager {
        PathManager { 
            config: self.config.clone(),
            root: self.build_dir(),
        }
    }

    pub fn to_build_path(&self, path: &PathBuf) -> PathBuf {
        self.build_dir().join(path)
    }

    pub fn build_dir(&self) -> PathBuf {
        self.root.join(self.config.get_build_dir())
    }

    pub fn fonts_dir(&self) -> PathBuf {
        self.root.join(self.config.get_fonts_dir())
    }

    pub fn meta_dir(&self) -> PathBuf {
        self.root.join(self.config.get_meta_dir())
    }

    pub fn posts_dir(&self) -> PathBuf {
        self.root.join(self.config.get_posts_dir())
    }

    pub fn styles_dir(&self) -> PathBuf {
        self.root.join(self.config.get_styles_dir())
    }

    pub fn index_path(&self) -> PathBuf {
        self.root.join(self.config.get_index())
    }

    pub fn style_path(&self) -> PathBuf {
        self.styles_dir().join(self.config.get_style())
    }

    pub fn script_path(&self) -> PathBuf {
        self.root.join(self.config.get_script())
    }

    pub fn post_paths(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let post_paths: Vec<PathBuf> = fs::read_dir(self.posts_dir())?
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .filter(utility::is_typ)
            .collect();
        Ok(post_paths)
    }

    pub fn posts_metadata_path(&self) -> PathBuf {
        self.meta_dir().join(self.config.get_posts_metadata())
    }

    pub fn fonts_metadata_path(&self) -> PathBuf {
        self.meta_dir().join(self.config.get_fonts_metadata())
    }

    pub fn default_font_dir(&self) -> PathBuf {
        self.fonts_dir().join(self.config.get_fonts_default())
    }

    pub fn primary_font_dir(&self) -> Option<PathBuf> {
        self.config.get_fonts_primary().map(|p|self.fonts_dir().join(p))
    }

    pub fn secondary_font_dir(&self) -> Option<PathBuf> {
        self.config.get_fonts_secondary().map(|p|self.fonts_dir().join(p))
    }

    pub fn default_font_style_path(&self) -> PathBuf {
        self.styles_dir().join(self.config.get_fonts_default()).with_extension("css")
    }

    fn to_style_path(&self, font_dir: Option<PathBuf>) -> Option<PathBuf> {
        font_dir
            .and_then(|dir| {
                dir.file_name()
                .and_then(|os_str| os_str.to_str())
                .map(|font_name| self.styles_dir().join(font_name).with_extension("css"))
                }
            )
    }

    pub fn primary_font_style_path(&self) -> Option<PathBuf> {
        self.to_style_path(self.primary_font_dir())
    }

    pub fn secondary_font_style_path(&self) -> Option<PathBuf> {
        self.to_style_path(self.secondary_font_dir())
    }

}
