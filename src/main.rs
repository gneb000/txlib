use std::fs;
use std::path::Path;

mod parse_lib;

/// If config file was verified, returns (true, epub_dir_path). Else, returns (false, "").
fn startup_verifications(config_path: &Path, config_file: &Path) -> (bool, String) {
    // Verify config directory path
    fs::create_dir_all(config_path).expect("Unable to access config directory.");

    // Verify config file and load epub library path
    let epub_dir_path;
    if !config_file.exists() {
        println!("No config file found. In the config file 'telmrc' located in \
            '$HOME/.config/telm' add the path to the root directory to search for epub files.");
        fs::write(config_file, "library_path=")
            .expect("Unable to create config file.");
        return (false, "".to_string())
    } else {
        let config_content = fs::read_to_string(config_file)
            .expect("Unable to read config file.");
        epub_dir_path = (config_content.split("=").collect::<Vec<&str>>())[1].to_string();
    }

    // Verify provided epub library path
    if !Path::new(&epub_dir_path).exists() {
        println!("Provided epub library path does not exist.");
        return (false, "".to_string())
    }

    (true, epub_dir_path)
}

/// Makes a backup of the epub library DB before startup.
fn backup_library_db(lib_db_file: &str) {
    if Path::new(&lib_db_file).exists() {
        let bak_dg = lib_db_file.to_owned() + ".bak";
        fs::copy(lib_db_file, Path::new(bak_dg.as_str()))
            .expect("Unable to create DB backup.");
    }
}

fn main() {
    // Config file paths
    let config_path = dirs::config_dir().unwrap().join("telm");
    let config_file = config_path.join("telmrc");
    let lib_db_file = config_path.join("epub_db.txt");

    // Verify config file
    let verified = startup_verifications(config_path.as_path(), config_file.as_path());
    if !verified.0 {
        return;
    }
    let epub_dir_path = verified.1;

    let lib_db_file_str = lib_db_file.as_path().to_str().unwrap();
    backup_library_db(lib_db_file_str);

    // Load library, save to DB file and pipe to stdout
    let lib = parse_lib::load_library(lib_db_file_str, epub_dir_path.as_str());
    let lib_str = parse_lib::save_library(&lib, lib_db_file_str);
    println!("{}", lib_str);
}