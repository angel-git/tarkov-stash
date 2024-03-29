// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::Mutex;

use log::LevelFilter;
use tauri::{Manager, State, Wry};
use tauri_plugin_log::LogTarget;
use tauri_plugin_store::Store;

use prelude::*;

pub mod app;
pub mod spt;
pub mod stash;
pub mod ui_profile;
pub mod utils;

mod prelude {
    pub use serde::de::Deserializer;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{json, Error, Value};

    pub use crate::app::handles::*;
    pub use crate::app::menu::*;
    pub use crate::app::store::*;
    pub use crate::app::telemetry::*;
    pub use crate::spt::*;
    pub use crate::stash::stash_utils::*;
    pub use crate::ui_profile::ui_profile_serializer::*;
    pub use crate::utils::*;
}

pub struct TarkovStashState {
    pub state: Mutex<MutexState>,
}

pub struct MutexState {
    pub profile_file_path: Option<String>,
    pub bsg_items: Option<HashMap<String, Value>>,
    pub globals: Option<HashMap<String, Value>>,
    pub locale: Option<HashMap<String, Value>>,
    pub store: Option<Store<Wry>>,
}

fn main() {
    let apta_key = load_apta_key();
    tauri::Builder::default()
        .plugin(tauri_plugin_aptabase::Builder::new(apta_key.as_str()).build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout])
                .level(LevelFilter::Info)
                .build(),
        )
        .menu(build_menu())
        .manage(TarkovStashState {
            state: Mutex::new(MutexState {
                bsg_items: None,
                globals: None,
                locale: None,
                profile_file_path: None,
                store: None,
            }),
        })
        .on_menu_event(handle_menu_event)
        .setup(|app| {
            {
                let state: State<TarkovStashState> = app.state();
                let mut internal_state = state.state.lock().unwrap();
                let store = initialize_store(app);

                let main_window = app.get_window("main").unwrap();
                let locale_id = format!(
                    "locale_{}",
                    store.get(SETTING_LOCALE).unwrap().as_str().unwrap()
                );
                let telemetry_selected = store.get(SETTING_TELEMETRY).unwrap().as_bool().unwrap();
                update_selected_menu_telemetry(main_window.menu_handle(), telemetry_selected);
                update_selected_menu_locale(main_window.menu_handle(), locale_id);
                internal_state.store = Some(store);
            }
            // track event needs its own lock
            {
                track_event(&app.handle(), "app_started", None);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_profile_file,
            change_amount,
            change_fir,
            restore_durability,
            add_item,
            remove_item,
            add_preset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
