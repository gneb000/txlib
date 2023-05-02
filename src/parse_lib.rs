use chrono::{Datelike, Utc};
use epub::doc::EpubDoc;
use glob::glob;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const DELIM: &str = "  /";          // Column delimiter, double space to differentiate from path slashes
const READ_SYMBOL: &str = "*";      // Symbol to mark book as read
const CHARS_PER_PAGE: usize = 2000; // Chars per page for counting pages

pub struct Book {
    timestamp: u32,
    read: bool,
    title: String,
    author: String,
    series: String,
    pages: usize,
    path: String,
}

impl Book {
    fn read_symbol(&self) -> String {
        if self.read {
            READ_SYMBOL.to_string()
        } else {
            String::from(" ")
        }
    }
}

pub enum SortBy {
    Date,
    Read,
    Title,
    Author,
    Pages,
    Series,
}

// LOAD LIBRARY //

/// Returns library data structure after joining saved DB and epub file list.
pub fn load_library(lib_db_path: &Path, epub_path: &str, sort_by: &SortBy, reverse: bool) -> Result<Vec<Book>, &'static str> {
    let epub_list = find_epub_files(epub_path);
    let mut library = read_library_db(lib_db_path)?;

    // Check whether new epub files are available and add them to the library accordingly
    for epub_path in &epub_list {
        if !library.iter().any(|b| &b.path == epub_path) {
            match create_book_from_epub(epub_path) {
                Some(book) => library.push(book),
                None => {
                    println!("warning: unable to load \"{}\"", epub_path);
                    continue;
                }
            }
        }
    }

    // Remove unavailable epub files from library DB
    library.retain(|e| epub_list.contains(&e.path));

    sort_library(&mut library, sort_by, reverse);
    Ok(library)
}

/// Returns a vector with the paths to all epub files in provided root path.
fn find_epub_files(root_path: &str) -> Vec<String> {
    let glob_pattern = root_path.to_owned() + "/**/*.epub";
    glob(&glob_pattern)
        .unwrap()
        .filter(|p| p.is_ok())
        .map(|p| p.unwrap().display().to_string())
        .collect()
}

/// Reads tabulated input and returns vector with respective Book data structure.
fn read_library_db(lib_file_path: &Path) -> Result<Vec<Book>, &'static str> {
    if !lib_file_path.exists() {
        return Ok(Vec::new());
    }
    let file = match File::open(lib_file_path) {
        Ok(f) => f,
        Err(_) => return Err("error: unable to read library DB"),
    };
    let library = BufReader::new(file)
        .lines()
        .skip(1) // Skip header line
        .filter(|l| l.is_ok() && !(l.as_ref().unwrap().is_empty() || l.as_ref().unwrap().starts_with('#')))
        .map(|l| line_to_book(&l.unwrap()))
        .collect();
    Ok(library)
}

/// Returns a Book struct from the line string and based on provided column lengths.
fn line_to_book(line: &str) -> Book {
    let fields: Vec<&str> = line
        .split(DELIM)
        .map(str::trim)
        .collect();

    Book {
        timestamp: fields[0].parse().unwrap_or(999_999),
        read: !fields[1].trim().is_empty(),
        title: fields[2].to_string(),
        author: fields[3].to_string(),
        pages: fields[4].parse().unwrap_or(0),
        series: fields[5].to_string(),
        path: fields[6].to_string(),
    }
}

/// Returns Book data structure from data of the provided epub file path.
fn create_book_from_epub(epub_path: &str) -> Option<Book> {
    let mut epub_doc = match EpubDoc::new(epub_path) {
        Ok(doc) => doc,
        Err(_) => return None,
    };
    let book = Book {
        timestamp: create_timestamp(),
        read: false,
        title: epub_doc.mdata("title").unwrap_or(String::from("Unknown title")),
        author: epub_doc.mdata("creator").unwrap_or(String::from("Unknown author")),
        series: String::new(),
        pages: count_epub_pages(&mut epub_doc),
        path: epub_path.to_string(),
    };
    Some(book)
}

/// Returns page count in provided epub file based on `CHARS_PER_PAGE` constant.
fn count_epub_pages(epub_doc: &mut EpubDoc<BufReader<File>>) -> usize {
    let char_count = epub_doc.spine.clone().iter().fold(0_usize, |acc, r| {
        acc + epub_doc
            .get_resource_str(r)
            .unwrap_or((String::new(), String::new()))
            .0
            .chars()
            .filter(|s| *s != '\n')
            .count()
    });
    char_count / CHARS_PER_PAGE
}

