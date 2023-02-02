use std::fs::File;
use std::io::{BufRead, BufReader};

/*struct Book {
    title: String,
    author: String,
    series: String,
    series_index: usize,
    pages: u32,
    path: String,
}*/

fn main() {
    let filename = "caldb.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let header_line = lines.next().unwrap().unwrap();
    let col_lens = get_col_lens(header_line);
}

fn get_col_lens(line: String) -> Vec<i32> {
    let mut col_lens = Vec::new();
    let mut count = -1;
    let mut space_start = false;
    for c in line.chars() {
        count += 1;
        if c == ' ' {
            space_start = true;
        } else if space_start && c != ' ' {
            space_start = false;
            col_lens.push(count);
            count = 0;
        }
    }
    col_lens
}
