use clap::{Parser, Subcommand};

use std::{fs, process};
use std::path::PathBuf;
use std::process::Command;

mod config;
mod post;

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


fn is_typ(path_buf: &PathBuf) -> bool {
    return path_buf.extension().map(|ext| ext == "typ").unwrap_or(false);
}

fn is_yml(path_buf: &PathBuf) -> bool {
    return path_buf.extension().map(|ext| ext == "yml" || ext == "yaml" ).unwrap_or(false);
}

fn to_build_path(file_path: &PathBuf, config: &config::Config) -> PathBuf {
    return PathBuf::from(&config.dir.build).join(file_path);
}

fn make_html(file_path: &PathBuf, config: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let build_path = to_build_path(file_path, &config).with_extension("html");
    let metadata_path = PathBuf::from(&config.dir.posts).join("meta").join(&config.filename.metadata);
    

    let output = Command::new("typst")
        .args([
            "compile",
            file_path.to_str().unwrap(),
            build_path.to_str().unwrap(),
            "--features", "html",
            "--root", &(config.dir.root),
            "--input", &format!("post-metadata-path=\\{}", metadata_path.to_str().unwrap())
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
        .unwrap_or_else(|| PathBuf::from(config::get_default_path()));

    // Check if config file exists.
    if !(config_path.exists() && config_path.is_file()) {
        eprintln!(
            "Error: Config file not found or is not a valid file: {}",
            config_path.display()
        );
        process::exit(1);
    // Check if config file has .yml or .yaml extension.
    } else if !is_yml(&config_path) {
        eprintln!(
            "Config file at '{}' does not have the .yml or .yaml extension",
            config_path.display()
        );
        process::exit(1);
    }

    // Create config
    let config = config::Config::try_from(&config_path)?;
    let metadata_factory = post::MetadataFactory::new(&config);


    match &cli.command {
        Commands::Build => {
            // Generate metadata for post.typ files
            let post_dir = PathBuf::from(&(config.dir.posts));
            let post_paths: Vec<PathBuf> = 
                fs::read_dir(&post_dir)?
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| path.is_file())
                    .filter(is_typ)
                    .collect();
            
            let post_metadata: Vec<post::PostMetadata> =
                post_paths
                    .iter()
                    .filter_map(|path| (&metadata_factory).build(&path).ok())
                    .collect();
            
            let post_metadata_json = serde_json::to_string_pretty(&post_metadata)?;
            let meta_dir = post_dir.join("meta");
            
            fs::create_dir_all(&meta_dir)?;
            println!("Writing post metadata to {}", meta_dir.join(&config.filename.metadata).display());
            fs::write(meta_dir.join(&config.filename.metadata).as_path(), &post_metadata_json)?;

            // Prepare the build directory
            println!("Building site...");
            let build_dir = PathBuf::from(&config.dir.build);
            if !(build_dir.exists() && build_dir.is_dir()) {
                println!("Creating directory at {}", build_dir.display());
                fs::create_dir_all(&build_dir)?;
            }

            // Generate index.html to build
            let root_dir = PathBuf::from(&config.dir.root);
            let index_path = root_dir.join(&config.filename.index);
            if !(index_path.exists() && index_path.is_file()) {
                eprintln!("{} does not exist in the root directory.", &config.filename.index);
                std::process::exit(1);
            }

            make_html(&index_path, &config)?;

            // Generate post.html to build
            let build_post_dir = to_build_path(&post_dir, &config);
            if !(build_post_dir.exists() && build_post_dir.is_dir()) {
                println!("Creating directory at {}", build_post_dir.display());
                fs::create_dir_all(build_post_dir)?;
            }
            for path  in &post_paths {
                if let Err(e) = make_html(&path, &config) {
                    eprintln!("Failed to compile {}: {}", path.display(), e);
                }
            }

            // Move style.css to build
            // TODO: change to SCSS
            let style_path = root_dir.join(&config.filename.style);
            if style_path.exists() {
                println!("Copying {} to build.", style_path.display());
                fs::copy(&style_path, to_build_path(&style_path, &config))?;
            } else {
                println!("Warning: {} not found in root directory.", &config.filename.style);
            }

            // Move script.js to build
            let script_path = root_dir.join(&config.filename.script);
            if script_path.exists() {
                fs::copy(&script_path, to_build_path(&script_path, &config))?;
                println!("Copied {} to build.", script_path.display());
            } else {
                println!("Warning: {} not found in root directory.", &config.filename.script);
            }

            // Move fonts to build
            let fonts_dir = PathBuf::from(&config.dir.fonts);
            if !(fonts_dir.is_dir() && fonts_dir.exists()) {
                println!("Warning: {} not found in root directory.", fonts_dir.display())
            } else {
                match &config.font.default {
                    Some(default_font) => {
                        let default_font_dir = fonts_dir.join(&default_font);
                        if !(default_font_dir.is_dir() && default_font_dir.exists()) {
                            println!("Warning: {} directory not found.", default_font_dir.display());
                        } else {
                            let default_font_paths: Vec<PathBuf> = fs::read_dir(&default_font_dir)?
                                .flatten()
                                .map(|entry| entry.path())
                                .filter(|path| path.is_file())
                                .collect();

                            let build_default_font_dir = to_build_path(&default_font_dir, &config);
                            if !(build_default_font_dir.exists() && build_default_font_dir.is_dir()) {
                                println!("Creating directory at {}", build_default_font_dir.display());
                                fs::create_dir_all(build_default_font_dir)?;
                            }
                            for path in &default_font_paths {
                                fs::copy(&path, to_build_path(&path, &config))?;
                            }
                        }

                        // Move default default-font.css to build
                        let default_font_style_path = root_dir.join(&default_font).with_extension("css");
                        if default_font_style_path.exists() {
                            println!("Copying {} to build.", default_font_style_path.display());
                            fs::copy(&default_font_style_path, to_build_path(&default_font_style_path, &config))?;
                        } else {
                            println!("Warning: {} not found.", default_font_style_path.display());
                        }

                    }
                    None => {
                        println!("Using default system fonts.");
                    }
                }
            }
            

            println!("Build successful.");
        }
    }
    Ok(())
}


// TODO: add tests!