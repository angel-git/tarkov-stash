// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate dotenv;

pub mod spt;
pub mod stash;
pub mod ui_profile;
pub mod utils;

mod prelude {
    pub use crate::spt::*;
    pub use crate::stash::stash_utils::*;
    pub use crate::ui_profile::ui_profile_serializer::*;
    pub use crate::utils::*;
    pub use serde::de::Deserializer;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{json, Error, Value};
}

use dotenv::dotenv;
use log::{error, info, LevelFilter};
use prelude::*;
use std::collections::HashMap;
use std::fs;
use std::net::TcpStream;
use std::path::Path;
use std::sync::Mutex;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{CustomMenuItem, Manager, Menu, State, Submenu};
use tauri_plugin_aptabase::EventTracker;
use tauri_plugin_log::LogTarget;

struct TarkovStashState {
    pub state: Mutex<MutexState>,
}

struct MutexState {
    pub profile_file_path: Option<String>,
    pub bsg_items: Option<HashMap<String, Value>>,
    pub globals: Option<HashMap<String, Value>>,
    pub locale: Option<HashMap<String, Value>>,
    pub locale_lang: String,
}

fn main() {
    dotenv().ok();

    let apta_key = load_apta_key();
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

    let menu = Menu::new()
        .add_submenu(file_submenu)
        .add_submenu(locale_submenu);

    tauri::Builder::default()
        .plugin(tauri_plugin_aptabase::Builder::new(apta_key.as_str()).build())
        .plugin(tauri_plugin_log::Builder::default()
                .targets([
                         LogTarget::LogDir,
                         LogTarget::Stdout,
                ])
                .level(LevelFilter::Debug)
                .build())
        .menu(menu)
        .manage(TarkovStashState {
            state: Mutex::new(MutexState {
                bsg_items: None,
                globals: None,
                locale: None,
                profile_file_path: None,
                locale_lang: "en".to_string(),
            })

        })
        .on_menu_event(|event| match event.menu_item_id() {
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
            "locale_cz" | "locale_en" | "locale_fr" | "locale_ge" | "locale_hu" | "locale_it" | "locale_jp" | "locale_kr" | "locale_pl" | "locale_po" | "locale_sk" | "locale_es" | "locale_es-mx" | "locale_tu" | "locale_ru" => {
                let window = event.window();
                let menu_handle = window.menu_handle();
                let state: State<TarkovStashState>  = window.state();
                let mut internal_state = state.state.lock().unwrap();
                internal_state.locale_lang = event.menu_item_id().replace("locale_", "").to_string();
                let menu_item_id = event.menu_item_id().to_string();

                if internal_state.profile_file_path.is_some() {
                    window.emit("profile_loaded", "").expect("Can't emit event to window!");
                }


                std::thread::spawn(move || {
                    menu_handle.get_item("locale_cz").set_selected(false).expect("Can't find menu item for locale_cz");
                    menu_handle.get_item("locale_en").set_selected(false).expect("Can't find menu item for locale_en");
                    menu_handle.get_item("locale_fr").set_selected(false).expect("Can't find menu item for locale_fr");
                    menu_handle.get_item("locale_ge").set_selected(false).expect("Can't find menu item for locale_ge");
                    menu_handle.get_item("locale_hu").set_selected(false).expect("Can't find menu item for locale_hu");
                    menu_handle.get_item("locale_it").set_selected(false).expect("Can't find menu item for locale_it");
                    menu_handle.get_item("locale_jp").set_selected(false).expect("Can't find menu item for locale_jp");
                    menu_handle.get_item("locale_kr").set_selected(false).expect("Can't find menu item for locale_kr");
                    menu_handle.get_item("locale_pl").set_selected(false).expect("Can't find menu item for locale_pl");
                    menu_handle.get_item("locale_po").set_selected(false).expect("Can't find menu item for locale_po");
                    menu_handle.get_item("locale_sk").set_selected(false).expect("Can't find menu item for locale_sk");
                    menu_handle.get_item("locale_es").set_selected(false).expect("Can't find menu item for locale_es");
                    menu_handle.get_item("locale_es-mx").set_selected(false).expect("Can't find menu item for locale_es-mx");
                    menu_handle.get_item("locale_tu").set_selected(false).expect("Can't find menu item for locale_tu");
                    menu_handle.get_item("locale_ru").set_selected(false).expect("Can't find menu item for locale_ru");
                    menu_handle.get_item(&menu_item_id).set_selected(true).expect("Can't find selected menu item");
                });
            }
            _ => {}
        })
        .setup(|app| {
            app.track_event("app_started", None);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_profile_file, change_amount, change_fir, restore_durability, add_item, remove_item, add_preset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn load_profile_file(state: State<'_, TarkovStashState>) -> Result<UIProfile, String> {
    let mut internal_state = state.state.lock().unwrap();
    let b = &internal_state.profile_file_path;
    let b_clone = b.clone();
    let binding = b_clone.as_ref();
    match binding {
        Some(profile_file_path) => {
            if verify_spt_folder(profile_file_path) {
                create_backup(profile_file_path);
                // save to state internal data
                let locale = load_locale(profile_file_path, internal_state.locale_lang.clone());
                if locale.is_err() {
                    return Err(format!("Can't load your locale selection, please check that this file exists: [SPT]\\Aki_Data\\database\\locales\\global\\{}.json", internal_state.locale_lang.clone()));
                }
                let bsg_items = load_bsg_items(profile_file_path);
                let globals = load_globals(profile_file_path);
                let bsg_items_root: HashMap<String, Value> =
                    serde_json::from_str(bsg_items.as_str()).unwrap();
                let locale_root: HashMap<String, Value> =
                    serde_json::from_str(locale.unwrap().as_str()).unwrap();
                let globals_root: HashMap<String, Value> =
                    serde_json::from_str(globals.as_str()).unwrap();

                internal_state.locale = Some(locale_root);
                internal_state.bsg_items = Some(bsg_items_root);
                internal_state.globals = Some(globals_root);

                let content = fs::read_to_string(profile_file_path).unwrap();
                let tarkov_profile = spt_profile_serializer::load_profile(content.as_str());
                match tarkov_profile {
                    Ok(p) => {
                        let ui_profile_result = convert_profile_to_ui(
                            p,
                            internal_state.bsg_items.as_ref().unwrap(),
                            internal_state.locale.as_ref().unwrap(),
                            internal_state.globals.as_ref().unwrap(),
                        );
                        match ui_profile_result {
                            Ok(mut ui_profile) => {
                                ui_profile.spt_version =
                                    Some(get_server_version(profile_file_path));
                                Ok(ui_profile)
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e.to_string()),
                }
            } else {
                Err("I can't load your SPT files, your profile file must be located under SPT\\user\\profiles for me to work fine".to_string())
            }
        }
        None => Err("Could not file loaded into memory".to_string()),
    }
}

#[tauri::command]
async fn change_amount(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    app.track_event("change_amount", Some(json!({"item_id": item.id.as_str()})));
    info!("Changing amount to item {}", item.id.as_str());
    with_state_do(item, app, update_item_amount)
}

#[tauri::command]
async fn change_fir(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    app.track_event("change_fir", Some(json!({"item_id": item.id.as_str()})));
    info!("Setting fir to item {}", item.id.as_str());
    with_state_do(item, app, update_spawned_in_session)
}

#[tauri::command]
async fn restore_durability(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    app.track_event(
        "restore_durability",
        Some(json!({"item_id": item.id.as_str()})),
    );
    info!("Restoring durability to item {}", item.id.as_str());
    with_state_do(item, app, update_durability)
}

#[tauri::command]
async fn remove_item(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    app.track_event("remove_item", Some(json!({"item_id": item.id.as_str()})));
    info!("Deleting item {}", item.id.as_str());
    with_state_do(item, app, delete_item)
}

#[tauri::command]
async fn add_item(item: NewItem, app: tauri::AppHandle) -> Result<String, String> {
    app.track_event("add_item", Some(json!({"item_id": item.id.as_str()})));
    info!(
        "Adding item {} on [{},{}]",
        item.id.as_str(),
        item.location_x,
        item.location_y
    );
    let state: State<TarkovStashState> = app.state();
    let internal_state = state.state.lock().unwrap();
    let profile_file_path_option = &internal_state.profile_file_path;
    let profile_file_path = profile_file_path_option.as_ref().unwrap();
    let profile_content = fs::read_to_string(profile_file_path).unwrap();
    let bsg_items_option = &internal_state.bsg_items;
    let bsg_items = bsg_items_option.as_ref().unwrap();
    let response = add_new_item(
        profile_content.as_str(),
        item.id.as_str(),
        item.location_x,
        item.location_y,
        bsg_items,
    );
    match response {
        Ok(new_content) => {
            fs::write(profile_file_path, new_content).expect("Cant write profile file!");
            app.emit_all("profile_loaded", "")
                .expect("Can't emit event to window!");
            Ok("done".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn add_preset(item: NewItem, app: tauri::AppHandle) -> Result<String, String> {
    app.track_event("add_preset", Some(json!({"item_id": item.id.as_str()})));
    info!(
        "Adding preset id {} on [{},{}]",
        item.id.as_str(),
        item.location_x,
        item.location_y
    );
    let state: State<TarkovStashState> = app.state();
    let internal_state = state.state.lock().unwrap();
    let profile_file_path_option = &internal_state.profile_file_path;
    let profile_file_path = profile_file_path_option.as_ref().unwrap();
    let profile_content = fs::read_to_string(profile_file_path).unwrap();
    let globals_option = &internal_state.globals;
    let globals = globals_option.as_ref().unwrap();
    let response = add_new_preset(
        profile_content.as_str(),
        item.id.as_str(),
        item.location_x,
        item.location_y,
        globals,
    );
    match response {
        Ok(new_content) => {
            fs::write(profile_file_path, new_content).expect("Cant write profile file!");
            app.emit_all("profile_loaded", "")
                .expect("Can't emit event to window!");
            Ok("done".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

type UpdateFunction = fn(
    file_content: &str,
    item: &Item,
    bsg_items: &HashMap<String, Value>,
) -> Result<String, Error>;

fn with_state_do(item: Item, app: tauri::AppHandle, f: UpdateFunction) -> Result<String, String> {
    let state: State<TarkovStashState> = app.state();
    let internal_state = state.state.lock().unwrap();
    let profile_file_path_option = &internal_state.profile_file_path;
    let bsg_items_option = &internal_state.bsg_items;
    if profile_file_path_option.is_none() || bsg_items_option.is_none() {
        return Err("Could not find file inside app state".to_string());
    }
    let profile_file_path = profile_file_path_option.as_ref().unwrap();
    let bsg_items = bsg_items_option.as_ref().unwrap();
    let profile_content = fs::read_to_string(profile_file_path).unwrap();
    let new_profile = f(profile_content.as_str(), &item, bsg_items);
    match new_profile {
        Ok(new_content) => {
            fs::write(profile_file_path, new_content).expect("Cant write profile file!");
            app.emit_all("profile_loaded", "")
                .expect("Can't emit event to window!");
            Ok("updated".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn is_server_running() -> bool {
    TcpStream::connect("127.0.0.1:6969").is_ok()
}

fn get_server_version(file: &String) -> String {
    let core = Path::new(file)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("Aki_Data")
        .join("Server")
        .join("configs")
        .join("core.json");

    let core_json: Value =
        serde_json::from_str(fs::read_to_string(core).unwrap().as_str()).unwrap();
    core_json
        .get("akiVersion")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

fn create_backup(profile_path: &str) {
    let mut backup_number = 0;
    let mut backup_path = format!("{profile_path}.back.{backup_number}");
    while fs::metadata(&backup_path).is_ok() {
        backup_number += 1;
        backup_path = format!("{profile_path}.back.{backup_number}");
    }
    fs::copy(profile_path, backup_path).unwrap();
}

fn load_bsg_items(file: &String) -> String {
    let items = Path::new(file)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("Aki_Data")
        .join("Server")
        .join("database")
        .join("templates")
        .join("items.json");
    items.try_exists().expect(
        "Can't find `items.json` in your `SPT\\Aki_Data\\Server\\database\\templates\\items` folder",
    );
    info!("Reading bsg_items from {}", items.display());
    fs::read_to_string(items).unwrap()
}

fn load_globals(file: &String) -> String {
    let items = Path::new(file)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("Aki_Data")
        .join("Server")
        .join("database")
        .join("globals.json");
    items
        .try_exists()
        .expect("Can't find `globals.json` in your `SPT\\Aki_Data\\Server\\database` folder");
    info!("Reading globals from {}", items.display());
    fs::read_to_string(items).unwrap()
}

fn load_locale(file: &String, locale_menu_item: String) -> std::io::Result<String> {
    let locale_id = format!("{}.json", locale_menu_item);
    let locale = Path::new(file)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("Aki_Data")
        .join("Server")
        .join("database")
        .join("locales")
        .join("global")
        .join(locale_id.clone());
    fs::read_to_string(locale)
}

fn verify_spt_folder(profile_file_path: &String) -> bool {
    Path::new(profile_file_path)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("Aki_Data")
        .exists()
}

fn load_apta_key() -> String {
    // TODO if user opt-out this should return ""
    std::env::var("APTABASE_KEY").unwrap_or_else(|_r| {
        error!("Can't load APTABASE_KEY from environment");
        "".to_string()
    })
}
