use serde::Deserialize;
use serde_yaml;

use std::path::PathBuf;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Config {
    root: RootConfig,
    build: BuildConfig,
    fonts: FontsConfig,
    meta: MetaConfig,
    posts: PostsConfig,
    styles: StylesConfig,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
struct RootConfig {
    dir: String,
    index: String,
    script: String,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
struct BuildConfig {
    dir: String,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
struct FontsConfig {
    dir: String,
    default: String,
    primary: Option<String>,
    secondary: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
struct MetaConfig {
    dir: String,
    posts: String,
    fonts: String,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
struct PostsConfig {
    dir: String,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
struct StylesConfig {
    dir: String,
    style: String,
}


impl Config {
    pub fn new(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let yml_bytes = std::fs::read(&path)?;
        let config: Config = serde_yaml::from_slice(&yml_bytes)?;
        Ok(config)
    }

    pub fn get_root(&self) -> String {
        return self.root.dir.clone();
    }

    pub fn get_dirs(&self) -> Vec<String> {
        return vec![
            self.posts.dir.clone(),
            self.build.dir.clone(),
            self.fonts.dir.clone(),
            self.meta.dir.clone(),
            self.styles.dir.clone(),
        ]
    }

    pub fn get_build_dirs(&self) -> Vec<String> {
        return vec![
            self.posts.dir.clone(),
            self.fonts.dir.clone(),
            self.styles.dir.clone(),
        ]
    }

    pub fn get_build_dir(&self) -> String {
        return self.build.dir.clone();
    }

    pub fn get_fonts_dir(&self) -> String {
        return self.fonts.dir.clone();
    }

    pub fn get_meta_dir(&self) -> String {
        return self.meta.dir.clone(); 
    }

    pub fn get_posts_dir(&self) -> String {
        return self.posts.dir.clone();
    }

    pub fn get_styles_dir(&self) -> String {
        return self.styles.dir.clone();
    }

    pub fn get_posts_metadata(&self) -> String {
        return self.meta.posts.clone();
    }

    pub fn get_fonts_metadata(&self) -> String {
        return self.meta.fonts.clone();
    }

    pub fn get_fonts_default(&self) -> String {
        return self.fonts.default.clone()
    }

    pub fn get_fonts_primary(&self) -> Option<String> {
        return self.fonts.primary.clone()
    }

    pub fn get_fonts_secondary(&self) -> Option<String> {
        return self.fonts.secondary.clone()
    }

    pub fn get_style(&self) -> String {
        return self.styles.style.clone();
    }

    pub fn get_index(&self) -> String {
        return self.root.index.clone();
    }

    pub fn get_script(&self) -> String {
        return self.root.script.clone();
    }
}