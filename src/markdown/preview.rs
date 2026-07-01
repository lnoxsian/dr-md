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

    pub fn show(&mut self, ui: &mut egui::Ui, content: &str, font_size: f32) {
        let processed = super::parser::preprocess_wiki_links(content);
        
        egui::ScrollArea::vertical()
            .id_source("markdown_preview_scroll")
            .show(ui, |ui| {
                let mut style = ui.style().as_ref().clone();
                let body_font = egui::FontId::new(font_size, egui::FontFamily::Proportional);
                let heading_font = egui::FontId::new(font_size * 1.4, egui::FontFamily::Proportional);
                let monospace_font = egui::FontId::new(font_size, egui::FontFamily::Monospace);
                
                style.text_styles.insert(egui::TextStyle::Body, body_font);
                style.text_styles.insert(egui::TextStyle::Heading, heading_font);
                style.text_styles.insert(egui::TextStyle::Monospace, monospace_font);
                
                ui.set_style(style);

                CommonMarkViewer::new("markdown_viewer").show(ui, &mut self.cache, &processed);
            });
    }
}
