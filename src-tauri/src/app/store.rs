use crate::{MutexState, DEFAULT_LOCALE, SETTING_LOCALE, SETTING_TELEMETRY};
use serde_json::json;
use std::sync::MutexGuard;
use tauri::{App, Wry};
use tauri_plugin_store::{JsonValue, Store, StoreBuilder};

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

fn add_defaults(store: &mut Store<Wry>) {
    if !store.has(SETTING_LOCALE) {
        insert_and_save(store, SETTING_LOCALE.to_string(), json!(DEFAULT_LOCALE))
    }
    if !store.has(SETTING_TELEMETRY) {
        insert_and_save(store, SETTING_TELEMETRY.to_string(), json!(true))
    }
}

pub fn update_state_locale_from_store(
    store: &Store<Wry>,
    internal_state: &mut MutexGuard<MutexState>,
) {
    if store.has(SETTING_LOCALE) {
        let locale_from_settings = store.get(SETTING_LOCALE).unwrap().as_str().unwrap();
        internal_state.locale_lang = locale_from_settings.to_string();
    }
}

pub fn update_state_telemetry_from_store(
    store: &Store<Wry>,
    internal_state: &mut MutexGuard<MutexState>,
) {
    if store.has(SETTING_TELEMETRY) {
        let telemetry_from_settings = store.get(SETTING_TELEMETRY).unwrap().as_bool().unwrap();
        internal_state.telemetry_enabled = telemetry_from_settings;
    }
}

pub fn insert_and_save(store: &mut Store<Wry>, key: String, value: JsonValue) {
    let _ = store.insert(key, value);
    let _ = store.save();
}
