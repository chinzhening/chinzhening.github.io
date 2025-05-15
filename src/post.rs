use serde::Serialize;
use serde_json::Value;

use std::path::PathBuf;
use std::process::Command;

use crate::config;

fn query(file_path: &str, root_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let output = Command::new("typst")
        .args([
            "query",
            file_path, "<front-matter>",
            "--field", "value",
            "--input", "front-matter=true",
            "--root", root_path,
            "--one",
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("Query failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    let front_matter: Value = serde_json::from_slice(&output.stdout)?;
    Ok(front_matter)
}

pub struct MetadataFactory<'a> {
    config: &'a config::Config,
}

impl <'a> MetadataFactory<'a> {
    pub fn new(config: &'a config::Config) -> Self {
        MetadataFactory { config }
    }

    pub fn build(&self, file_path: &PathBuf) -> Result<PostMetadata, Box<dyn std::error::Error>> {
        let frontmatter = query(file_path.to_str().unwrap(),&self.config.dir.root)?;
        Ok(PostMetadata {
            path: file_path.to_string_lossy().into(),
            frontmatter: frontmatter,
        })
    }
}


#[derive(Serialize)]
pub struct PostMetadata {
    path: String,
    frontmatter: Value,
}