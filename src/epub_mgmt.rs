use std::fs::File;
use std::io::BufReader;
use chrono::{Datelike, Utc};
use epub::doc::EpubDoc;
use glob::glob;

use crate::parse_lib::Book;

const CHARS_PER_PAGE: usize = 2000;

// Returns a vector with the paths to all epub files in provided root path.
pub fn find_epub_files(root_path: &str) -> Vec<String>{
    let glob_pattern = root_path.to_owned() + "/**/*.epub";
    glob(&glob_pattern).unwrap()
        .map(|x| x.unwrap().display().to_string())
        .collect()
}

// Returns Book data structure from data of the provided epub file path.
pub fn create_book(epub_path: &str) -> Book {
    let epub_info = get_epub_data(epub_path);

    Book {
        timestamp: epub_info.0,
        title: epub_info.1,
        author: epub_info.2,
        series: "".to_string(),
        pages: epub_info.3,
        path: epub_path.to_string()
    }
}

// Returns tuple with (timestamp, title, author, pages) of the provided epub file.
fn get_epub_data(epub_file: &str) -> (u32, String, String, i32) {
    let epub_doc = EpubDoc::new(epub_file).unwrap();
    let timestamp = create_timestamp();
    let title = epub_doc.mdata("title").unwrap_or("Unknown title".to_string());
    let author = epub_doc.mdata("creator").unwrap_or("Unknown author".to_string());
    let pages = count_epub_pages(epub_doc) as i32;
    (timestamp, title, author, pages)
}

// Returns page count in provided epub file based on CHARS_PER_PAGE constant.
fn count_epub_pages(mut epub_doc: EpubDoc<BufReader<File>>) -> usize {
    let mut spine = epub_doc.spine.clone();
    let mut char_count = 0;
    for res_id in spine.iter_mut() {
        char_count += epub_doc.get_resource_str(res_id).unwrap().0.chars().filter(|s| *s!='\n').count();
    }
    char_count / CHARS_PER_PAGE
}

// Returns today as a timestamp with YYMMDD format.
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

