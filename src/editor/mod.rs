pub mod buffer;
pub mod cursor;
pub mod selection;
pub mod undo;
pub mod renderer;

pub use buffer::EditBuffer;
pub use cursor::Cursor;
pub use selection::Selection;
pub use undo::UndoStack;

use std::path::PathBuf;

pub struct Editor {
    pub buffer: EditBuffer,
    pub cursor: Cursor,
    pub selection: Selection,
    pub undo_stack: UndoStack,
    pub active_path: Option<PathBuf>,
    pub is_dirty: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: EditBuffer::new(),
            cursor: Cursor::default(),
            selection: Selection::default(),
            undo_stack: UndoStack::new(),
            active_path: None,
            is_dirty: false,
        }
    }

    pub fn load_file(&mut self, path: PathBuf) -> anyhow::Result<()> {
        self.buffer.load_file(&path)?;
        self.active_path = Some(path);
        self.cursor = Cursor::default();
        self.selection.clear(0);
        self.undo_stack.clear();
        self.is_dirty = false;
        Ok(())
    }

    pub fn save_file(&mut self) -> anyhow::Result<()> {
        if let Some(ref path) = self.active_path {
            self.buffer.save_file(path)?;
            self.is_dirty = false;
        }
        Ok(())
    }

    pub fn save_as(&mut self, path: PathBuf) -> anyhow::Result<()> {
        self.buffer.save_file(&path)?;
        self.active_path = Some(path);
        self.is_dirty = false;
        Ok(())
    }

    pub fn set_text(&mut self, text: &str) {
        self.buffer = EditBuffer::from_str(text);
        self.cursor = Cursor::default();
        self.selection.clear(0);
        self.undo_stack.clear();
        self.is_dirty = true;
    }

    pub fn insert_text(&mut self, text: &str) {
        let current_text = self.buffer.to_string();
        self.undo_stack.push(current_text);

        if let Some(range) = self.selection.range() {
            self.buffer.remove(range.start, range.end);
            self.cursor.char_idx = range.start;
            self.selection.clear(self.cursor.char_idx);
        }

        self.buffer.insert(self.cursor.char_idx, text);
        self.cursor.char_idx += text.chars().count();
        self.selection.clear(self.cursor.char_idx);
        self.is_dirty = true;
    }

    pub fn delete_backward(&mut self) {
        if let Some(range) = self.selection.range() {
            let current_text = self.buffer.to_string();
            self.undo_stack.push(current_text);

            self.buffer.remove(range.start, range.end);
            self.cursor.char_idx = range.start;
            self.selection.clear(self.cursor.char_idx);
            self.is_dirty = true;
        } else if self.cursor.char_idx > 0 {
            let current_text = self.buffer.to_string();
            self.undo_stack.push(current_text);

            let start = self.cursor.char_idx - 1;
            let end = self.cursor.char_idx;
            self.buffer.remove(start, end);
            self.cursor.char_idx = start;
            self.selection.clear(self.cursor.char_idx);
            self.is_dirty = true;
        }
    }

    pub fn delete_forward(&mut self) {
        if let Some(range) = self.selection.range() {
            let current_text = self.buffer.to_string();
            self.undo_stack.push(current_text);

            self.buffer.remove(range.start, range.end);
            self.cursor.char_idx = range.start;
            self.selection.clear(self.cursor.char_idx);
            self.is_dirty = true;
        } else if self.cursor.char_idx < self.buffer.len_chars() {
            let current_text = self.buffer.to_string();
            self.undo_stack.push(current_text);

            let start = self.cursor.char_idx;
            let end = self.cursor.char_idx + 1;
            self.buffer.remove(start, end);
            self.selection.clear(self.cursor.char_idx);
            self.is_dirty = true;
        }
    }

    pub fn undo(&mut self) {
        let current_text = self.buffer.to_string();
        if let Some(prev) = self.undo_stack.undo(current_text) {
            self.buffer = EditBuffer::from_str(&prev);
            self.cursor.char_idx = self.cursor.char_idx.min(self.buffer.len_chars());
            self.selection.clear(self.cursor.char_idx);
            self.is_dirty = true;
        }
    }

    pub fn redo(&mut self) {
        let current_text = self.buffer.to_string();
        if let Some(next) = self.undo_stack.redo(current_text) {
            self.buffer = EditBuffer::from_str(&next);
            self.cursor.char_idx = self.cursor.char_idx.min(self.buffer.len_chars());
            self.selection.clear(self.cursor.char_idx);
            self.is_dirty = true;
        }
    }
}
