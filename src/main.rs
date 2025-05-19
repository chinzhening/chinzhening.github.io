use clap::{Parser, Subcommand};

use std::{fs, process};
use std::path::PathBuf;
use std::process::Command;

mod config;
mod font;
mod path_manager;
mod post;
mod utility;

#[derive(Parser)]
#[command(name = "ssg")]
#[command(author = "Chin Zhe Ning")]
#[command(version = "0.1.0")]
#[command(about = "A static site generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Sets custom file path to the config file.
    #[arg(short, long)]
    config_path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site
    Build,
}

fn make_html(file_path: &PathBuf, path_manager: &path_manager::PathManager) -> Result<(), Box<dyn std::error::Error>> {
    let build_path = path_manager.to_build_path(file_path).with_extension("html");

    let output = Command::new("typst")
        .args([
            "compile",
            file_path.to_str().unwrap(),
            build_path.to_str().unwrap(),
            "--features", "html",
            "--root", path_manager.root().to_str().unwrap(),
            "--input", &format!("posts-metadata-path=\\{}", path_manager.posts_metadata_path().to_str().unwrap()),
            "--input", &format!("fonts-metadata-path=\\{}", path_manager.fonts_metadata_path().to_str().unwrap())
        ])
        .output()?;
    if !output.status.success() {
        return Err(format!("{}", String::from_utf8_lossy(&output.stderr)).into());
    }
    println!("Creating {}", build_path.display());
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config_path = cli
        .config_path
        .unwrap_or_else(|| PathBuf::from(path_manager::default_config_path()));

    // Check if config file exists.
    if !(config_path.exists() && config_path.is_file()) {
        eprintln!(
            "Error: Config file not found or is not a valid file: {}",
            config_path.display()
        );
        process::exit(1);
    // Check if config file has .yml or .yaml extension.
    } else if !utility::is_yml(&config_path) {
        eprintln!(
            "Config file at '{}' does not have the .yml or .yaml extension",
            config_path.display()
        );
        process::exit(1);
    }

    // Create config
    let config = config::Config::new(&config_path)?;

    // Verify config project structure
    let path_manager = path_manager::PathManager::new(config)?;
    path_manager.create_dirs()?;
    path_manager.to_build().create_dirs()?;
    
    // Create metadata factories
    let post_metadata_factory = post::PostMetadataFactory::new(&path_manager);
    let font_metadata_factory = font::FontMetadataFactory::new(&path_manager);


    match &cli.command {
        Commands::Build => {
            // Generate metadata
            println!("Generating metadata...");
            post_metadata_factory.generate_metadata()?;
            font_metadata_factory.generate_metadata()?;

            // Build site
            println!("Building site...");

            // Generate index.html to build
            let index_path = path_manager.index_path();
            if !(index_path.exists() && index_path.is_file()) {
                eprintln!("{} does not exist.", index_path.display());
                std::process::exit(1);
            }

            make_html(&index_path, &path_manager)?;

            // Generate post.html to build
            for path in path_manager.posts_paths()? {
                if let Err(e) = make_html(&path, &path_manager) {
                    eprintln!("Failed to compile {}: {}", path.display(), e);
                }
            }

            // Move assets to build
            for path in path_manager.assets_paths()? {
                if let Err(e) = fs::copy(&path, &path_manager.to_build_path(&path)) {
                    eprintln!("Failed to move {}: {}", path.display(), e);
                }
            }

            // Move style.css to build
            // TODO: change to SCSS
            let style_path = path_manager.style_path();
            if style_path.exists() {
                println!("Copying {} to build.", style_path.display());
                fs::copy(&style_path, path_manager.to_build().style_path())?;
            } else {
                println!("Warning: {} not found.", path_manager.style_path().display());
            }

            // Move script.js to build
            let script_path = path_manager.script_path();
            if script_path.exists() {
                fs::copy(&script_path, path_manager.to_build().script_path())?;
                println!("Copied {} to build.", script_path.display());
            } else {
                println!("Warning: {} not found.", script_path.display());
            }

            // Move default font to build
            let default_font_dir = path_manager.default_font_dir();
            if !(default_font_dir.is_dir() && default_font_dir.exists()) {
                println!("Warning: {} not found.", default_font_dir.display());
            } else {
                let font_contents: Vec<PathBuf> = fs::read_dir(&default_font_dir)?
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| path.is_file())
                    .collect();

                let build_default_font_dir = path_manager.to_build().default_font_dir();
                if !(build_default_font_dir.exists() && build_default_font_dir.is_dir()) {
                    println!("Creating directory at {}", build_default_font_dir.display());
                    fs::create_dir_all(build_default_font_dir)?;
                }
                for path in &font_contents {
                    fs::copy(&path, path_manager.to_build_path(&path))?;
                }

                // Move default default-font.css to build
                if let Some(font_metadata) = font_metadata_factory.generate_default()? {
                    
                    let font_style_path = font_metadata.font_style_path();
                    if font_style_path.exists() {
                        println!("Copying {} to build.", font_style_path.display());
                        fs::copy(&font_style_path, path_manager.to_build_path(&font_style_path))?;
                    } else {
                        println!("Warning: {} not found.", font_style_path.display());
                    }
                }
            }

            // Move primary font to build
            if let Some(primary_font_dir) = path_manager.primary_font_dir() {
                if !(primary_font_dir.is_dir() && primary_font_dir.exists()) {
                    println!("Warning: {} not found.", primary_font_dir.display());
                } else {
                    let font_contents: Vec<PathBuf> = fs::read_dir(&primary_font_dir)?
                        .flatten()
                        .map(|entry| entry.path())
                        .filter(|path| path.is_file())
                        .collect();

                    if let Some(build_primary_font_dir) = path_manager.to_build().primary_font_dir() {
                        if !(build_primary_font_dir.exists() && build_primary_font_dir.is_dir()) {
                            println!("Creating directory at {}", build_primary_font_dir.display());
                            fs::create_dir_all(build_primary_font_dir)?;
                        }
                        for path in &font_contents {
                            fs::copy(&path, path_manager.to_build_path(&path))?;
                        }
                    }
                    
                    // Move primary primary-font.css to build
                    if let Some(font_metadata) = font_metadata_factory.generate_primary()? {
                        let font_style_path = font_metadata.font_style_path();
                        if font_style_path.exists() {
                            println!("Copying {} to build.", font_style_path.display());
                            fs::copy(&font_style_path, path_manager.to_build_path(&font_style_path))?;
                        } else {
                            println!("Warning: {} not found.", font_style_path.display());
                        }
                    }
                }
            }

            // Move secondary font to build
            if let Some(secondary_font_dir) = path_manager.secondary_font_dir() {
                if !(secondary_font_dir.is_dir() && secondary_font_dir.exists()) {
                    println!("Warning: {} not found.", secondary_font_dir.display());
                } else {
                    let font_contents: Vec<PathBuf> = fs::read_dir(&secondary_font_dir)?
                        .flatten()
                        .map(|entry| entry.path())
                        .filter(|path| path.is_file())
                        .collect();

                    if let Some(build_secondary_font_dir) = path_manager.to_build().secondary_font_dir() {
                        if !(build_secondary_font_dir.exists() && build_secondary_font_dir.is_dir()) {
                            println!("Creating directory at {}", build_secondary_font_dir.display());
                            fs::create_dir_all(build_secondary_font_dir)?;
                        }
                        for path in &font_contents {
                            fs::copy(&path, path_manager.to_build_path(&path))?;
                        }
                    }
                    
                    // Move secondary secondary-font.css to build
                    if let Some(font_metadata) = font_metadata_factory.generate_secondary()? {
                        let font_style_path = font_metadata.font_style_path();
                        if font_style_path.exists() {
                            println!("Copying {} to build.", font_style_path.display());
                            fs::copy(&font_style_path, path_manager.to_build_path(&font_style_path))?;
                        } else {
                            println!("Warning: {} not found.", font_style_path.display());
                        }
                    }
                }
            }

            println!("Build successful.");
        }
    }
    Ok(())
}


// TODO: add tests!