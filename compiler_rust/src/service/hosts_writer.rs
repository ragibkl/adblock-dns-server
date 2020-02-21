use std::fs::File;
use std::io::prelude::*;

pub fn build_content(domains: Vec<String>) -> String {
    let mut lines: Vec<String> = vec![
        "$TTL 1H",
        "@               SOA     LOCALHOST. named-mgr.example.com (1 1h 15m 30d 2h)",
        "                NS      LOCALHOST.",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let domain_lines = domains
        .iter()
        .map(|s| format!("{} CNAME null.null-zone.null.", s));

    lines.extend(domain_lines);
    lines.join("\n")
}

pub fn write_to_file(path: &str, content: &str) {
    let mut f = File::create(path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    f.sync_all().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![
            "www.example.com",
            "abc.example.com",
            "xn--bcher-kva.example.com",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let output = build_content(input);
        let expected = "
$TTL 1H
@               SOA     LOCALHOST. named-mgr.example.com (1 1h 15m 30d 2h)
                NS      LOCALHOST.
www.example.com CNAME null.null-zone.null.
abc.example.com CNAME null.null-zone.null.
xn--bcher-kva.example.com CNAME null.null-zone.null."
            .trim()
            .to_string();

        assert_eq!(output, expected);
    }
}
