#![allow(dead_code)]

mod app;
mod config;
mod editor;
mod explorer;
mod keymap;
mod markdown;
mod workspace;

use app::DoctorMarkdownApp;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting dr.md (Doctor Markdown)");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("dr.md")
            .with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dr.md",
        options,
        Box::new(|cc| Box::new(DoctorMarkdownApp::new(cc, None))),
    )
}
