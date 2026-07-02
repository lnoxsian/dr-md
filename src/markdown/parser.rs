pub fn extract_wiki_links(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' && chars.peek() == Some(&'[') {
            chars.next(); // consume second '['
            let mut link = String::new();
            while let Some(&nc) = chars.peek() {
                if nc == ']' {
                    chars.next();
                    if chars.peek() == Some(&']') {
                        chars.next(); // consume second ']'
                        links.push(link);
                        break;
                    } else {
                        link.push(']');
                    }
                } else {
                    link.push(chars.next().unwrap());
                }
            }
        }
    }
    links
}

pub fn preprocess_wiki_links(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' && chars.peek() == Some(&'[') {
            chars.next(); // consume second '['
            let mut link = String::new();
            let mut found = false;
            while let Some(&nc) = chars.peek() {
                if nc == ']' {
                    chars.next();
                    if chars.peek() == Some(&']') {
                        chars.next(); // consume second ']'
                        found = true;
                        break;
                    } else {
                        link.push(']');
                    }
                } else {
                    link.push(chars.next().unwrap());
                }
            }
            if found {
                if let Some(pipe_pos) = link.find('|') {
                    let path = &link[..pipe_pos];
                    let label = &link[pipe_pos + 1..];
                    result.push_str(&format!("[{}]({}.md)", label, path));
                } else {
                    result.push_str(&format!("[{}]({}.md)", link, link));
                }
            } else {
                result.push_str("[[");
                result.push_str(&link);
            }
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_wiki_links_complete() {
        assert_eq!(preprocess_wiki_links("[[hello]]"), "[hello](hello.md)");
        assert_eq!(
            preprocess_wiki_links("[[hello|label]]"),
            "[label](hello.md)"
        );
    }

    #[test]
    fn test_preprocess_wiki_links_incomplete() {
        assert_eq!(preprocess_wiki_links("[[hello"), "[[hello");
        assert_eq!(preprocess_wiki_links("[[hello]"), "[[hello]");
        assert_eq!(preprocess_wiki_links("hello [ world"), "hello [ world");
    }

    #[test]
    fn test_extract_wiki_links() {
        assert_eq!(extract_wiki_links("[[hello]]"), vec!["hello".to_string()]);
        assert_eq!(
            extract_wiki_links("[[hello|label]]"),
            vec!["hello|label".to_string()]
        );
        let empty: Vec<String> = vec![];
        assert_eq!(extract_wiki_links("[[hello"), empty);
    }
}
