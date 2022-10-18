// #![allow(dead_code, unused_variables, unused_imports)]

// test code in README file
#[doc = include_str!("../README.md")]
// imports
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::error::Error;

pub fn parse_markdown(markdown_input: &str) -> Result<(String, &str), Box<dyn Error>> {
    // https://rustexp.lpil.uk/
    let re = Regex::new(r"(?ms)((?:\-\-\-)(.*?)(?:\-\-\-))?(.*)")?;

    // separate yaml and md
    let caps = re.captures(&markdown_input).unwrap();
    let yaml = caps.get(2).map_or("", |m| m.as_str());
    let md = caps.get(3).map_or("", |m| m.as_str());

    // let mut deserialized_yaml: Option<HashMap<String, Value>> = None;
    // if yaml.len() > 0 {
    //     deserialized_yaml = serde_yaml::from_str(&yaml)?;
    // }

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&md, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok((html_output, yaml))
}
