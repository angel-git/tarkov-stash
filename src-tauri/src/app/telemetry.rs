use serde_json::Value;
use tauri::{Manager, State};
use tauri_plugin_aptabase::EventTracker;

use crate::prelude::SETTING_TELEMETRY;
use crate::TarkovStashState;

pub fn track_event(app: &tauri::AppHandle, name: &str, props: Option<Value>) {
    let state: State<TarkovStashState> = app.state();
    let mut internal_state = state.state.lock().unwrap();
    let store = internal_state.store.as_mut().unwrap();
    if store.get(SETTING_TELEMETRY).unwrap().as_bool().unwrap() {
        app.track_event(name, props);
    }
}

pub fn load_apta_key() -> String {
    option_env!("APTABASE_KEY")
        .unwrap_or_else(|| "")
        .to_string()
}
