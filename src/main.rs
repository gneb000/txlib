mod parse_lib;

fn main() {
    let input_file = "caldb_in.txt";
    let output_file = "caldb_out.txt";

    let lib = parse_lib::load_library(input_file);

    parse_lib::save_library(&lib, output_file);
    println!("{}", parse_lib::library_to_string(&lib));
}
