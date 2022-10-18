# Struct Helpers

[![dependency status](https://deps.rs/repo/github/yandeu/parse-markdown-rs/status.svg)](https://deps.rs/repo/github/yandeu/parse-markdown-rs)
[![CI](https://github.com/yandeu/parse-markdown-rs/actions/workflows/main.yml/badge.svg)](https://github.com/yandeu/parse-markdown-rs/actions/workflows/main.yml)

## Install

```toml
[dependencies]
parse_markdown_rs = { git = "https://github.com/yandeu/parse-markdown-rs" }
```

## Example

```rust
use serde::Deserialize;
use serde_yaml::from_str;
use std::{collections::HashMap, fs::read_to_string};
use parse_markdown_rs::parse_markdown;

#[derive(Deserialize)]
struct YamlStruct {
    title: Option<String>,
    events: HashMap<String, i32>,
}

fn main() {
    let markdown_input = read_to_string("./tests/file.md").expect("read file from disk");

    let (md, yaml_str) = parse_markdown(&markdown_input).unwrap();

    // markdown
    assert!(md.starts_with("<h1>Hello world</h1>"));

    // yaml
    let yaml: YamlStruct = from_str(yaml_str).unwrap();
    assert_eq!(yaml.title, Some("hello".to_string()));
    assert_eq!(yaml.events.get("bla").unwrap().to_owned(), 213);
}
```
