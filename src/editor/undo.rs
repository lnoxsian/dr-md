#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditKind {
    None,
    InsertWord,
    InsertSpace,
    InsertPunctuation,
    DeleteWord,
    DeleteSpace,
    DeletePunctuation,
    Replace,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditType {
    Insert,
    Delete,
    Replace,
}

pub struct UndoStack {
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,

    // Tracking state for word-by-word grouping
    last_edit_kind: EditKind,
    last_edit_idx: usize,
    last_edit_len: usize,
}

impl UndoStack {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_edit_kind: EditKind::None,
            last_edit_idx: 0,
            last_edit_len: 0,
        }
    }

    pub fn push(&mut self, text: String) {
        if self.undo_stack.last() != Some(&text) {
            self.undo_stack.push(text);
            self.redo_stack.clear();
        }
        self.reset_tracking();
    }

    pub fn push_change(&mut self, old_text: String, new_text: &str) {
        let diff = find_char_difference(&old_text, new_text);
        let (edit_type, start_idx, deleted, inserted) = match diff {
            Some(d) => d,
            None => return,
        };

        let current_kind = determine_edit_kind(
            &edit_type,
            if edit_type == EditType::Insert {
                &inserted
            } else {
                &deleted
            },
        );
        let change_len = if edit_type == EditType::Insert {
            inserted.chars().count()
        } else {
            deleted.chars().count()
        };

        let is_boundary = self.last_edit_kind == EditKind::None
            || current_kind == EditKind::Other
            || current_kind == EditKind::Replace
            || self.last_edit_kind != current_kind
            || !self.is_contiguous(start_idx, change_len, &edit_type);

        if is_boundary {
            if self.undo_stack.last() != Some(&old_text) {
                self.undo_stack.push(old_text);
                self.redo_stack.clear();
            }
        }

        self.last_edit_kind = current_kind;
        self.last_edit_idx = start_idx;
        self.last_edit_len = change_len;
    }

    pub fn undo(&mut self, current: String) -> Option<String> {
        self.reset_tracking();
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(current);
            Some(prev)
        } else {
            None
        }
    }

    pub fn redo(&mut self, current: String) -> Option<String> {
        self.reset_tracking();
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(current);
            Some(next)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.reset_tracking();
    }

    fn reset_tracking(&mut self) {
        self.last_edit_kind = EditKind::None;
        self.last_edit_idx = 0;
        self.last_edit_len = 0;
    }

    fn is_contiguous(&self, start_idx: usize, len: usize, edit_type: &EditType) -> bool {
        match edit_type {
            EditType::Insert => start_idx == self.last_edit_idx + self.last_edit_len,
            EditType::Delete => {
                start_idx + len == self.last_edit_idx || start_idx == self.last_edit_idx
            }
            EditType::Replace => false,
        }
    }
}

fn classify_char(c: char) -> EditKind {
    if c.is_whitespace() {
        EditKind::InsertSpace
    } else if c.is_alphanumeric() || c == '_' {
        EditKind::InsertWord
    } else {
        EditKind::InsertPunctuation
    }
}

fn classify_str(s: &str) -> EditKind {
    if s.is_empty() {
        return EditKind::None;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    let first_kind = classify_char(first);
    for c in chars {
        if classify_char(c) != first_kind {
            return EditKind::Other;
        }
    }
    first_kind
}

fn determine_edit_kind(edit_type: &EditType, text: &str) -> EditKind {
    let base_kind = classify_str(text);
    match edit_type {
        EditType::Insert => match base_kind {
            EditKind::InsertWord => EditKind::InsertWord,
            EditKind::InsertSpace => EditKind::InsertSpace,
            EditKind::InsertPunctuation => EditKind::InsertPunctuation,
            _ => EditKind::Other,
        },
        EditType::Delete => match base_kind {
            EditKind::InsertWord => EditKind::DeleteWord,
            EditKind::InsertSpace => EditKind::DeleteSpace,
            EditKind::InsertPunctuation => EditKind::DeletePunctuation,
            _ => EditKind::Other,
        },
        EditType::Replace => EditKind::Replace,
    }
}

pub fn find_char_difference(
    old_str: &str,
    new_str: &str,
) -> Option<(EditType, usize, String, String)> {
    let old_chars: Vec<char> = old_str.chars().collect();
    let new_chars: Vec<char> = new_str.chars().collect();

    let mut prefix_len = 0;
    while prefix_len < old_chars.len()
        && prefix_len < new_chars.len()
        && old_chars[prefix_len] == new_chars[prefix_len]
    {
        prefix_len += 1;
    }

    let mut suffix_len = 0;
    while suffix_len < (old_chars.len() - prefix_len)
        && suffix_len < (new_chars.len() - prefix_len)
        && old_chars[old_chars.len() - 1 - suffix_len]
            == new_chars[new_chars.len() - 1 - suffix_len]
    {
        suffix_len += 1;
    }

    let deleted_range = prefix_len..(old_chars.len() - suffix_len);
    let inserted_range = prefix_len..(new_chars.len() - suffix_len);

    if deleted_range.is_empty() && inserted_range.is_empty() {
        return None;
    }

    let deleted: String = old_chars[deleted_range.clone()].iter().collect();
    let inserted: String = new_chars[inserted_range.clone()].iter().collect();

    let edit_type = if deleted.is_empty() {
        EditType::Insert
    } else if inserted.is_empty() {
        EditType::Delete
    } else {
        EditType::Replace
    };

    Some((edit_type, prefix_len, deleted, inserted))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_char_difference() {
        assert_eq!(find_char_difference("hello", "hello"), None);
        assert_eq!(
            find_char_difference("hello", "hello world"),
            Some((EditType::Insert, 5, "".to_string(), " world".to_string()))
        );
        assert_eq!(
            find_char_difference("hello world", "hello"),
            Some((EditType::Delete, 5, " world".to_string(), "".to_string()))
        );
        assert_eq!(
            find_char_difference("hello", "hella"),
            Some((EditType::Replace, 4, "o".to_string(), "a".to_string()))
        );
    }

    #[test]
    fn test_undo_stack_word_by_word() {
        let mut stack = UndoStack::new();

        // Type "hello" character by character
        stack.push_change("".to_string(), "h");
        stack.push_change("h".to_string(), "he");
        stack.push_change("he".to_string(), "hel");
        stack.push_change("hel".to_string(), "hell");
        stack.push_change("hell".to_string(), "hello");

        // Type space " "
        stack.push_change("hello".to_string(), "hello ");

        // Type "world"
        stack.push_change("hello ".to_string(), "hello w");
        stack.push_change("hello w".to_string(), "hello wo");
        stack.push_change("hello wo".to_string(), "hello wor");
        stack.push_change("hello wor".to_string(), "hello worl");
        stack.push_change("hello worl".to_string(), "hello world");

        // Now undo
        let u1 = stack.undo("hello world".to_string());
        // Should go back to "hello " (before we started typing "world")
        assert_eq!(u1, Some("hello ".to_string()));

        let u2 = stack.undo("hello ".to_string());
        // Should go back to "hello" (before we started typing space)
        assert_eq!(u2, Some("hello".to_string()));

        let u3 = stack.undo("hello".to_string());
        // Should go back to "" (before we started typing "hello")
        assert_eq!(u3, Some("".to_string()));
    }
}
