use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub struct Book {
    title: String,
    author: String,
    series: String,
    series_index: f32,
    pages: i32,
    //path: String,
}

impl Book {
    pub fn series_string(&self) -> String {
        if self.series == "None" {
            "".to_string()
        } else {
            format!("{} {}", self.series, self.series_index)
        }
    }
}

// LOAD LIBRARY //

/// Read tabulated input and return vector with respective Book data structure.
pub fn load_library(lib_file_path: &str) -> Vec<Book>{
    // Read lines from text file containing the library data
    let file = File::open(lib_file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Get column lengths from header line
    let header_line = lines.next().unwrap().unwrap();
    let col_lens = get_column_sizes_from_file(header_line);

    // Load library based on column lengths
    let mut library = Vec::new();
    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        library.push(Book {
            title: (&line[col_lens[0]..col_lens[1]]).trim().to_string(),
            author: (&line[col_lens[1]..col_lens[2]]).trim().to_string(),
            pages: (&line[col_lens[2]..col_lens[3]].trim()).parse().unwrap(),
            series: (&line[col_lens[3]..col_lens[4]]).trim().to_string(),
            series_index: (&line[col_lens[4]..].trim()).parse().unwrap(),
        });
    }
    library
}

/// Get each column width from the input file.
fn get_column_sizes_from_file(line: String) -> Vec<usize> {
    let mut col_lens = Vec::new();
    let mut pos: i32 = -1;
    let mut count = 0;
    let mut space_start = false;
    for c in line.chars() {
        pos += 1;
        if c == ' ' {
            space_start = true;
        } else if space_start && c != ' ' {
            space_start = false;
            count = count + pos as usize;
            col_lens.push(count);
            pos = 0;
        }
    }
    col_lens
}

// SAVE LIBRARY //

/// Write library data structure to file with appropriate formatting.
pub fn save_library(library: Vec<Book>, output_path: &str) {
    let lib_str = library_to_string(library);
    let mut output_file = File::create(output_path).unwrap();
    write!(output_file, "{}", lib_str).expect("Unable to write library to file.");
}

/// Generate a tabulated string from library data structure.
pub fn library_to_string(library: Vec<Book>) -> String {
    let mut lib_str = String::new();

    let col_lens = get_column_sizes_from_library(&library);

    // Create and add header line
    let col_text = [&"TITLE".to_string(), &"AUTHOR".to_string(), &"PG".to_string(), &"SERIES".to_string()];
    lib_str.push_str(tabulate_string(&col_text, &col_lens).as_str());

    // Create and add a tabulated string from each book in the library
    for book in library.iter() {
        let col_text = [&book.title, &book.author, &book.pages.to_string(), &book.series_string()];
        lib_str.push_str(tabulate_string(&col_text, &col_lens).as_str());
    }
    lib_str.trim_end().to_string()
}

// Iterate through each book field and return its contents as a tabulated string.
fn tabulate_string(col_text: &[&String], col_lens: &[usize]) -> String {
    let mut tab_str = String::new();
    for (i, _col) in col_text.iter().enumerate() {
        let adj_col = adjust_string_len(&col_text[i], col_lens[i]);
        tab_str.push_str(adj_col.as_str());
    }
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
    adj_str.push_str("  ");
    adj_str
}

/// Get each column width from the library data structure.
fn get_column_sizes_from_library(library: &Vec<Book>) -> [usize; 4] {
    [
        library.iter().map(|b| b.title.len()).max().unwrap(),
        library.iter().map(|b| b.author.len()).max().unwrap(),
        library.iter().map(|b| b.pages.to_string().len()).max().unwrap(),
        library.iter().map(|b| b.series_string().len()).max().unwrap(),
    ]
}