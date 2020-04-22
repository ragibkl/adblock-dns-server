extern crate addr;
extern crate regex;

use addr::DomainName;
use regex::Regex;

use super::parser_utils::clean_text;
use crate::service::core::Parser;

fn extract_domain(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<domain>.{2,200}\.[a-z]{2,6})").unwrap();
    }

    RE.captures(&text)
        .and_then(|cap| cap.name("domain"))
        .and_then(|d| d.as_str().parse::<DomainName>().ok())
        .map(|d| d.as_str().trim().to_string())
}

pub struct ListParser;

impl ListParser {
    pub fn new() -> ListParser {
        ListParser {}
    }
}

impl Parser for ListParser {
    fn parse(&self, content: &str) -> Vec<String> {
        let lines = content
            .lines()
            .map(clean_text)
            .map(|l| extract_domain(&l))
            .filter(|l| l.is_some())
            .map(|l| l.unwrap())
            .collect::<Vec<_>>();

        println!("[ListParser] Done parsing {} domains", lines.len());
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_domain() {
        let input = "abc.example.com";
        let output = extract_domain(input);
        let expected = "abc.example.com".to_string();
        assert_eq!(output, Some(expected));

        let input = "Bücher.example.com";
        let output = extract_domain(input);
        let expected = "xn--bcher-kva.example.com".to_string();
        assert_eq!(output, Some(expected));

        let input = "";
        let output = extract_domain(input);
        assert_eq!(output, None);
    }

    #[test]
    fn it_works() {
        let parser = ListParser::new();
        let input = "
            # This is a comment
            abc.example.com # this should work
            def.example.com\r

            ghi.example.com\r
        ";

        let output = parser.parse(input);

        let expected = vec![
            "abc.example.com".to_string(),
            "def.example.com".to_string(),
            "ghi.example.com".to_string(),
        ];
        assert_eq!(output, expected);
    }
}
