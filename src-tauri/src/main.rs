// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{info, LevelFilter};
use std::collections::HashMap;
use std::fs;
use std::net::TcpStream;
use std::path::Path;
use std::sync::Mutex;

use serde_json::{Error, Value};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{CustomMenuItem, Manager, Menu, State, Submenu};
use tauri_plugin_log::LogTarget;

use crate::spt::spt_profile_serializer::load_profile;
use crate::stash::stash_utils::{
    add_new_item, delete_item, update_durability, update_item_amount, update_spawned_in_session,
    NewItem,
};
use crate::ui_profile::ui_profile_serializer::{convert_profile_to_ui, Item, UIProfile};

pub mod spt;
pub mod stash;
pub mod ui_profile;
pub mod utils;

struct TarkovStashState {
    pub state: Mutex<MutexState>,
}

struct MutexState {
    pub profile_file_path: Option<String>,
    pub bsg_items: Option<HashMap<String, Value>>,
    pub globals: Option<HashMap<String, Value>>,
    pub locale: Option<HashMap<String, Value>>,
}

fn main() {
    let open = CustomMenuItem::new("open".to_string(), "Open profile");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let submenu = Submenu::new("File", Menu::new().add_item(open).add_item(quit));

    let menu = Menu::new()
        // .add_native_item(MenuItem::Quit)
        .add_submenu(submenu);

    tauri::Builder::default()
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
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![load_profile_file, change_amount, change_fir, restore_durability, add_item, remove_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_profile_file(state: State<TarkovStashState>) -> Result<UIProfile, String> {
    let mut internal_state = state.state.lock().unwrap();
    let b = &internal_state.profile_file_path;
    let b_clone = b.clone();
    let binding = b_clone.as_ref();
    match binding {
        Some(profile_file_path) => {
            if verify_spt_folder(profile_file_path) {
                create_backup(profile_file_path);
                // save to state internal data
                let locale = load_locale(profile_file_path);
                let bsg_items = load_bsg_items(profile_file_path);
                let globals = load_globals(profile_file_path);
                let bsg_items_root: HashMap<String, Value> =
                    serde_json::from_str(bsg_items.as_str()).unwrap();
                let locale_root: HashMap<String, Value> =
                    serde_json::from_str(locale.as_str()).unwrap();
                let globals_root: HashMap<String, Value> =
                    serde_json::from_str(globals.as_str()).unwrap();

                internal_state.locale = Some(locale_root);
                internal_state.bsg_items = Some(bsg_items_root);
                internal_state.globals = Some(globals_root);

                let content = fs::read_to_string(profile_file_path).unwrap();
                let tarkov_profile = load_profile(content.as_str());
                match tarkov_profile {
                    Ok(p) => {
                        let ui_profile_result = convert_profile_to_ui(
                            p,
                            internal_state.bsg_items.as_ref().unwrap(),
                            internal_state.locale.as_ref().unwrap(),
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
fn change_amount(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    info!("Changing amount to item {}", item.id.as_str());
    with_state_do(item, app, update_item_amount)
}

#[tauri::command]
fn change_fir(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    info!("Setting fir to item {}", item.id.as_str());
    with_state_do(item, app, update_spawned_in_session)
}

#[tauri::command]
fn restore_durability(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    info!("Restoring durability to item {}", item.id.as_str());
    with_state_do(item, app, update_durability)
}

#[tauri::command]
fn remove_item(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    info!("Deleting item {}", item.id.as_str());
    with_state_do(item, app, delete_item)
}

#[tauri::command]
fn add_item(item: NewItem, app: tauri::AppHandle) -> Result<String, String> {
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

fn load_locale(file: &String) -> String {
    let locale = Path::new(file)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("Aki_Data")
        .join("Server")
        .join("database")
        .join("locales")
        .join("global")
        .join("en.json");
    locale.try_exists().expect(
        "Can't find `en.json` in your `SPT\\Aki_Data\\Server\\database\\locales\\global` folder",
    );
    info!("Reading locale from {}", locale.display());
    fs::read_to_string(locale).unwrap()
}

fn verify_spt_folder(profile_file_path: &String) -> bool {
    Path::new(profile_file_path)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("Aki_Data")
        .exists()
}
