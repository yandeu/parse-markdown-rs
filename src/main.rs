use std::fs::read_to_string;

mod lib;

fn main() {
    let markdown_input = read_to_string("./tests/file.md").expect("read file from disk");

    let (mut md, yaml) = lib::parse_markdown(&markdown_input).unwrap();
    md = md.replace("{title}", "TITLE");

    println!("MD: {:#?}", md);

    if !yaml.is_empty() {
        println!("YAML: {:#?}", yaml);
    }
}
