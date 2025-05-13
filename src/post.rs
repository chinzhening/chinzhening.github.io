use serde::Serialize;
use serde_json::Value;

use std::path::PathBuf;
use std::process::Command;

use crate::ROOT_DIR;

fn query(file_path: &PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    let output = Command::new("typst")
        .args([
            "query",
            file_path.to_str().unwrap(), "<front-matter>",
            "--field", "value",
            "--input", "front-matter=true",
            "--root", ROOT_DIR,
            "--one",
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("Query failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    let front_matter: Value = serde_json::from_slice(&output.stdout)?;
    Ok(front_matter)
}


#[derive(Serialize)]
pub struct PostMetadata {
    path: String,
    frontmatter: Value,
}

impl TryFrom<&PathBuf> for PostMetadata {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let frontmatter = query(&path)?;
        let path = path.to_string_lossy().to_string();
        Ok(PostMetadata { path, frontmatter })
    }
}