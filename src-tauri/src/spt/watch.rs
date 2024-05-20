use std::{
    collections::HashMap,
    path::PathBuf,
    sync::mpsc::{channel, Receiver},
    thread::spawn,
};

use log::{error, info, warn};
use notify::{Config, Error, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::Window;

type Result = std::result::Result<Event, Error>;

#[derive(Default)]
pub struct WatcherCollection(HashMap<String, (PathBuf, RecommendedWatcher, String)>);

pub fn watch(
    event_name: String,
    path: &PathBuf,
    window: Window,
    watcher_collection: &mut WatcherCollection,
    session_id: String,
) {
    let (tx, rx) = channel();
    let watcher_result = RecommendedWatcher::new(tx, Config::default());
    match watcher_result {
        Ok(mut watcher) => {
            if watcher.watch(path, RecursiveMode::Recursive).is_ok() {
                watch_raw(window, rx, event_name.clone(), session_id.clone());
                watcher_collection.0.insert(
                    event_name.clone(),
                    (path.clone(), watcher, session_id.clone()),
                );
                info!(
                    "Starting to watch changes for {:?} in {:?}",
                    event_name, path
                );
            } else {
                error!(
                    "Failed to create watcher for {:?} in {:?}",
                    event_name, path
                );
            }
        }
        Err(e) => {
            error!("Failed to create watcher: {:?}", e);
        }
    }
}

pub fn unwatch(event_name: String, watcher_collection: &mut WatcherCollection) {
    if let Some((path, mut watcher, _)) = watcher_collection.0.remove(&event_name) {
        if watcher.unwatch(&path).is_ok() {
            info!(
                "Stopped watching changes for {:?} in {:?}",
                event_name, path
            );
        } else {
            error!(
                "Failed to stop watching changes for {:?} in {:?}",
                event_name, path
            );
        }
    } else {
        warn!("Failed to find watcher for {:?}", event_name);
    }
}

fn watch_raw(window: Window, rx: Receiver<Result>, id: String, session_id: String) {
    spawn(move || {
        info!("Started watcher for: {:?}", id);
        while let Ok(event) = rx.recv() {
            if event.is_ok() {
                info!("File changed: {:?} for session {:?}", id, session_id);
                window
                    .emit(&id, session_id.clone())
                    .expect("Failed to emit event");
            } else {
                error!("Failed to watch file: {:?}", id);
            }
        }
        info!("Stopped watcher for: {:?}", id);
    });
}
