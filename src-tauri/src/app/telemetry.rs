use crate::TarkovStashState;
use serde_json::Value;
use tauri::{Manager, State};
use tauri_plugin_aptabase::EventTracker;

pub fn track_event(app: &tauri::AppHandle, name: &str, props: Option<Value>) {
    let state: State<TarkovStashState> = app.state();
    if state.state.lock().unwrap().telemetry_enabled {
        println!("sending tracking");
        app.track_event(name, props);
    } else {
        println!("disabled tracking");
    }
}

pub fn load_apta_key() -> String {
    option_env!("APTABASE_KEY")
        .unwrap_or_else(|| "")
        .to_string()
}
