use std::net::TcpStream;
use std::path::PathBuf;

use log::error;
use serde_json::json;
use tauri::api::dialog::FileDialogBuilder;
use tauri::api::shell::open;
use tauri::window::MenuHandle;
use tauri::{CustomMenuItem, Manager, Menu, State, Submenu, WindowMenuEvent};

use crate::prelude::insert_and_save;
use crate::{TarkovStashState, SETTING_LOCALE};

pub fn build_menu() -> Menu {
    let open = CustomMenuItem::new("open".to_string(), "Open profile");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let locale_cz = CustomMenuItem::new("locale_cz".to_string(), "Czech");
    let locale_en = CustomMenuItem::new("locale_en".to_string(), "English").selected();
    let locale_fr = CustomMenuItem::new("locale_fr".to_string(), "French");
    let locale_ge = CustomMenuItem::new("locale_ge".to_string(), "German");
    let locale_hu = CustomMenuItem::new("locale_hu".to_string(), "Hungarian");
    let locale_it = CustomMenuItem::new("locale_it".to_string(), "Italian");
    let locale_jp = CustomMenuItem::new("locale_jp".to_string(), "Japanese");
    let locale_kr = CustomMenuItem::new("locale_kr".to_string(), "Korean");
    let locale_pl = CustomMenuItem::new("locale_pl".to_string(), "Polish");
    let locale_po = CustomMenuItem::new("locale_po".to_string(), "Portugal");
    let locale_sk = CustomMenuItem::new("locale_sk".to_string(), "Slovak");
    let locale_es = CustomMenuItem::new("locale_es".to_string(), "Spanish");
    let locale_es_mx = CustomMenuItem::new("locale_es-mx".to_string(), "Spanish Mexico");
    let locale_tu = CustomMenuItem::new("locale_tu".to_string(), "Turkish");
    let locale_ru = CustomMenuItem::new("locale_ru".to_string(), "Русский");
    let file_submenu = Submenu::new("File", Menu::new().add_item(open).add_item(quit));
    let locale_submenu = Submenu::new(
        "Locale",
        Menu::new()
            .add_item(locale_cz)
            .add_item(locale_en)
            .add_item(locale_fr)
            .add_item(locale_ge)
            .add_item(locale_hu)
            .add_item(locale_it)
            .add_item(locale_jp)
            .add_item(locale_kr)
            .add_item(locale_pl)
            .add_item(locale_po)
            .add_item(locale_sk)
            .add_item(locale_es)
            .add_item(locale_es_mx)
            .add_item(locale_tu)
            .add_item(locale_ru),
    );

    let open_logs = CustomMenuItem::new("open_logs".to_string(), "Open logs");
    let source_code = CustomMenuItem::new("view_source_code".to_string(), "View source code");
    let config = CustomMenuItem::new("open_config".to_string(), "Open config");

    let help_submenu = Submenu::new(
        "Help",
        Menu::new()
            .add_item(open_logs)
            .add_item(config)
            .add_item(source_code),
    );

    Menu::new()
        .add_submenu(file_submenu)
        .add_submenu(locale_submenu)
        .add_submenu(help_submenu)
}

