mod parse_lib;
mod epub_mgmt;

use crate::parse_lib::Book;

// Returns library data structure after joining saved DB and epub file list.
fn load_library(lib_db_path: &str, epub_files_path: &str) -> Vec<Book> {
    // Load library DB
    let mut library = parse_lib::load_library(lib_db_path);

    // Load epub list
    let epub_list = epub_mgmt::find_epub_files(epub_files_path);

    // Check whether new epub files are available and add to the library accordingly
    for epub_path in epub_list.iter() {
        if !library.iter().any(|b| &b.path == epub_path) {
            library.push(epub_mgmt::create_book(epub_path));
        }
    }

    // Remove unavailable epub files from library DB
    library.retain(|e| epub_list.contains(&e.path));

    library
}

fn main() {
    let input_file = "caldb_in.txt";
    //let output_file = "caldb_out.txt";

    let lib_path = "/path/to/epubs";

    let lib = load_library(input_file, lib_path);

    // parse_lib::save_library(&lib, output_file);
    println!("{}", parse_lib::library_to_string(&lib));
}