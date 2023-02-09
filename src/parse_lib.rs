use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Write};
use chrono::{Datelike, Utc};
use epub::doc::EpubDoc;
use glob::glob;

pub struct Book {
    pub timestamp: u32,
    pub title: String,
    pub author: String,
    pub series: String,
    pub pages: u32,
    pub path: String
}

pub enum SortBy {
    Date,
    Title,
    Author,
    Pages,
    Series
}

const DELIM: &str = "  /";          // Column delimiter, double space to differentiate from path slashes
const CHARS_PER_PAGE: usize = 2000; // Chars per page for counting pages


// LOAD LIBRARY //

/// Returns library data structure after joining saved DB and epub file list.
pub fn load_library(lib_db_path: &str, epub_path: &str, sort_by: SortBy, reverse: bool) -> Vec<Book> {
    let epub_list = find_epub_files(epub_path);
    let mut library = read_library_db(lib_db_path);

    // Check whether new epub files are available and add them to the library accordingly
    for epub_path in epub_list.iter() {
        if !library.iter().any(|b| &b.path == epub_path) {
            library.push(create_book_from_epub(epub_path));
        }
    }

    // Remove unavailable epub files from library DB
    library.retain(|e| epub_list.contains(&e.path));

    sort_library(&mut library, sort_by, reverse);
    library
}

// Returns a vector with the paths to all epub files in provided root path.
fn find_epub_files(root_path: &str) -> Vec<String>{
    let glob_pattern = root_path.to_owned() + "/**/*.epub";
    glob(&glob_pattern).unwrap()
        .map(|x| x.unwrap().display().to_string())
        .collect()
}

/// Reads tabulated input and returns vector with respective Book data structure.
fn read_library_db(lib_file_path: &str) -> Vec<Book>{
    let mut library_db = Vec::new();
    if Path::new(lib_file_path).exists() {
        // Read lines from text file containing the library data
        let file = File::open(lib_file_path).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        lines.next(); // Skip header line.

        // Load library based on delimiter, ignoring empty or commented lines
        lines
            .filter(|l| !(l.as_ref().unwrap().is_empty() || l.as_ref().unwrap().starts_with("#")))
            .map(|l| library_db.push(line_to_book(l.unwrap())))
            .count();
    }
    library_db
}

/// Returns a Book struct from the line string and based on provided column lengths.
fn line_to_book(line: String) -> Book {
    let fields: Vec<&str> = line
        .split(DELIM)
        .map(|s| s.trim())
        .collect();

    Book {
        timestamp: fields[0].parse().unwrap(),
        title: fields[1].to_string(),
        author: fields[2].to_string(),
        pages: fields[3].parse().unwrap(),
        series: fields[4].to_string(),
        path: fields[5].to_string()
    }
}

// Returns Book data structure from data of the provided epub file path.
fn create_book_from_epub(epub_path: &str) -> Book {
    let mut epub_doc = EpubDoc::new(epub_path).unwrap();
    Book {
        timestamp: create_timestamp(),
        title: epub_doc.mdata("title").unwrap_or("Unknown title".to_string()),
        author: epub_doc.mdata("creator").unwrap_or("Unknown author".to_string()),
        series: String::new(),
        pages: count_epub_pages(&mut epub_doc),
        path: epub_path.to_string()
    }
}

// Returns page count in provided epub file based on CHARS_PER_PAGE constant.
fn count_epub_pages(epub_doc: &mut EpubDoc<BufReader<File>>) -> u32 {
    let mut spine = epub_doc.spine.clone();
    let mut char_count = 0;
    for res_id in spine.iter_mut() {
        char_count += (*epub_doc).get_resource_str(res_id).unwrap_or_default().0.chars()
            .filter(|s| *s!='\n')
            .count();
    }
    (char_count / CHARS_PER_PAGE) as u32
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

/// Sorts library based on provided Book field.
fn sort_library(library: &mut Vec<Book>, sort_by: SortBy, reverse: bool) {
    match sort_by {
        SortBy::Date => library.sort_by_key(|b| b.timestamp),
        SortBy::Title => library.sort_by(|b1, b2| b1.title.cmp(&b2.title)),
        SortBy::Author => library.sort_by(|b1, b2| b1.author.cmp(&b2.author)),
        SortBy::Pages => library.sort_by_key(|b| b.pages),
        SortBy::Series => library.sort_by(|b1, b2| b1.series.cmp(&b2.series))
    }
    if reverse {
        library.reverse();
    }
}


// WRITE LIBRARY //

/// Write library data structure to stdout and to file with appropriate formatting.
pub fn write_library(library: &Vec<Book>, output_path: &str, no_save: bool) {
    let lib_str = library_to_string(library);
    if !no_save {
        let mut output_file = File::create(output_path).unwrap();
        write!(output_file, "{}", lib_str).expect("Unable to write library to file.");
    }
    println!("{}", lib_str);
}

/// Returns a tabulated string from library data structure.
fn library_to_string(library: &Vec<Book>) -> String {
    let mut lib_str = String::new();

    let col_lens = get_max_column_sizes(&library);

    // Create and add header line
    let col_text = [&"DATE".to_string(), &"TITLE".to_string(), &"AUTHOR".to_string(),
        &"PG".to_string(), &"SERIES".to_string(), &"PATH".to_string()];
    lib_str.push_str(tabulate_string(&col_text, &col_lens).as_str());

    // Create and add a tabulated string from each book in the library
    library.iter()
        .map(|b| lib_str.push_str(book_to_line(b, col_lens).as_str()))
        .count();

    lib_str.trim_end().to_string()
}

/// Returns a tabulated string slice from provided Book struct.
fn book_to_line(book: &Book, col_lens: [usize; 6]) -> String {
    let col_text = [&book.timestamp.to_string(), &book.title, &book.author,
        &book.pages.to_string(), &book.series.to_string(), &book.path];
    tabulate_string(&col_text, &col_lens)
}

/// Iterates through each book field and returns its contents as a tabulated string slice.
fn tabulate_string(col_text: &[&String], col_lens: &[usize]) -> String {
    let mut tab_str = String::new();
    col_text.iter().enumerate()
        .map(|i| tab_str.push_str(adjust_string_len(&col_text[i.0], col_lens[i.0]).as_str()))
        .count();
    tab_str.push('\n');
    tab_str
}

/// Returns each book field with required spacing to tabulate the content.
fn adjust_string_len(field: &String, max_len: usize) -> String {
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
fn get_max_column_sizes(library: &Vec<Book>) -> [usize; 6] {
    [
        6,
        library.iter().map(|b| b.title.len()).max().unwrap(),
        library.iter().map(|b| b.author.len()).max().unwrap(),
        library.iter().map(|b| b.pages.to_string().len()).max().unwrap(),
        library.iter().map(|b| b.series.len()).max().unwrap(),
        library.iter().map(|b| b.path.len()).max().unwrap()
    ]
}