pub fn handle_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "open" => {
            FileDialogBuilder::default()
                .add_filter("json", &["json"])
                .pick_file(move |path_buf| if let Some(p) = path_buf {
                    let window = event.window();
                    let is_server_running = is_server_running();
                    if is_server_running {
                        window.emit("error", "Looks like your server is running, please stop it and try to open your profile again").expect("Can't emit event to window!");
                    } else {
                        let state: State<TarkovStashState> = window.state();
                        let mut internal_state = state.state.lock().unwrap();
                        internal_state.profile_file_path =
                            Some(p.as_path().to_str().unwrap().to_string());
                        window.emit("profile_loaded", "").expect("Can't emit event to window!");
                    }
                });
        }
        "locale_cz" | "locale_en" | "locale_fr" | "locale_ge" | "locale_hu" | "locale_it"
        | "locale_jp" | "locale_kr" | "locale_pl" | "locale_po" | "locale_sk" | "locale_es"
        | "locale_es-mx" | "locale_tu" | "locale_ru" => {
            let window = event.window();
            let menu_handle = window.menu_handle();
            let state: State<TarkovStashState> = window.state();
            let mut internal_state = state.state.lock().unwrap();
            let locale_lang = event.menu_item_id().replace("locale_", "").to_string();
            internal_state.locale_lang = locale_lang.clone();
            let menu_item_id = event.menu_item_id().to_string();

            {
                let store = internal_state.store.as_mut().unwrap();
                insert_and_save(store, SETTING_LOCALE.to_string(), json!(locale_lang));
            }
            if internal_state.profile_file_path.is_some() {
                window
                    .emit("profile_loaded", "")
                    .expect("Can't emit event to window!");
            }

            update_selected_menu_locale(menu_handle, menu_item_id)
        }
        "open_logs" => {
            let path = event
                .window()
                .app_handle()
                .path_resolver()
                .app_log_dir()
                .unwrap();
            open_directory(&event, path, "Can't open logs folder");
        }
        "open_config" => {
            let path = event
                .window()
                .app_handle()
                .path_resolver()
                .app_data_dir()
                .unwrap();
            open_directory(&event, path, "Can't open config folder");
        }
        "view_source_code" => open_url(&event, "https://github.com/angel-git/tarkov-stash"),
        _ => {}
    }
}

pub fn update_selected_menu_locale(menu_handle: MenuHandle, id: String) {
    std::thread::spawn(move || {
        menu_handle
            .get_item("locale_cz")
            .set_selected(false)
            .expect("Can't find menu item for locale_cz");
        menu_handle
            .get_item("locale_en")
            .set_selected(false)
            .expect("Can't find menu item for locale_en");
        menu_handle
            .get_item("locale_fr")
            .set_selected(false)
            .expect("Can't find menu item for locale_fr");
        menu_handle
            .get_item("locale_ge")
            .set_selected(false)
            .expect("Can't find menu item for locale_ge");
        menu_handle
            .get_item("locale_hu")
            .set_selected(false)
            .expect("Can't find menu item for locale_hu");
        menu_handle
            .get_item("locale_it")
            .set_selected(false)
            .expect("Can't find menu item for locale_it");
        menu_handle
            .get_item("locale_jp")
            .set_selected(false)
            .expect("Can't find menu item for locale_jp");
        menu_handle
            .get_item("locale_kr")
            .set_selected(false)
            .expect("Can't find menu item for locale_kr");
        menu_handle
            .get_item("locale_pl")
            .set_selected(false)
            .expect("Can't find menu item for locale_pl");
        menu_handle
            .get_item("locale_po")
            .set_selected(false)
            .expect("Can't find menu item for locale_po");
        menu_handle
            .get_item("locale_sk")
            .set_selected(false)
            .expect("Can't find menu item for locale_sk");
        menu_handle
            .get_item("locale_es")
            .set_selected(false)
            .expect("Can't find menu item for locale_es");
        menu_handle
            .get_item("locale_es-mx")
            .set_selected(false)
            .expect("Can't find menu item for locale_es-mx");
        menu_handle
            .get_item("locale_tu")
            .set_selected(false)
            .expect("Can't find menu item for locale_tu");
        menu_handle
            .get_item("locale_ru")
            .set_selected(false)
            .expect("Can't find menu item for locale_ru");
        menu_handle
            .get_item(&id)
            .set_selected(true)
            .expect("Can't find selected menu item");
    });
}

fn is_server_running() -> bool {
    TcpStream::connect("127.0.0.1:6969").is_ok()
}

fn open_directory(event: &WindowMenuEvent, path: PathBuf, error_msg: &'static str) {
    let window = event.window();
    let scope = window.app_handle().shell_scope();
    if let Err(e) = open(&scope, path.display().to_string(), None) {
        handle_error(error_msg, e)
    }
}

fn open_url(event: &WindowMenuEvent, url: &'static str) {
    let window = event.window();
    let scope = window.app_handle().shell_scope();
    if let Err(e) = open(&scope, url, None) {
        handle_error("Can't open browser!", e)
    }
}

fn handle_error(error_msg: &str, e: tauri::api::Error) {
    error!("{}: {}", error_msg, e);
}
