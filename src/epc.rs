use std::env;
use std::path::Path;
use epub::doc::EpubDoc;
use glob::glob;

const CHARS_PER_PAGE: usize = 2000;

fn count_epub_pages(epub_file: &str) -> usize {
    let mut doc = EpubDoc::new(epub_file).unwrap();
    let mut spine = doc.spine.clone();

    let mut char_count = 0;
    for res_id in spine.iter_mut() {
        char_count += doc.get_resource_str(res_id).unwrap().0.chars().filter(|s| *s!='\n').count();
    }
    char_count / CHARS_PER_PAGE
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args[1].trim();

    let epub_list: Vec<String>;
    if Path::new(input_path).is_dir() {
        let glob_pattern = input_path.to_owned() + "/**/*.epub";
        epub_list = glob(&glob_pattern).unwrap()
            .map(|x| x.unwrap().display().to_string())
            .collect();
    } else {
        epub_list = vec!(input_path.to_string());
    }

    for item in epub_list.iter() {
        println!(
            "{} {}",
            count_epub_pages(item),
            Path::new(item).file_name().unwrap().to_string_lossy().replace(".epub", ""));
    }
}
