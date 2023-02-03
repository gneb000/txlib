use std::fs::File;
use std::io::{BufRead, BufReader};

struct Book {
    title: String,
    author: String,
    series: String,
    series_index: f32,
    pages: i32,
    //path: String,
}

impl Book {
    pub fn to_string(&self) -> String {
        let s = if self.series == "None" {
            "".to_string()
        } else {
            format!("{} {}", self.series, self.series_index)
        };
        format!("{} <> {} <> {} <> {}", self.title, self.author, self.pages, s)
    }
}

fn load_library(lib_file_path: &str) -> Vec<Book>{
    // Read lines from text file containing the library data
    let file = File::open(lib_file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Get column lengths from header line
    let header_line = lines.next().unwrap().unwrap();
    let col_lens = get_col_lens(header_line);

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

fn get_col_lens(line: String) -> Vec<usize> {
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

fn main() {
    let filename = "caldb.txt";

    let lib = load_library(filename);

    for book in lib.iter() {
        println!("{}", book.to_string());
    }
}