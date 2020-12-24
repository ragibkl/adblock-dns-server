fn remove_comments(text: &str) -> String {
    let string: String = text.chars().take_while(|c| *c != '#').collect();
    string.trim().to_string()
}

fn replace_whitespace(text: &str) -> String {
    text.replace("\t", " ").replace("\r", "").trim().to_string()
}

pub fn clean_text(text: &str) -> String {
    let text = replace_whitespace(text);
    let text = remove_comments(&text);
    text
}

pub fn parse<T: Fn(&str) -> Option<String>>(content: &str, extract: T) -> Vec<String> {
    content
        .lines()
        .map(clean_text)
        .map(|l| extract(&l))
        .filter(|l| l.is_some())
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_remove_comment() {
        let input = "127.0.0.1 abc.example.com # this is an example";
        let output = remove_comments(input);
        let expected = "127.0.0.1 abc.example.com".to_string();

        assert_eq!(output, expected);
    }

    #[test]
    fn it_replaces_whitespace() {
        let input = "127.0.0.1\tabc.example.com";
        let output = replace_whitespace(input);
        let expected = "127.0.0.1 abc.example.com".to_string();

        assert_eq!(output, expected);

        let input = "127.0.0.1 abc.example.com ";
        let output = replace_whitespace(input);
        let expected = "127.0.0.1 abc.example.com".to_string();

        assert_eq!(output, expected);
    }
}
