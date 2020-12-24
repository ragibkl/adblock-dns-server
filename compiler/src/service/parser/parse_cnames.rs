extern crate addr;
extern crate regex;

use addr::DomainName;
use regex::Regex;

use super::common::parse;

fn extract_hosts(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<domain>.{2,200}\.[a-z]{2,6})\s+(CNAME|cname)\s+(?P<alias>.{2,200}\.[a-z]{2,6})\."
        )
        .unwrap();
    }

    RE.captures(&text)
        .and_then(|cap| {
            let domain = cap.name("domain");
            let alias = cap.name("alias");

            match (domain, alias) {
                (Some(d), Some(a)) => Some((d, a)),
                _ => None,
            }
        })
        .and_then(|(d, a)| {
            let domain = d.as_str().parse::<DomainName>().ok();
            let alias = a.as_str().parse::<DomainName>().ok();
            match (domain, alias) {
                (Some(d), Some(a)) => Some(format!("{} CNAME {}.", d, a)),
                _ => None,
            }
        })
        .map(|d| d.as_str().trim().to_string())
}

pub fn parse_cnames(content: &str) -> Vec<String> {
    let lines = parse(content, extract_hosts);
    println!("[parse_hosts] - Done parsing {} domains", lines.len());
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_domain() {
        let input = "www.bing.com    CNAME   strict.bing.com.";
        let output = extract_hosts(input);
        let expected = "www.bing.com CNAME strict.bing.com.".to_string();

        assert_eq!(output, Some(expected));
    }

    #[test]
    fn it_works() {
        // let parser = CnameParser::new();
        let input = "
            www.bing.com    cname   strict.bing.com.

            duckduckgo.com      CNAME   safe.duckduckgo.com.
            www.duckduckgo.com  CNAME   safe.duckduckgo.com.

            google.com.my    CNAME   forcesafesearch.google.com.
            www.google.com.my    CNAME   forcesafesearch.google.com.
        ";
        let output = parse_cnames(input);
        let expected = vec![
            "www.bing.com CNAME strict.bing.com.".to_string(),
            "duckduckgo.com CNAME safe.duckduckgo.com.".to_string(),
            "www.duckduckgo.com CNAME safe.duckduckgo.com.".to_string(),
            "google.com.my CNAME forcesafesearch.google.com.".to_string(),
            "www.google.com.my CNAME forcesafesearch.google.com.".to_string(),
        ];
        assert_eq!(output, expected);
    }
}
