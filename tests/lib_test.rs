#[cfg(test)]
mod tests {
    use parse_markdown_rs::parse_markdown;
    use serde::Deserialize;
    use serde_yaml::{from_str, Value};
    use std::collections::HashMap;

    #[test]
    fn parse_yaml_to_struct() {
        #[derive(Deserialize)]
        struct YamlStruct {
            titles: Option<String>,
            title: Option<String>,
            events: HashMap<String, i32>,
        }

        let markdown_input = "---\r\ntitle: hello\r\nevents:\r\n  bla: 213\r\n  bli: 213\r\n---\r\n\r\n# Hello world\r\n\r\nThe title is {title}.\r\n\r\nParagraph\r\n\r\n<h2><span id=\"title\">hello</span></h2>\"\r\n";
        let (_md, yaml_str) = parse_markdown(&markdown_input).unwrap();

        let yaml: YamlStruct = from_str(&yaml_str).unwrap();

        assert_eq!(yaml.title, Some("hello".to_string()));
        assert_eq!(yaml.titles, None);
        assert_eq!(*yaml.events.get("bla").unwrap(), 213);
    }

    #[test]
    fn parse_markdown_with_yaml() {
        let markdown_input = "---\r\ntitle: hello\r\nevents:\r\n  bla: 213\r\n  bli: 213\r\n---\r\n\r\n# Hello world\r\n\r\nThe title is {title}.\r\n\r\nParagraph\r\n\r\n<h2><span id=\"title\">hello</span></h2>\"\r\n";
        let (md, yaml_str) = parse_markdown(&markdown_input).unwrap();

        let yaml: HashMap<String, Value> = from_str(&yaml_str).unwrap();

        let title = yaml["title"].as_str().map_or("title", |v| v);

        assert_eq!(md.contains("<h1>Hello world</h1>"), true);
        assert_eq!(title, "hello");
    }

    #[test]
    fn parse_markdown_without_yaml() {
        let markdown_input = "# Hello world\r\n\r\nThe title is {title}.\r\n\r\nParagraph\r\n\r\n<h2><span id=\"title\">hello</span></h2>\"\r\n";
        let (md, yaml_str) = parse_markdown(&markdown_input).unwrap();

        let yaml: HashMap<String, Value> = from_str(&yaml_str).unwrap();

        let mut title = "";
        if !yaml.is_empty() {
            title = yaml["title"].as_str().map_or("title", |v| v);
        }

        assert_eq!(md.contains("<h1>Hello world</h1>"), true);
        assert_eq!(title, "");
        assert_eq!(yaml.is_empty(), true);
    }

    #[test]
    fn parse_yaml_only() {
        let markdown_input = "---\r\ntitle: hello\r\nevents:\r\n  bla: 213\r\n  bli: 213\r\n---";
        let (md, _) = parse_markdown(&markdown_input).unwrap();
        assert_eq!(md, "");
    }

    #[test]
    fn wrong_yaml() {
        let markdown_input = "---\r\ntitle- hello\r\nevents:\r\n  bla: 213\r\n  bli: 213\r\n---";
        let (_md, yaml_str) = parse_markdown(&markdown_input).unwrap();

        let mut bli = Default::default();
        if let Ok(bla) = from_str::<HashMap<String, Value>>(&yaml_str) {
            bli = bla
        };
        assert_eq!("{}", format!("{:#?}", bli));

        let res = match from_str::<HashMap<String, Value>>(&yaml_str) {
            Ok(_map) => "success",
            Err(_e) => "failed",
        };

        assert_eq!(res, "failed");
    }
}