/// Returns today as a timestamp with YYMMDD format.
fn create_timestamp() -> u32 {
    let now = Utc::now();
    let date_str = format!("{:02}{:02}{:02}", now.year(), now.month(), now.day());
    (date_str[2..]).parse().unwrap_or(999_999)
}

/// Sorts library based on provided Book field.
fn sort_library(library: &mut [Book], sort_by: &SortBy, reverse: bool) {
    match sort_by {
        SortBy::Date => library.sort_unstable_by_key(|b| b.timestamp),
        SortBy::Read => library.sort_unstable_by_key(|b| b.read),
        SortBy::Title => library.sort_unstable_by(|b1, b2| b1.title.cmp(&b2.title)),
        SortBy::Author => library.sort_unstable_by(|b1, b2| b1.author.cmp(&b2.author)),
        SortBy::Pages => library.sort_unstable_by_key(|b| b.pages),
        SortBy::Series => library.sort_unstable_by(|b1, b2| b1.series.cmp(&b2.series)),
    }
    if reverse {
        library.reverse();
    }
}

// WRITE LIBRARY //

/// Write library data structure to stdout and/or to file with appropriate formatting.
pub fn write_library(library: &[Book], output_path: &Path, no_save: bool) -> Result<(), &'static str> {
    let lib_str = library_to_string(library);
    if no_save {
        println!("{lib_str}");
        return Ok(());
    }
    let mut output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(_) => return Err("error: unable to open library DB"),
    };
    if write!(output_file, "{lib_str}").is_err() {
        return Err("error: unable to write library to DB");
    }
    Ok(())
}

/// Returns a tabulated string from library data structure.
fn library_to_string(library: &[Book]) -> String {
    let mut lib_str = String::new();

    let col_lens = get_max_column_sizes(library);

    // Create and add header line
    let col_text = [
        &"DATE".to_string(),
        &"R".to_string(),
        &"TITLE".to_string(),
        &"AUTHOR".to_string(),
        &"PG".to_string(),
        &"SERIES".to_string(),
        &"PATH".to_string(),
    ];
    lib_str.push_str(tabulate_string(&col_text, &col_lens).as_str());

    // Create and add a tabulated string from each book in the library
    lib_str.push_str(library
            .iter()
            .map(|b| book_to_line(b, col_lens))
            .collect::<String>()
            .as_str()
    );

    lib_str.trim_end().to_string()
}

/// Returns a tabulated string slice from provided Book struct.
fn book_to_line(book: &Book, col_lens: [usize; 7]) -> String {
    let col_text = [
        &book.timestamp.to_string(),
        &book.read_symbol(),
        &book.title,
        &book.author,
        &book.pages.to_string(),
        &book.series.to_string(),
        &book.path,
    ];
    tabulate_string(&col_text, &col_lens)
}

/// Iterates through each book field and returns its contents as a tabulated string slice.
fn tabulate_string(col_text: &[&String], col_lens: &[usize]) -> String {
    let mut tab_str = col_text
        .iter()
        .zip(col_lens)
        .map(|(t, l)| adjust_string_len(t, *l))
        .collect::<String>();
    tab_str.push('\n');
    tab_str
}

/// Returns each book field with required spacing to tabulate the content.
fn adjust_string_len(field: &str, max_len: usize) -> String {
    let mut adj_str = String::from(field);
    let mut width = adj_str.chars().collect::<Vec<char>>().len();
    while width < max_len {
        adj_str.push(' ');
        width += 1;
    }
    adj_str.push_str(DELIM);
    adj_str
}

/// Return maximum width of each column from the library data structure.
fn get_max_column_sizes(library: &[Book]) -> [usize; 7] {
    [
        6, // timestamp has 6 digits
        2, // read symbol can have 2 characters
        library.iter().map(|b| b.title.len()).max().unwrap_or(10),
        library.iter().map(|b| b.author.len()).max().unwrap_or(10),
        library.iter().map(|b| b.pages.to_string().len()).max().unwrap_or(4),
        library.iter().map(|b| b.series.len()).max().unwrap_or(10),
        library.iter().map(|b| b.path.len()).max().unwrap_or(20),
    ]
}
