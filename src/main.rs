mod parse_lib;
mod epc;

use chrono::{Datelike, Utc};

use crate::parse_lib::Book;

// Returns timestamp with YYMMDD format.
fn create_timestamp() -> u32{
    let now = Utc::now();
    let date_str = format!(
        "{:02}{:02}{:02}",
        now.year(),
        now.month(),
        now.day(),
    );
    (&date_str[2..]).parse().unwrap()
}

fn create_book(epub_path: &String) -> Book {
    let page_count = epc::count_epub_pages(epub_path.as_str());

    let path_split: Vec<&str> = epub_path.split("/").collect();
    let binding = path_split[path_split.len() - 1].replace(".epub", "");
    let epub_name: Vec<&str> = binding.split(" - ").collect();

    Book {
        timestamp: create_timestamp(),
        title: epub_name[0].to_string(),
        author: epub_name[1].to_string(),
        series: "".to_string(),
        pages: page_count as i32,
        path: epub_path.to_string()
    }
}

fn main() {
    let input_file = "caldb_in.txt";
    //let output_file = "caldb_out.txt";

    let mut lib = parse_lib::load_library(input_file);
    lib.pop(); // TEST ONLY

    let lib_path = "/home/gaorbe/storage/Almacenamiento/Main/Reading/Ebooks";
    let epub_list = epc::find_epub_files(lib_path);

    for epub_path in epub_list.iter() {
        if !lib.iter().any(|b| &b.path == epub_path) {
            lib.push(create_book(epub_path));
        }
    }

    // parse_lib::save_library(&lib, output_file);
    println!("{}", parse_lib::library_to_string(&lib));
}
