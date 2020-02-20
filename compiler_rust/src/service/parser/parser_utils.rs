fn remove_comments(text: String) -> String {
    let string: String = text.chars().take_while(|c| *c != '#').collect();
    string.trim().to_string()
}

fn replace_whitespace(text: String) -> String {
    text.replace("\t", " ").replace("\r", "").trim().to_string()
}

pub fn clean_text(text: String) -> String {
    let text = replace_whitespace(text);
    let text = remove_comments(text);
    text
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
    fn it_replaces_whitespace() {
        let input = "127.0.0.1\tabc.example.com".to_string();
        let output = replace_whitespace(input);
        let expected = "127.0.0.1 abc.example.com".to_string();

        assert_eq!(output, expected);

        let input = "127.0.0.1 abc.example.com ".to_string();
        let output = replace_whitespace(input);
        let expected = "127.0.0.1 abc.example.com".to_string();

        assert_eq!(output, expected);
    }
}
