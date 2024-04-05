use serde_json::json;
use tauri::{App, Wry};
use tauri_plugin_store::{JsonValue, Store, StoreBuilder};

const DEFAULT_LOCALE: &str = "en";
pub const SETTING_LOCALE: &str = "locale";
pub const SETTING_TELEMETRY: &str = "telemetry";
pub const SETTING_ADD_ITEM: &str = "add_items";

pub fn initialize_store(app: &App) -> Store<Wry> {
    let mut store = StoreBuilder::new(
        app.handle(),
        "config.json".parse().expect("can't create config file"),
    )
    .build();
    let _ = store.load();
    add_defaults(&mut store);
    store
}

pub fn insert_and_save(store: &mut Store<Wry>, key: String, value: JsonValue) {
    let _ = store.insert(key, value);
    let _ = store.save();
}

fn add_defaults(store: &mut Store<Wry>) {
    if !store.has(SETTING_LOCALE) {
        insert_and_save(store, SETTING_LOCALE.to_string(), json!(DEFAULT_LOCALE))
    }
    if !store.has(SETTING_TELEMETRY) {
        insert_and_save(store, SETTING_TELEMETRY.to_string(), json!(true))
    }
    if !store.has(SETTING_ADD_ITEM) {
        insert_and_save(store, SETTING_ADD_ITEM.to_string(), json!(true))
    }
}
