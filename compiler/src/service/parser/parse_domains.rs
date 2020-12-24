extern crate addr;
extern crate regex;

use addr::DomainName;
use regex::Regex;

use super::common::parse;

fn extract_hosts(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<domain>.{2,200}\.[a-z]{2,6})").unwrap();
    }

    RE.captures(&text)
        .and_then(|cap| cap.name("domain"))
        .and_then(|d| d.as_str().parse::<DomainName>().ok())
        .map(|d| d.as_str().trim().to_string())
}

pub fn parse_domains(content: &str) -> Vec<String> {
    let lines = parse(content, extract_hosts);
    println!("[parse_hosts] - Done parsing {} domains", lines.len());
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_domain() {
        let input = "abc.example.com";
        let output = extract_hosts(input);
        let expected = "abc.example.com".to_string();
        assert_eq!(output, Some(expected));

        let input = "Bücher.example.com";
        let output = extract_hosts(input);
        let expected = "xn--bcher-kva.example.com".to_string();
        assert_eq!(output, Some(expected));

        let input = "";
        let output = extract_hosts(input);
        assert_eq!(output, None);
    }

    #[test]
    fn it_works() {
        let input = "
            # This is a comment
            abc.example.com # this should work
            def.example.com\r

            ghi.example.com\r
        ";

        let output = parse_domains(input);

        let expected = vec![
            "abc.example.com".to_string(),
            "def.example.com".to_string(),
            "ghi.example.com".to_string(),
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn it_still_works() {
        let input = "
            Malvertising list by Disconnect
            # License: GPLv3
            # Contact: support [at] disconnect.me

            malware-check.disconnect.me
            101order.com
            123found.com
            140proof.com
        ";

        let output = parse_domains(input);

        let expected = vec![
            "malware-check.disconnect.me".to_string(),
            "101order.com".to_string(),
            "123found.com".to_string(),
            "140proof.com".to_string(),
        ];
        assert_eq!(output, expected);
    }
}
