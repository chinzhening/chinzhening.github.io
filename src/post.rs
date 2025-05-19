use serde::Serialize;
use serde_json::Value;

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::path_manager;

#[derive(Serialize)]
pub struct PostMetadata {
    path: String,
    frontmatter: Value,
}

fn query(file_path: &PathBuf, root_path: &PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    match (file_path.to_str(), root_path.to_str()) {
        (Some(file_path), Some(root_path)) => {
            let output = Command::new("typst")
                .args([
                    "query",
                    file_path, "<front-matter>",
                    "--field", "value",
                    "--input", "front-matter=true",
                    "--root", root_path,
                    "--one",
                ])
                .output();

            match output {
                Ok(valid_output) => {
                    let front_matter: Value = serde_json::from_slice(&valid_output.stdout)?;
                    Ok(front_matter)
                }
                Err(e) => Err(format!("Query failed: {}", e).into()),
            }
                
        }
        (None, _) => Err("Invalid file path.".into()),
        (_, None) => Err("Invalid root path.".into()),
    }
}

pub struct PostMetadataFactory<'a> {
    path_manager: &'a path_manager::PathManager,
}

impl <'a> PostMetadataFactory<'a> {
    pub fn new(path_manager: &'a path_manager::PathManager) -> Self {
        PostMetadataFactory { path_manager }
    }

    fn create_metadata(&self, file_path: &PathBuf) -> Result<PostMetadata, Box<dyn std::error::Error>> {
        let frontmatter = query(
            file_path,
            &self.path_manager.root())?;
        Ok(
            PostMetadata {
                path: file_path.to_string_lossy().into(),
                frontmatter: frontmatter,
            }
        )
    }

    // Generate all metadata for post.typ files
    pub fn generate_metadata(&self) -> Result<(), Box<dyn std::error::Error>> {
        let posts_paths: Vec<PathBuf> = self.path_manager.posts_paths()?;
        
        let post_metadata: Vec<PostMetadata> =
            posts_paths
                .iter()
                .filter_map(|path| self.create_metadata(&path).ok())
                .collect();
        
        let post_metadata_json = serde_json::to_string_pretty(&post_metadata)?;
        let post_metadata_path = self.path_manager.posts_metadata_path();
        
        println!("Writing post metadata to {}", post_metadata_path.display());
        fs::write(&post_metadata_path, &post_metadata_json)?;
        Ok(())
    }
}