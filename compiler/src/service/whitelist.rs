use crate::configuration::AppConfig;
use crate::service::loader::load_content;
use crate::service::parser::{parse_cnames, parse_domains, parse_hosts, parse_zones};

use std::collections::HashSet;
use std::iter::FromIterator;

fn parse(format: &str, content: &str) -> Vec<String> {
    match format {
        "hosts" => parse_hosts(content),
        "domains" => parse_domains(content),
        "cname" => parse_cnames(content),
        "zone" => parse_zones(content),
        _ => panic!("invalid format"),
    }
}

pub async fn extract_whitelist(config: AppConfig) -> Vec<String> {
    let mut handles = Vec::new();
    for source in &config.blacklist {
        let path = source.path.clone();
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
    let whitelist_set: HashSet<String> = HashSet::from_iter(blacklists.iter().cloned());

    let whitelist_regex: Vec<String> = whitelists
        .iter()
        .filter(|x| x.starts_with("*."))
        .map(|x| x.replace("*.", ""))
        .collect();

    let reduced: Vec<String> = blacklist_set
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
        .collect();

    return reduced;
    // let new_list = blacklists
    //     .iter()
    //     .filter(|x| {
    //         for white in whitelists {
    //             if white.starts_with("*.") {
    //                 let pattern = white.replace("*.", "");
    //                 if x.ends_with(&pattern) {
    //                     return false;
    //                 }
    //             } else {
    //                 if x == &white {
    //                     return false;
    //                 }
    //             }
    //         }
    //         true
    //     })
    //     .map(|s| s.clone())
    //     .collect();

    // return new_list;
}
