use convert_case::{Case, Casing};
use serde::Serialize;

use std::fs;
use std::path::PathBuf;

use crate::path_manager;

#[derive(Serialize)]
pub struct FontMetadata {
    font_family: String,
    font_style_path: String,
    toggle_class: String,
    status: String,
}

impl FontMetadata {
    pub fn font_style_path(&self) -> PathBuf {
        PathBuf::from(self.font_style_path.clone())
    }
}

// TODO: enum status { default, primary, secondary }

pub struct FontMetadataFactory<'a> {
    path_manager: &'a path_manager::PathManager,
}

impl <'a> FontMetadataFactory<'a> {
    pub fn new(path_manager: &'a path_manager::PathManager) -> Self {
        FontMetadataFactory {
            path_manager,
        }
    }

    fn create_metadata(&self, font_path: &PathBuf, font_style_path: &PathBuf, status: &str) -> Result<FontMetadata, Box<dyn std::error::Error>> {
        match font_path.file_name().and_then(|os|os.to_str()) {
            Some(font_name) => {
                let font_family = font_name
                    .from_case(Case::Kebab)
                    .to_case(Case::Title);
    
                let toggle_class = format!("font--{}", font_name);
                let font_style_path = font_style_path.to_string_lossy().into();
                let status = status.to_string();

                Ok(FontMetadata {
                    font_family,
                    font_style_path,
                    toggle_class,
                    status,
                })
            }
            None => Err(format!("Invalid font path: {}", font_path.display()).into())
        }   
}

    pub fn generate_default(&self) -> Result<Option<FontMetadata>, Box<dyn std::error::Error>> {
        let dir = self.path_manager.default_font_dir();
        let style_path = self.path_manager.default_font_style_path();
        let font_metadata = match self.create_metadata(
            &dir,
            &style_path,
            "default"
        ) {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                eprintln!("Failed to create metadata for default font: {}", e);
                None
            }
        };
        Ok(font_metadata)
    }

    pub fn generate_primary(&self) -> Result<Option<FontMetadata>, Box<dyn std::error::Error>> {
        let dir = match self.path_manager.primary_font_dir() {
            Some(d) => d,
            None => return Ok(None),
        };

        let style_path = match self.path_manager.primary_font_style_path() {
            Some(p) => p,
            None => return Ok(None),
        };

        let font_metadata = match self.create_metadata(&dir, &style_path, "primary") {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                eprintln!("Failed to create metadata for primary font: {}", e);
                None
            }
        };

        Ok(font_metadata)
    }

    pub fn generate_secondary(&self) -> Result<Option<FontMetadata>, Box<dyn std::error::Error>> {
        let dir = match self.path_manager.secondary_font_dir() {
            Some(d) => d,
            None => return Ok(None),
        };

        let style_path = match self.path_manager.secondary_font_style_path() {
            Some(p) => p,
            None => return Ok(None),
        };

        let font_metadata = match self.create_metadata(&dir, &style_path, "secondary") {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                eprintln!("Failed to create metadata for secondary font: {}", e);
                None
            }
        };

        Ok(font_metadata)
    }

    pub fn generate_metadata(&self) -> Result<(), Box<dyn std::error::Error>> {
        let default = self.generate_default()?;
        let primary = self.generate_primary()?;
        let secondary = self.generate_secondary()?;

        let font_metadata: Vec<FontMetadata> = vec![default, primary, secondary]
            .into_iter()
            .flatten()
            .collect();

        // Serialize to JSON
        let font_metadata_json = serde_json::to_string_pretty(&font_metadata)?;
        let font_metadata_path = self.path_manager.fonts_metadata_path();
        fs::write(&font_metadata_path, font_metadata_json)?;
        Ok(())
    }
}