use std::path::Path;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event};
use std::sync::mpsc::{channel, Receiver};

pub struct FileWatcher {
    watcher: Option<RecommendedWatcher>,
    pub rx: Receiver<Result<Event, notify::Error>>,
}

impl FileWatcher {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        }).ok();

        Self {
            watcher,
            rx,
        }
    }

    pub fn watch(&mut self, path: &Path) -> Result<(), notify::Error> {
        if let Some(ref mut w) = self.watcher {
            w.watch(path, RecursiveMode::Recursive)?;
        }
        Ok(())
    }

    pub fn unwatch(&mut self, path: &Path) -> Result<(), notify::Error> {
        if let Some(ref mut w) = self.watcher {
            w.unwatch(path)?;
        }
        Ok(())
    }
}
