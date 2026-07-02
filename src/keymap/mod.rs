#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutAction {
    // File
    NewNote,
    NewFolder,
    OpenFolder,
    Save,
    SaveAs,
    CloseNote,

    // Editing
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,

    // Navigation
    QuickOpen,
    SearchInFile,
    SearchInVault,
    GoToLine,
    Back,
    Forward,

    // Markdown Formatting
    Bold,
    Italic,
    Link,
    CodeBlock,
    Checkbox,

    // Views
    ViewEditor,
    ViewPreview,
    ViewSplit,
    ToggleExplorer,
    ToggleFocusMode,
}

pub fn handle_key_events(ctx: &egui::Context) -> Option<ShortcutAction> {
    ctx.input(|i| {
        let ctrl = i.modifiers.command; // handles Ctrl on Linux/Windows and Cmd on macOS
        let shift = i.modifiers.shift;
        let alt = i.modifiers.alt;

        if ctrl && shift && i.key_pressed(egui::Key::N) {
            return Some(ShortcutAction::NewFolder);
        }
        if ctrl && i.key_pressed(egui::Key::N) {
            return Some(ShortcutAction::NewNote);
        }
        if ctrl && i.key_pressed(egui::Key::O) {
            return Some(ShortcutAction::OpenFolder);
        }
        if ctrl && shift && i.key_pressed(egui::Key::S) {
            return Some(ShortcutAction::SaveAs);
        }
        if ctrl && i.key_pressed(egui::Key::S) {
            return Some(ShortcutAction::Save);
        }
        if ctrl && i.key_pressed(egui::Key::W) {
            return Some(ShortcutAction::CloseNote);
        }

        // Editing
        if ctrl && shift && i.key_pressed(egui::Key::Z) {
            return Some(ShortcutAction::Redo);
        }
        if ctrl && i.key_pressed(egui::Key::Z) {
            return Some(ShortcutAction::Undo);
        }
        if ctrl && i.key_pressed(egui::Key::A) {
            return Some(ShortcutAction::SelectAll);
        }

        // Navigation
        if ctrl && i.key_pressed(egui::Key::P) {
            return Some(ShortcutAction::QuickOpen);
        }
        if ctrl && shift && i.key_pressed(egui::Key::F) {
            return Some(ShortcutAction::SearchInVault);
        }
        if ctrl && i.key_pressed(egui::Key::F) {
            return Some(ShortcutAction::SearchInFile);
        }
        if ctrl && i.key_pressed(egui::Key::G) {
            return Some(ShortcutAction::GoToLine);
        }
        if alt && i.key_pressed(egui::Key::ArrowLeft) {
            return Some(ShortcutAction::Back);
        }
        if alt && i.key_pressed(egui::Key::ArrowRight) {
            return Some(ShortcutAction::Forward);
        }

        // Markdown Formatting
        if ctrl && i.key_pressed(egui::Key::B) {
            return Some(ShortcutAction::Bold);
        }
        if ctrl && i.key_pressed(egui::Key::I) {
            return Some(ShortcutAction::Italic);
        }
        if ctrl && i.key_pressed(egui::Key::K) {
            return Some(ShortcutAction::Link);
        }
        if ctrl && shift && i.key_pressed(egui::Key::C) {
            return Some(ShortcutAction::CodeBlock);
        }
        if ctrl && i.key_pressed(egui::Key::L) {
            return Some(ShortcutAction::Checkbox);
        }

        // Views
        if ctrl && i.key_pressed(egui::Key::Num1) {
            return Some(ShortcutAction::ViewEditor);
        }
        if ctrl && i.key_pressed(egui::Key::Num2) {
            return Some(ShortcutAction::ViewPreview);
        }
        if ctrl && i.key_pressed(egui::Key::Num3) {
            return Some(ShortcutAction::ViewSplit);
        }
        if ctrl && i.key_pressed(egui::Key::E) {
            return Some(ShortcutAction::ToggleExplorer);
        }
        if i.key_pressed(egui::Key::F11) {
            return Some(ShortcutAction::ToggleFocusMode);
        }

        None
    })
}
