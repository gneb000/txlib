use std::env;
use std::path::Path;
use epub::doc::EpubDoc;
use glob::glob;

const CHARS_PER_PAGE: usize = 2000;

// Returns page count in provided epub file based on CHARS_PER_PAGE constant.
pub fn count_epub_pages(epub_file: &str) -> usize {
    let mut doc = EpubDoc::new(epub_file).unwrap();
    let mut spine = doc.spine.clone();

    let mut char_count = 0;
    for res_id in spine.iter_mut() {
        char_count += doc.get_resource_str(res_id).unwrap().0.chars().filter(|s| *s!='\n').count();
    }
    char_count / CHARS_PER_PAGE
}

// Returns a vector with the paths to all epub files in provided root path.
pub fn find_epub_files(root_path: &str) -> Vec<String>{
    let glob_pattern = root_path.to_owned() + "/**/*.epub";
    glob(&glob_pattern).unwrap()
        .map(|x| x.unwrap().display().to_string())
        .collect()
}