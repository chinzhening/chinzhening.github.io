use clap::{Parser, Subcommand};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod post;

// TODO: move to a config file to be determined dynamically at run time.
const POSTS_DIR: &str = "posts";
const BUILD_DIR: &str = "build";
const ROOT_DIR: &str = ".";

const INDEX_FILENAME: &str = "index.typ";
const STYLE_FILENAME: &str = "style.css";
const SCRIPT_FILENAME: &str = "script.js";
const METADATA_FILENAME: &str = "post-list.json";


#[derive(Parser)]
#[command(name = "ssg")]
#[command(author = "Chin Zhe Ning")]
#[command(version = "0.1.0")]
#[command(about = "A static site generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site (./build is the default directory.)
    Build,
}


fn is_typ(path_buf: &PathBuf) -> bool {
    return path_buf.extension().map(|ext| ext == "typ").unwrap_or(false);
}

fn to_build_path(file_path: &PathBuf) -> PathBuf {
    return PathBuf::from(BUILD_DIR).join(file_path);
}

fn make_html(file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let build_path = to_build_path(file_path).with_extension("html");

    let output = Command::new("typst")
        .args([
            "compile",
            file_path.to_str().unwrap(),
            build_path.to_str().unwrap(),
            "--features", "html",
            "--root",
            ROOT_DIR,
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

    match &cli.command {
        Commands::Build => {
            // 1. Generate metadata for .typ files in ./posts/
            let post_dir = PathBuf::from(POSTS_DIR);
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
                    .filter_map(|path| post::PostMetadata::try_from(path).ok())
                    .collect();
            
            let post_metadata_json = serde_json::to_string_pretty(&post_metadata)?;
            let meta_dir = post_dir.join("meta");
            
            fs::create_dir_all(&meta_dir)?;
            fs::write(meta_dir.join(METADATA_FILENAME).as_path(), &post_metadata_json)?;
            println!("Wrote metadata to {}", meta_dir.join(METADATA_FILENAME).display());

            // 2. Prepare ./build/ directory
            println!("Building site...");
            let build_dir = Path::new(BUILD_DIR);
            if !(build_dir.exists() && build_dir.is_dir()) {
                println!("Creating directory at {}", build_dir.display());
                fs::create_dir_all(build_dir)?;
            }

            // 3. Generate index.html to ./build/
            let index_path = PathBuf::from(ROOT_DIR).join(INDEX_FILENAME);
            if !(index_path.exists() && index_path.is_file()) {
                eprintln!("{} does not exist in the root directory.", INDEX_FILENAME);
                std::process::exit(1);
            }

            make_html(&index_path)?;

            // 4. Generate post.html to ./build/posts/
            let build_post_dir = build_dir.join(POSTS_DIR);
            if !(build_post_dir.exists() && build_post_dir.is_dir()) {
                println!("Creating directory at {}", build_post_dir.display());
                fs::create_dir_all(build_post_dir)?;
            }
            for path  in &post_paths {
                if let Err(e) = make_html(path) {
                    eprintln!("Failed to compile {}:\n{}", path.display(), e);
                }
            }

            // 5. Move style.css over
            // TODO: change to SCSS
            let style_path = PathBuf::from(ROOT_DIR).join(STYLE_FILENAME);
            if style_path.exists() {
                fs::copy(&style_path, to_build_path(&style_path))?;
                println!("Copied {} to build.", STYLE_FILENAME);
            } else {
                println!("Warning: {} not found in root directory.", STYLE_FILENAME);
            }

            // 6. Move fonts, images, and other assets over.
            let script_path = PathBuf::from(ROOT_DIR).join(SCRIPT_FILENAME);
            if script_path.exists() {
                fs::copy(&script_path, to_build_path(&script_path))?;
                println!("Copied {} to build.", SCRIPT_FILENAME);
            } else {
                println!("Warning: {} not found in root directory.", SCRIPT_FILENAME);
            }

            println!("Build successful.");
        }
    }
    Ok(())
}


// TODO: add tests!