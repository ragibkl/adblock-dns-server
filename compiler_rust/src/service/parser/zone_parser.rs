extern crate addr;
extern crate regex;

use addr::DomainName;
use regex::Regex;

use super::parser_utils::clean_text;
use crate::service::core::Parser;

fn extract_domain(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<domain>.{2,256}\.[a-z]{2,6})\s+(CNAME|cname)\s+(.{2,256}\.[a-z]{2,6})\."
        )
        .unwrap();
    }

    RE.captures(&text)
        .and_then(|cap| cap.name("domain"))
        .and_then(|d| d.as_str().parse::<DomainName>().ok())
        .map(|d| d.as_str().trim().to_string())
}

pub struct ZoneParser;

impl ZoneParser {
    pub fn new() -> ZoneParser {
        ZoneParser {}
    }
}

impl Parser for ZoneParser {
    fn parse(&self, content: &str) -> Vec<String> {
        let lines = content
            .lines()
            .map(clean_text)
            .map(|l| extract_domain(&l))
            .filter(|l| l.is_some())
            .map(|l| l.unwrap())
            .collect::<Vec<_>>();

        println!("[HostParser] - Done parsing {} domains", lines.len());
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_domain() {
        let input = "www.bing.com    CNAME   strict.bing.com.";
        let output = extract_domain(input);
        let expected = "www.bing.com".to_string();

        assert_eq!(output, Some(expected));
    }

    #[test]
    fn it_works() {
        let parser = ZoneParser::new();
        let input = "
            www.bing.com    cname   strict.bing.com.

            duckduckgo.com      CNAME   safe.duckduckgo.com.
            www.duckduckgo.com  CNAME   safe.duckduckgo.com.

            google.com.my    CNAME   forcesafesearch.google.com.
            www.google.com.my    CNAME   forcesafesearch.google.com.
        ";
        let output = parser.parse(input);
        let expected = vec![
            "www.bing.com".to_string(),
            "duckduckgo.com".to_string(),
            "www.duckduckgo.com".to_string(),
            "google.com.my".to_string(),
            "www.google.com.my".to_string(),
        ];
        assert_eq!(output, expected);
    }
}
