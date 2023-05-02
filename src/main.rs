mod parse_lib;

use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::parse_lib::SortBy;

/// txlib: text based epub library management
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// sort by: date, title, author, pages or series
    #[arg(short, long, default_value = "date")]
    sort: String,
    /// reverse sorting order
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    reverse: bool,
    /// print output without saving to DB
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    no_save: bool,
    /// open DB file (does not run the rest of the app)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    open_db: bool,
}

/// Returns `epub_dir_path` if verifications succeed, else returns error message.
fn startup_verifications(config_path: PathBuf, config_file: PathBuf) -> Result<PathBuf, &'static str> {
    if fs::create_dir_all(config_path).is_err() {
        return Err("error: unable to access config directory");
    }

    if !config_file.exists() {
        if fs::write(config_file, "library_path=").is_err() {
            return Err("error: unable to create config file");
        }
        return Err(
            "error: no config file found. In the config file 'txlibrc' located in \
            '$HOME/.config/txlib', add the path to the root directory to search for epub files",
        );
    }

    let Ok(config_content) = fs::read_to_string(config_file) else {
        return Err("error: unable to read config file")
    };

    let epub_dir_path = PathBuf::from(config_content.split('=').last().unwrap());
    if !epub_dir_path.exists() {
        return Err("error: provided epub library path does not exist");
    }
    Ok(epub_dir_path)
}

/// Makes a backup of the epub library DB before startup.
fn backup_library_db(lib_db_file: &Path) -> Result<(), &'static str> {
    if !lib_db_file.exists() {
        return Err("error: unable to access DB file");
    }
    if fs::copy(lib_db_file, lib_db_file.with_extension("txt.bak")).is_err() {
        return Err("error: unable to create DB backup");
    }
    Ok(())
}

/// Returns `SortBy` enum after parsing string sorting option.
fn parse_sorting_option(sort_str: &str) -> SortBy {
    match sort_str.to_lowercase().as_str() {
        "r" | "read" => SortBy::Read,
        "t" | "title" => SortBy::Title,
        "a" | "author" => SortBy::Author,
        "p" | "pages" => SortBy::Pages,
        "s" | "series" => SortBy::Series,
        _ => SortBy::Date,
    }
}

/// Open library DB file if exists.
fn open_db_file(lib_db_file: &Path) -> Result<(), &'static str> {
    if !lib_db_file.exists() {
        return Err("error: unable to locate DB file");
    }
    if open::with_command(lib_db_file, "xdg-open").status().is_err() {
        return Err("error: unable to open DB file");
    }
    Ok(())
}

/// Runs the program logic.
fn run_txlib() -> Result<(), &'static str> {
    // Parse CLI input
    let args = Args::parse();
    let sort_by = parse_sorting_option(&args.sort);

    // Config file paths
    let config_path = dirs::config_dir().unwrap().join("txlib_bak");
    let config_file = config_path.join("txlibrc");
    let lib_db_file = config_path.join("epub_db.txt");

    // Open DB file (if required)
    if args.open_db {
        open_db_file(&lib_db_file)?;
    }

    // Verify config file
    let epub_dir_path = startup_verifications(config_path, config_file)?;

    // Get DB file and create a backup
    backup_library_db(&lib_db_file)?;

    // Load library and write it to stdout and/or DB file
    let library = parse_lib::load_library(&lib_db_file, epub_dir_path.to_str().unwrap(), &sort_by, args.reverse)?;
    parse_lib::write_library(&library, &lib_db_file, args.no_save)?;
    Ok(())
}

fn main() {
    if let Err(error_msg) = run_txlib() {
        println!("{error_msg}");
        exit(1);
    }
}
