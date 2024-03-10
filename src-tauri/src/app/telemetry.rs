use serde_json::Value;
use tauri_plugin_aptabase::EventTracker;

pub fn track_event(app: &tauri::AppHandle, name: &str, props: Option<Value>) {
    // TODO if user opt-out don't call this
    app.track_event(name, props)
}

pub fn load_apta_key() -> String {
    option_env!("APTABASE_KEY")
        .unwrap_or_else(|| "")
        .to_string()
}
