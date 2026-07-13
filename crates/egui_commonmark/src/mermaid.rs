use std::hash::{Hash, Hasher};
use std::sync::OnceLock;
use egui::{Ui, Image};

static FONT_DB: OnceLock<std::sync::Arc<fontdb::Database>> = OnceLock::new();

fn get_font_db() -> std::sync::Arc<fontdb::Database> {
    FONT_DB.get_or_init(|| {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();

        // Query standard sans-serif candidates to set as default generic family
        let sans_candidates = [
            "DejaVu Sans",
            "Liberation Sans",
            "Noto Sans",
            "Arial",
            "Helvetica",
            "Ubuntu",
            "FreeSans",
        ];
        for family in sans_candidates {
            if db.query(&fontdb::Query {
                families: &[fontdb::Family::Name(family)],
                ..Default::default()
            }).is_some() {
                db.set_sans_serif_family(family);
                break;
            }
        }

        // Query standard monospace candidates to set as default generic family
        let mono_candidates = [
            "DejaVu Sans Mono",
            "Liberation Mono",
            "Noto Sans Mono",
            "Courier New",
            "Courier",
            "Ubuntu Mono",
            "FreeMono",
        ];
        for family in mono_candidates {
            if db.query(&fontdb::Query {
                families: &[fontdb::Family::Name(family)],
                ..Default::default()
            }).is_some() {
                db.set_monospace_family(family);
                break;
            }
        }

        std::sync::Arc::new(db)
    }).clone()
}

static SANS_SERIF_FAMILY: OnceLock<String> = OnceLock::new();

fn get_sans_serif_family() -> String {
    SANS_SERIF_FAMILY.get_or_init(|| {
        let db = get_font_db();
        let sans_candidates = [
            "DejaVu Sans",
            "Liberation Sans",
            "Noto Sans",
            "Arial",
            "Helvetica",
            "Ubuntu",
            "FreeSans",
        ];
        for family in sans_candidates {
            if db.query(&fontdb::Query {
                families: &[fontdb::Family::Name(family)],
                ..Default::default()
            }).is_some() {
                return family.to_string();
            }
        }
        "sans-serif".to_string()
    }).clone()
}

pub fn render_svg_to_png(svg_data: &str) -> Option<Vec<u8>> {
    let fontdb = get_font_db();
    let mut opt = usvg::Options::default();
    opt.fontdb = fontdb;
    opt.font_family = get_sans_serif_family();

    let tree = usvg::Tree::from_str(svg_data, &opt).ok()?;
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())?;

    resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
    pixmap.encode_png().ok()
}

#[derive(Clone, Debug)]
pub(crate) enum MermaidCacheEntry {
    Rendering,
    Ready {
        uri: String,
        bytes: std::sync::Arc<[u8]>,
    },
    Failure(String),
}

/// Renders a Mermaid diagram into the UI.
/// If local SVG-to-PNG rasterization succeeds (using system fonts for text rendering),
/// the diagram is rendered as a PNG image. Otherwise, it falls back to the raw SVG format.
/// Rendering is performed asynchronously in a background thread to prevent UI freezing.
pub fn render_mermaid(ui: &mut Ui, cache: &mut crate::CommonMarkCache, content: &str, max_width: f32) -> Result<(), String> {
    // Generate unique, deterministic cache key based on content and dark mode state.
    // Since the background is transparent, we don't need to re-render when the background color changes,
    // only when the light/dark mode changes (which affects text/border foreground colors).
    let dark_mode = ui.visuals().dark_mode;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content.hash(&mut hasher);
    dark_mode.hash(&mut hasher);
    let hash = hasher.finish();

    let entry = {
        let mut map = cache.mermaid_cache.lock().unwrap();
        if let Some(entry) = map.get(&hash) {
            entry.clone()
        } else {
            map.insert(hash, MermaidCacheEntry::Rendering);

            let cache_clone = cache.mermaid_cache.clone();
            let content_str = content.to_string();
            let ctx = ui.ctx().clone();

            std::thread::spawn(move || {
                let result = render_mermaid_sync(&content_str, dark_mode);
                let entry = match result {
                    Ok((uri, bytes)) => MermaidCacheEntry::Ready {
                        uri,
                        bytes: std::sync::Arc::from(bytes),
                    },
                    Err(err) => MermaidCacheEntry::Failure(err),
                };
                cache_clone.lock().unwrap().insert(hash, entry);
                ctx.request_repaint();
            });

            MermaidCacheEntry::Rendering
        }
    };

    match entry {
        MermaidCacheEntry::Rendering => {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Rendering Mermaid diagram...");
            });
            Ok(())
        }
        MermaidCacheEntry::Ready { uri, bytes } => {
            let image = Image::from_bytes(uri, bytes)
                .fit_to_original_size(1.0)
                .max_width(max_width);
            ui.add(image);
            Ok(())
        }
        MermaidCacheEntry::Failure(err) => Err(err),
    }
}

fn render_mermaid_sync(content: &str, dark_mode: bool) -> Result<(String, Vec<u8>), String> {
    // Determine theme based on UI visuals
    let mut theme = if dark_mode {
        mermaid_rs_renderer::Theme::dark()
    } else {
        mermaid_rs_renderer::Theme::modern()
    };
    
    // Set background to transparent so that the image is rendered with a transparent background.
    // This allows the diagram to dynamically adapt to any theme background color instantly without re-rendering.
    theme.background = "none".to_string();
    
    // Explicitly align the layout font family to the resolved system font family
    let system_font = get_sans_serif_family();
    theme.font_family = format!("{}, sans-serif", system_font);

    let config = mermaid_rs_renderer::LayoutConfig::default();

    // Parse and render to SVG
    let parsed = mermaid_rs_renderer::parse_mermaid(content)
        .map_err(|err| format!("{}", err))?;
    let layout = mermaid_rs_renderer::compute_layout(&parsed.graph, &theme, &config);
    let svg_string = mermaid_rs_renderer::render_svg(&layout, &theme, &config);

    // Render SVG to PNG to ensure text and labels render properly
    let (uri, bytes) = if let Some(png_bytes) = render_svg_to_png(&svg_string) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        content.hash(&mut hasher);
        dark_mode.hash(&mut hasher);
        let hash = hasher.finish();
        (format!("bytes://mermaid_{}.png", hash), png_bytes)
    } else {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        content.hash(&mut hasher);
        dark_mode.hash(&mut hasher);
        let hash = hasher.finish();
        (format!("bytes://mermaid_{}.svg", hash), svg_string.into_bytes())
    };

    Ok((uri, bytes))
}
