mod parse_lib;

use clap::Parser;
use std::fs;
use std::path::Path;

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

/// If config file was verified, returns (true, epub_dir_path). Else, returns (false, "").
fn startup_verifications(config_path: &Path, config_file: &Path) -> (bool, String) {
    // Verify config directory path
    fs::create_dir_all(config_path).expect("Unable to access config directory.");

    // Verify config file and load epub library path
    let epub_dir_path;
    if !config_file.exists() {
        println!(
            "No config file found. In the config file 'txlibrc' located in \
            '$HOME/.config/txlib' add the path to the root directory to search for epub files."
        );
        fs::write(config_file, "library_path=").expect("Unable to create config file.");
        return (false, "".to_string());
    } else {
        let config_content = fs::read_to_string(config_file).expect("Unable to read config file.");
        epub_dir_path = (config_content.split("=").collect::<Vec<&str>>())[1].to_string();
    }

    // Verify provided epub library path
    if !Path::new(&epub_dir_path).exists() {
        println!("Provided epub library path does not exist.");
        return (false, "".to_string());
    }

    (true, epub_dir_path)
}

/// Makes a backup of the epub library DB before startup.
fn backup_library_db(lib_db_file: &str) {
    if Path::new(&lib_db_file).exists() {
        let bak_dg = lib_db_file.to_owned() + ".bak";
        fs::copy(lib_db_file, Path::new(bak_dg.as_str())).expect("Unable to create DB backup.");
    }
}

/// Returns SortBy enum after parsing string sorting option.
fn parse_sorting_option(sort_str: String) -> SortBy {
    match sort_str.to_lowercase().as_str() {
        "d" | "date" => SortBy::Date,
        "r" | "read" => SortBy::Read,
        "t" | "title" => SortBy::Title,
        "a" | "author" => SortBy::Author,
        "p" | "pages" => SortBy::Pages,
        "s" | "series" => SortBy::Series,
        _ => SortBy::Date,
    }
}

/// Open library DB file if exists.
fn open_db_file(lib_db_file: &Path) {
    if lib_db_file.exists() {
        open::that(lib_db_file).expect("Unable to open DB file.");
    }
}

fn main() {
    // Parse CLI input
    let args = Args::parse();
    let sort_by = parse_sorting_option(args.sort);

    // Config file paths
    let config_path = dirs::config_dir().unwrap().join("txlib");
    let config_file = config_path.join("txlibrc");
    let lib_db_file = config_path.join("epub_db.txt");

    // Open DB file (if required)
    if args.open_db {
        open_db_file(lib_db_file.as_path());
        return;
    }

    // Verify config file
    let verified = startup_verifications(config_path.as_path(), config_file.as_path());
    if !verified.0 {
        return;
    }
    let epub_dir_path = verified.1;

    // Get DB file and create a backup
    let lib_db_file_str = lib_db_file.as_path().to_str().unwrap();
    backup_library_db(lib_db_file_str);

    // Load library and write it to stdout and/or DB file
    let lib = parse_lib::load_library(
        lib_db_file_str,
        epub_dir_path.as_str(),
        sort_by,
        args.reverse,
    );
    parse_lib::write_library(&lib, lib_db_file_str, args.no_save);
}
