extern crate addr;
extern crate regex;

use addr::DomainName;
use regex::Regex;

use super::common::parse;

fn extract(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(127\.0\.0\.1|0\.0\.0\.0)\s+(?P<domain>.{2,200}\.[a-z]{2,6})").unwrap();
    }

    RE.captures(&text)
        .and_then(|cap| cap.name("domain"))
        .and_then(|d| d.as_str().parse::<DomainName>().ok())
        .map(|d| d.as_str().trim().to_string())
}

pub fn parse_hosts(content: &str) -> Vec<String> {
    let lines = parse(content, extract);
    // println!("[parse_hosts] - Done parsing {} domains", lines.len());
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_domain() {
        let input = "127.0.0.1 abc.example.com";
        let output = extract(input);
        let expected = "abc.example.com".to_string();

        assert_eq!(output, Some(expected));
    }

    #[test]
    fn it_works() {
        let input = "
            127.0.0.1  abc.example.com
            0.0.0.0  abc.example.com\r
            127.0.0.1 abc.example.com
            0.0.0.0 abc.example.com\r
        ";

        let output = parse_hosts(input);

        let expected = vec![
            "abc.example.com".to_string(),
            "abc.example.com".to_string(),
            "abc.example.com".to_string(),
            "abc.example.com".to_string(),
        ];
        assert_eq!(output, expected);
    }
}
