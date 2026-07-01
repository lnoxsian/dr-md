use ropey::Rope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cursor {
    pub char_idx: usize,
}

impl Cursor {
    pub fn new(char_idx: usize) -> Self {
        Self { char_idx }
    }

    pub fn line_col(&self, rope: &Rope) -> (usize, usize) {
        let line_idx = rope.char_to_line(self.char_idx.min(rope.len_chars()));
        let line_char_idx = rope.line_to_char(line_idx);
        let col = self.char_idx.min(rope.len_chars()) - line_char_idx;
        (line_idx, col)
    }

    pub fn from_line_col(rope: &Rope, line: usize, col: usize) -> Self {
        let line_idx = line.min(rope.len_lines().saturating_sub(1));
        let line_char_idx = rope.line_to_char(line_idx);
        let line_len = rope.line(line_idx).len_chars();
        // Don't include trailing newline if there is one
        let max_col = if line_idx < rope.len_lines().saturating_sub(1) {
            line_len.saturating_sub(1)
        } else {
            line_len
        };
        let col_idx = col.min(max_col);
        Self {
            char_idx: line_char_idx + col_idx,
        }
    }

    pub fn move_left(&mut self, _rope: &Rope) {
        if self.char_idx > 0 {
            self.char_idx -= 1;
        }
    }

    pub fn move_right(&mut self, rope: &Rope) {
        if self.char_idx < rope.len_chars() {
            self.char_idx += 1;
        }
    }

    pub fn move_up(&mut self, rope: &Rope) {
        let (line, col) = self.line_col(rope);
        if line > 0 {
            *self = Self::from_line_col(rope, line - 1, col);
        }
    }

    pub fn move_down(&mut self, rope: &Rope) {
        let (line, col) = self.line_col(rope);
        if line < rope.len_lines().saturating_sub(1) {
            *self = Self::from_line_col(rope, line + 1, col);
        }
    }
}
