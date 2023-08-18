// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::net::TcpStream;
use std::path::Path;
use std::sync::Mutex;

use serde_json::{Error, Value};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{CustomMenuItem, Manager, Menu, State, Submenu};

use crate::spt::spt_profile_serializer::load_profile;
use crate::stash::stash_utils::{update_item_amount, update_spawned_in_session};
use crate::ui_profile::ui_profile_serializer::{convert_profile_to_ui, Item, UIProfile};

pub mod spt;
pub mod stash;
pub mod ui_profile;

struct TarkovStashState {
    pub profile_file: Mutex<Option<String>>,
}

fn main() {
    let open = CustomMenuItem::new("open".to_string(), "Open profile");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let submenu = Submenu::new("File", Menu::new().add_item(open).add_item(quit));

    let menu = Menu::new()
        // .add_native_item(MenuItem::Quit)
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .manage(TarkovStashState {
            profile_file: Mutex::new(None),
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
                                *state.profile_file.lock().unwrap() =
                                    Some(p.as_path().to_str().unwrap().to_string());
                                window.emit("profile_loaded", "").expect("Can't emit event to window!");
                            }
                    });
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![load_profile_file, change_amount, change_fir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_profile_file(state: State<TarkovStashState>) -> Result<UIProfile, String> {
    let b = state.profile_file.lock().unwrap();
    let binding = b.as_ref();
    match binding {
        Some(file) => {
            create_backup(file);
            let bsg_items = load_bsg_items(file);
            let locale = load_locale(file);
            let content = fs::read_to_string(file).unwrap();
            let tarkov_profile = load_profile(content.as_str());
            match tarkov_profile {
                Ok(p) => {
                    let mut ui_profile =
                        convert_profile_to_ui(p, bsg_items.as_str(), locale.as_str());
                    ui_profile.spt_version = Some(get_server_version(file));
                    Ok(ui_profile)
                }
                Err(e) => Err(e.to_string()),
            }
        }
        None => Err("Could not file loaded into memory".to_string()),
    }
}

#[tauri::command]
fn change_amount(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    with_state_do(item, app, update_item_amount)
}

#[tauri::command]
fn change_fir(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    with_state_do(item, app, update_spawned_in_session)
}

fn with_state_do(
    item: Item,
    app: tauri::AppHandle,
    f: fn(file_content: &str, item: &Item) -> Result<String, Error>,
) -> Result<String, String> {
    let state: State<TarkovStashState> = app.state();
    let b = state.profile_file.lock().unwrap();
    let binding = b.as_ref();
    match binding {
        Some(file) => {
            let content = fs::read_to_string(file).unwrap();
            let new_content_result = f(content.as_str(), &item);
            match new_content_result {
                Ok(new_content) => {
                    fs::write(file, new_content).expect("Cant write profile file!");
                    app.emit_all("profile_loaded", "")
                        .expect("Can't emit event to window!");
                    Ok("updated".to_string())
                }
                Err(e) => Err(e.to_string()),
            }
        }
        None => Err("Could not find file inside app state".to_string()),
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
    fs::read_to_string(items).unwrap()
}

fn load_locale(file: &String) -> String {
    let items = Path::new(file)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("Aki_Data")
        .join("Server")
        .join("database")
        .join("locales")
        .join("global")
        .join("en.json");
    items.try_exists().expect(
        "Can't find `en.json` in your `SPT\\Aki_Data\\Server\\database\\locales\\global` folder",
    );
    fs::read_to_string(items).unwrap()
}
