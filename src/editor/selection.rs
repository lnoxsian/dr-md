#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Selection {
    pub anchor: usize,
    pub head: usize,
}

impl Selection {
    pub fn new(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    pub fn start(&self) -> usize {
        self.anchor.min(self.head)
    }

    pub fn end(&self) -> usize {
        self.anchor.max(self.head)
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    pub fn range(&self) -> Option<std::ops::Range<usize>> {
        if self.is_empty() {
            None
        } else {
            Some(self.start()..self.end())
        }
    }

    pub fn clear(&mut self, cursor_idx: usize) {
        self.anchor = cursor_idx;
        self.head = cursor_idx;
    }
}
