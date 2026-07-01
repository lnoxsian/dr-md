use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

pub struct MarkdownPreview {
    pub cache: CommonMarkCache,
}

impl MarkdownPreview {
    pub fn new() -> Self {
        Self {
            cache: CommonMarkCache::default(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, content: &str) {
        let processed = super::parser::preprocess_wiki_links(content);
        
        egui::ScrollArea::vertical()
            .id_source("markdown_preview_scroll")
            .show(ui, |ui| {
                CommonMarkViewer::new("markdown_viewer").show(ui, &mut self.cache, &processed);
            });
    }
}
