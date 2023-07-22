// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod spt_profile;
pub mod ui_profile;

use crate::spt_profile::spt_profile::load_profile;
use crate::ui_profile::ui_profile::{convert_profile_to_ui, UIProfile};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_profile_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_profile_file(content: &str) -> Result<UIProfile, String> {
    let tarkov_profile = load_profile(content);
    let res = match tarkov_profile {
        Ok(p) => Ok(convert_profile_to_ui(p)),
        _ => Err("whops".into()),
    };
    res
}
