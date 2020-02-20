extern crate addr;
extern crate regex;

use addr::DomainName;
use regex::Regex;

use crate::service::core::Parser;

fn remove_comments(text: String) -> String {
    let string: String = text.chars().take_while(|c| *c != '#').collect();
    string.trim().to_string()
}

fn replace_whitespace(text: String) -> String {
    text.replace("\t", " ").replace("\r", "").trim().to_string()
}

fn clean_text(text: String) -> String {
    let text = replace_whitespace(text);
    let text = remove_comments(text);
    text
}

fn extract_domain(text: String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(127\.0\.0\.1|0\.0\.0\.0)\s+(?P<domain>.{2,256}\.[a-z]{2,6})").unwrap();
    }

    RE.captures(&text)
        .and_then(|cap| cap.name("domain"))
        .and_then(|d| d.as_str().parse::<DomainName>().ok())
        .map(|d| d.as_str().trim().to_string())
}

pub struct HostParser;

impl HostParser {
    pub fn new() -> HostParser {
        HostParser {}
    }
}

impl Parser for HostParser {
    fn parse(&self, content: String) -> Vec<String> {
        let lines = content
            .lines()
            .map(|l| l.to_string())
            .map(|l| clean_text(l))
            .map(|l| extract_domain(l))
            .filter(|l| l.is_some())
            .map(|l| l.unwrap());

        lines.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_remove_comment() {
        let input = "127.0.0.1 abc.example.com # this is an example".to_string();
        let output = remove_comments(input);
        let expected = "127.0.0.1 abc.example.com".to_string();

        assert_eq!(output, expected);
    }

    #[test]
    fn it_replaces_whitespace() {
        let input = "127.0.0.1\tabc.example.com".to_string();
        let output = replace_whitespace(input);
        let expected = "127.0.0.1 abc.example.com".to_string();

        assert_eq!(output, expected);

        let input = "127.0.0.1 abc.example.com ".to_string();
        let output = replace_whitespace(input);
        let expected = "127.0.0.1 abc.example.com".to_string();

        assert_eq!(output, expected);
    }

    #[test]
    fn it_extract_domain() {
        let input = "127.0.0.1 abc.example.com".to_string();
        let output = extract_domain(input);
        let expected = "abc.example.com".to_string();

        assert_eq!(output, Some(expected));
    }

    #[test]
    fn it_works() {
        let parser = HostParser::new();
        let input = "
            127.0.0.1  abc.example.com
            0.0.0.0  abc.example.com\r
            127.0.0.1 abc.example.com
            0.0.0.0 abc.example.com\r
        "
        .to_string();

        let output = parser.parse(input);

        let expected = vec![
            "abc.example.com".to_string(),
            "abc.example.com".to_string(),
            "abc.example.com".to_string(),
            "abc.example.com".to_string(),
        ];
        assert_eq!(output, expected);
    }
}
