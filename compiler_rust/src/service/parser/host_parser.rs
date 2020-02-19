use crate::service::core::Parser;

fn remove_comments(text: String) -> String {
    let string: String = text.chars().take_while(|c| *c != '#').collect();
    string.trim().to_string()
}

fn replace_whitespace(text: String) -> String {
    text.replace("\t", " ").replace("\r", "").replace("  ", " ").trim()
}

fn clean_text(text: String) -> String {
    let text = replace_whitespace(text);
    let text = remove_comments(text);
    text
}

pub struct HostParser;

impl HostParser {
    fn new() -> HostParser {
        HostParser{}
    }
}

impl Parser for HostParser {
    fn parse(&self, content: String) -> Vec<String> {
        // let lines = content.sp
        vec![]
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
    fn it_works() {
        let parser = HostParser::new();
        let input = "
            127.0.0.1 abc.example.com
        ".to_string();

        let output = parser.parse(input);

        let expected = vec!["abc.example.com".to_string()];
        assert_eq!(output, expected);
    }
}
