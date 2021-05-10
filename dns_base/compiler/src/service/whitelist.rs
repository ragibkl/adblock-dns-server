use crate::configuration::{AppConfig, WhitelistFormat};
use crate::service::loader::{build_path, load_content};
use crate::service::parser::{parse_cnames, parse_domains, parse_hosts, parse_zones};

use std::collections::HashSet;
use std::iter::FromIterator;

fn parse(format: &WhitelistFormat, content: &str) -> Vec<String> {
    match format {
        WhitelistFormat::hosts => parse_hosts(content),
        WhitelistFormat::domains => parse_domains(content),
        WhitelistFormat::cname => parse_cnames(content),
        WhitelistFormat::zone => parse_zones(content),
    }
}

pub async fn extract_whitelist(config: AppConfig) -> Vec<String> {
    let mut handles = Vec::new();
    for source in &config.whitelist {
        let path = build_path(&config.config_dir, &source.path);
        let format = source.format.clone();

        let task = tokio::spawn(async move {
            let content = load_content(&path).await.unwrap_or_default();
            parse(&format, content.as_str())
        });

        handles.push(task);
    }

    let mut results = Vec::new();
    for handle in handles {
        let s = handle.await.unwrap();
        results.extend(s);
    }

    results
}

pub fn filter_whitelist(blacklists: &Vec<String>, whitelists: &Vec<String>) -> Vec<String> {
    let blacklist_set: HashSet<String> = HashSet::from_iter(blacklists.iter().cloned());
    let whitelist_set: HashSet<String> = HashSet::from_iter(whitelists.iter().cloned());

    let whitelist_regex: Vec<String> = whitelists
        .iter()
        .filter(|x| x.starts_with("*."))
        .map(|x| x.replace("*.", ""))
        .collect();

    blacklist_set
        .difference(&whitelist_set)
        .map(|s| s.clone())
        .filter(|s| {
            for w in whitelist_regex.iter() {
                if s.ends_with(w) {
                    return false;
                }
            }
            true
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_empty_whitelist() {
        let blacklist = vec!["zedo.com", "doubleclick.net"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let whitelist = Vec::new();

        let output = filter_whitelist(&blacklist, &whitelist);

        let expected: Vec<_> = vec!["zedo.com", "doubleclick.net"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(
            output.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn it_works_with_domain_whitelist() {
        let blacklist = vec!["zedo.com", "doubleclick.net", "bit.ly", "awstrack.me"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let whitelist = vec!["bit.ly", "awstrack.me"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let output = filter_whitelist(&blacklist, &whitelist);

        let expected: Vec<_> = vec!["zedo.com", "doubleclick.net"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(
            output.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn it_works_with_regex_whitelist() {
        let blacklist = vec![
            "zedo.com",
            "doubleclick.net",
            "bit.ly",
            "awstrack.me",
            "cs531.wpc.edgecastcdn.net",
            "scontent-a-lhr.xx.fbcdn.net",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let whitelist = vec![
            "bit.ly",
            "awstrack.me",
            "*.wpc.edgecastcdn.net",
            "*.xx.fbcdn.net",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let output = filter_whitelist(&blacklist, &whitelist);

        let expected: Vec<_> = vec!["zedo.com", "doubleclick.net"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(
            output.into_iter().collect::<HashSet<_>>(),
            expected.into_iter().collect::<HashSet<_>>()
        );
    }
}
