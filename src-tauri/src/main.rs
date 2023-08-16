// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::net::TcpStream;
use std::sync::Mutex;

use tauri::api::dialog::FileDialogBuilder;
use tauri::{CustomMenuItem, Manager, Menu, State, Submenu};

use crate::spt_profile::spt_profile::load_profile;
use crate::ui_profile::ui_profile::{convert_profile_to_ui, Item, UIProfile};
use crate::utils::utils::update_item_amount;

pub mod spt_profile;
pub mod ui_profile;
pub mod utils;

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
                    .pick_file(move |path_buf| match path_buf {
                        Some(p) => {
                            let window = event.window();
                            let is_server_running = check_if_server_is_running();
                            if is_server_running {
                                window.emit("error", "Looks like your server is running, please stop it and try to open your profile again").expect("Can't emit event to window!");
                            } else {
                                let state: State<TarkovStashState> = window.state();
                                *state.profile_file.lock().unwrap() =
                                    Some(p.as_path().to_str().unwrap().to_string());
                                window.emit("profile_loaded", "").expect("Can't emit event to window!");
                            }
                        }
                        _ => {}
                    });
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![load_profile_file, change_amount])
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
            let content = fs::read_to_string(file).unwrap();
            let tarkov_profile = load_profile(content.as_str());
            let res = match tarkov_profile {
                Ok(p) => Ok(convert_profile_to_ui(p)),
                Err(e) => Err(e.to_string()),
            };
            res
        }
        None => Err("Could not file loaded into memory".to_string()),
    }
}

#[tauri::command]
fn change_amount(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    let state: State<TarkovStashState> = app.state();
    let b = state.profile_file.lock().unwrap();
    let binding = b.as_ref();
    match binding {
        Some(file) => {
            let content = fs::read_to_string(file).unwrap();
            let new_content_result =
                update_item_amount(content.as_str(), item.id.as_str(), item.amount);
            match new_content_result {
                Ok(new_content) => {
                    fs::write(file, new_content).expect("Cant write profile file!");
                    app.emit_all("profile_loaded", "")
                        .expect("Can't emit event to window!");
                    Ok("huehue".to_string())
                }
                Err(e) => Err(e.to_string()),
            }
        }
        None => Err("Could not find file inside app state".to_string()),
    }
}

fn check_if_server_is_running() -> bool {
    TcpStream::connect("127.0.0.1:6969").is_ok()
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
