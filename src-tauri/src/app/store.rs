use crate::{MutexState, SETTING_LOCALE, SETTING_TELEMETRY};
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
    store
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
