// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Manager, Menu, State, Submenu};
use tauri::api::dialog::FileDialogBuilder;
use std::fs;
use std::sync::Mutex;
use crate::spt_profile::spt_profile::load_profile;
use crate::ui_profile::ui_profile::{convert_profile_to_ui, UIProfile};

pub mod spt_profile;
pub mod ui_profile;

struct TarkovStashState {
    pub profile_file: Mutex<Option<String>>,
    pub json_content: Mutex<Option<String>>,
}

fn main() {
    let open = CustomMenuItem::new("open".to_string(), "Open profile");
    let restore_backup = CustomMenuItem::new("restore_backup".to_string(), "Restore backup");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let submenu = Submenu::new(
        "File",
        Menu::new()
            .add_item(open)
            .add_item(restore_backup)
            .add_item(quit),
    );

    let menu = Menu::new()
        // .add_native_item(MenuItem::Quit)
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .manage(TarkovStashState { profile_file: Mutex::new(None), json_content: Mutex::new(None) })
        // .setup(|app| {
        //     app.manage(AppState(Mutex::new(TarkovStashState { profile_file: None })));
        //     let main_window = app.get_window("main").unwrap();
        //
        //     let handle = app.handle();
        //     main_window.on_menu_event(move |event| {
        //         match event.menu_item_id() {
        //             "quit" => {
        //                 std::process::exit(0);
        //             }
        //             "open" => {
        //                 FileDialogBuilder::default()
        //                     .add_filter("json", &["json"])
        //                     .pick_file(move |path_buf| match path_buf {
        //                         Some(p) => {
        //                             let window = event.borrow().;
        //
        //                             // get file name: p.file_name().unwrap().to_str().unwrap()
        //                             // path + filename: p.as_path().to_str().unwrap()
        //                             let content = fs::read_to_string(p.as_path()).expect("Unable to read file");
        //                             match load_profile_file(content.as_str()) {
        //                                 Ok(ui_profile) => {
        //                                     main_window.emit("error", "huehuee");
        //                                     // window.emit("profile_loaded", ui_profile);
        //                                 }
        //                                 Err(err) => {}
        //                             }
        //                         }
        //                         _ => {}
        //                     });
        //             }
        //             _ => {}
        //         }
        //     });
        //
        //     Ok(())
        // })
        .on_menu_event(|event| {

            match event.menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                "open" => {
                    FileDialogBuilder::default()
                        .add_filter("json", &["json"])
                        .pick_file(move |path_buf| match path_buf {
                            Some(p) => {
                                let window = event.window();
                                let state: State<TarkovStashState> = window.state();
                                // let content = fs::read_to_string(p.as_path()).expect("Unable to read file");
                                *state.profile_file.lock().unwrap() = Some(p.as_path().to_str().unwrap().to_string());
                                window.emit("profile_loaded", "");
                                // match load_profile_file(content.as_str()) {
                                //     Ok(ui_profile) => {
                                //         state.set_profile_file(p.as_path().to_str().unwrap().to_string());
                                //         // window.emit("error", "huehuee");
                                //         // window.emit("profile_loaded", ui_profile);
                                //         window.emit("profile_loaded", "");
                                //     }
                                //     Err(err) => {
                                //
                                //     }
                                // }
                            }
                            _ => {}
                        });
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![load_profile_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[tauri::command]
// fn load_profile_file(content: &str) -> Result<UIProfile, String> {
//     let tarkov_profile = load_profile(content);
//     let res = match tarkov_profile {
//         Ok(p) => Ok(convert_profile_to_ui(p)),
//         Err(e) => Err(e.to_string()),
//     };
//     res
// }

#[tauri::command]
fn load_profile_file(state: State<TarkovStashState>) -> Result<UIProfile, String> {
    let b = state.profile_file.lock().unwrap();
    let binding = b.as_ref();
    match binding {
        Some(file) => {
            let content = fs::read_to_string(file).unwrap();
            *state.json_content.lock().unwrap() = Some(content.clone());
            let tarkov_profile = load_profile(content.as_str());
            let res = match tarkov_profile {
                Ok(p) => {
                    Ok(convert_profile_to_ui(p))
                },
                Err(e) => Err(e.to_string()),
            };
            res
        }
        None => {
            Err("Could not file loaded into memory".to_string())
        }
    }
}
