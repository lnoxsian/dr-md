pub struct UndoStack {
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
}

impl UndoStack {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, text: String) {
        if self.undo_stack.last() != Some(&text) {
            self.undo_stack.push(text);
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self, current: String) -> Option<String> {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(current);
            Some(prev)
        } else {
            None
        }
    }

    pub fn redo(&mut self, current: String) -> Option<String> {
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
    }
}
