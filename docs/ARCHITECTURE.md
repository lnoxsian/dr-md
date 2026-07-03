# Architecture Overview

dr.md is organized as a small native desktop app with a clear split between UI state, editing primitives, preview rendering, and vault management.

## Entry Point

- `src/main.rs` initializes logging, loads the window icon, and launches `eframe`.
- `src/app/mod.rs` builds the top-level app state and handles the main update loop.

## Core Modules

- `src/app/` - menu bar, command execution, app state, and editor window orchestration.
- `src/config/` - config loading, saving, and theme application.
- `src/editor/` - rope-backed text storage, cursor handling, selection, undo/redo, and rendering support.
- `src/explorer/` - filesystem tree, selection, rename, drag and drop, and creation flows.
- `src/markdown/` - preview rendering and Markdown preprocessing.
- `src/workspace/` - vault root and active file tracking.
- `src/keymap/` - keyboard shortcut routing.

## Data Flow

1. The app loads config and restores the last opened folder when available.
2. The workspace vault exposes the current root and active file.
3. The explorer drives file creation, rename, selection, and movement.
4. The editor owns the loaded file contents and editing history.
5. The preview caches the editor version and re-renders when content changes.

## Rendering Model

The UI is built with `egui` immediate-mode widgets. The app uses the same native window for the explorer, editor, and preview, with split layouts controlled by the current view mode.

## Markdown Preview

The preview layer uses `egui_commonmark` and a cache to avoid reprocessing unchanged content. It also preprocesses wiki-style links and writes checkbox toggles back into the editor content.

## Editor Model

The editor uses `ropey` for text storage and stores cursor, selection, undo stack, dirty state, and active path separately. This keeps editing operations independent from the preview and file tree.

## File System Model

The application treats a selected folder as a vault. Files are opened, saved, and created directly inside that folder tree, and the explorer hides dotfiles by default.
