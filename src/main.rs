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
    /// Build the static site (./build is the default directory.)
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
    println!("Created {}", build_path.display());
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
    // Check if config file is yml.
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
            // 1. Generate metadata for .typ files in ./posts/
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
            fs::write(meta_dir.join(&config.filename.metadata).as_path(), &post_metadata_json)?;
            println!("Wrote metadata to {}", meta_dir.join(&config.filename.metadata).display());

            // 2. Prepare ./build/ directory
            println!("Building site...");
            let build_dir = PathBuf::from(&config.dir.build);
            if !(build_dir.exists() && build_dir.is_dir()) {
                println!("Creating directory at {}", build_dir.display());
                fs::create_dir_all(&build_dir)?;
            }

            // 3. Generate index.html to ./build/
            let root_dir = PathBuf::from(&config.dir.root);
            let index_path = root_dir.join(&config.filename.index);
            if !(index_path.exists() && index_path.is_file()) {
                eprintln!("{} does not exist in the root directory.", &config.filename.index);
                std::process::exit(1);
            }

            make_html(&index_path, &config)?;

            // 4. Generate post.html to ./build/posts/
            let build_post_dir = build_dir.join(&post_dir);
            if !(build_post_dir.exists() && build_post_dir.is_dir()) {
                println!("Creating directory at {}", build_post_dir.display());
                fs::create_dir_all(build_post_dir)?;
            }
            for path  in &post_paths {
                if let Err(e) = make_html(&path, &config) {
                    eprintln!("Failed to compile {}:\n{}", path.display(), e);
                }
            }

            // 5. Move style.css over
            // TODO: change to SCSS
            let style_path = root_dir.join(&config.filename.style);
            if style_path.exists() {
                fs::copy(&style_path, to_build_path(&style_path, &config))?;
                println!("Copied {} to build.", style_path.display());
            } else {
                println!("Warning: {} not found in root directory.", &config.filename.style);
            }

            // 6. Move fonts, images, and other assets over.
            let script_path = root_dir.join(&config.filename.script);
            if script_path.exists() {
                fs::copy(&script_path, to_build_path(&script_path, &config))?;
                println!("Copied {} to build.", script_path.display());
            } else {
                println!("Warning: {} not found in root directory.", &config.filename.script);
            }

            println!("Build successful.");
        }
    }
    Ok(())
}


// TODO: add tests!