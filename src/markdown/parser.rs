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
