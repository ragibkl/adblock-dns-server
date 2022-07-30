extern crate addr;
extern crate regex;

use addr::DomainName;
use regex::Regex;

use super::common::parse;

fn extract(text: &str) -> Option<String> {
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

pub fn parse_zones(content: &str) -> Vec<String> {
    parse(content, extract)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_domain() {
        let input = "www.bing.com    CNAME   strict.bing.com.";
        let output = extract(input);
        let expected = "www.bing.com".to_string();

        assert_eq!(output, Some(expected));
    }

    #[test]
    fn it_works() {
        let input = "
            www.bing.com    cname   strict.bing.com.

            duckduckgo.com      CNAME   safe.duckduckgo.com.
            www.duckduckgo.com  CNAME   safe.duckduckgo.com.

            google.com.my    CNAME   forcesafesearch.google.com.
            www.google.com.my    CNAME   forcesafesearch.google.com.
        ";
        let output = parse_zones(input);
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
