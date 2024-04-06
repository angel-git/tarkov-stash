use crate::prelude::{
    add_new_item, add_new_preset, convert_profile_to_ui, delete_item, spt_profile_serializer,
    track_event, update_durability, update_enable_add_items, update_item_amount,
    update_spawned_in_session, Item, NewItem, UIProfile, SETTING_LOCALE,
};
use crate::TarkovStashState;
use log::info;
use serde_json::{json, Error, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tauri::{Manager, State};

#[tauri::command]
pub async fn load_profile_file(
    state: State<'_, TarkovStashState>,
    window: tauri::Window,
) -> Result<UIProfile, String> {
    let mut internal_state = state.state.lock().unwrap();
    let b = &internal_state.profile_file_path;
    let b_clone = b.clone();
    let binding = b_clone.as_ref();
    match binding {
        Some(profile_file_path) => {
            if verify_spt_folder(profile_file_path) {
                create_backup(profile_file_path);
                // save to state internal data
                let locale_from_settings = internal_state
                    .store
                    .as_mut()
                    .unwrap()
                    .get(SETTING_LOCALE)
                    .unwrap()
                    .as_str()
                    .unwrap();
                let locale = load_locale(profile_file_path, locale_from_settings.to_string());
                if locale.is_err() {
                    return Err(format!("Can't load your locale selection, please check that this file exists: [SPT]\\Aki_Data\\database\\locales\\global\\{}.json", locale_from_settings));
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
                                drop(internal_state);
                                ui_profile.spt_version =
                                    Some(get_server_version(profile_file_path));
                                // let's disable add items feature if custom items are present
                                if !ui_profile.unknown_items.is_empty() {
                                    update_enable_add_items(&window, Some(false));
                                } else {
                                    update_enable_add_items(&window, Some(true));
                                }
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
pub async fn change_amount(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    info!("Changing amount to item {}", item.id.as_str());
    track_event(
        &app,
        "change_amount",
        Some(json!({"item_id": item.id.as_str()})),
    );
    with_state_do(item, app, update_item_amount)
}

#[tauri::command]
pub async fn change_fir(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    info!("Setting fir to item {}", item.id.as_str());
    track_event(
        &app,
        "change_fir",
        Some(json!({"item_id": item.id.as_str()})),
    );
    with_state_do(item, app, update_spawned_in_session)
}

#[tauri::command]
pub async fn restore_durability(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    info!("Restoring durability to item {}", item.id.as_str());
    track_event(
        &app,
        "restore_durability",
        Some(json!({"item_id": item.id.as_str()})),
    );
    with_state_do(item, app, update_durability)
}

#[tauri::command]
pub async fn remove_item(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    info!("Deleting item {}", item.id.as_str());
    track_event(
        &app,
        "remove_item",
        Some(json!({"item_id": item.id.as_str()})),
    );
    with_state_do(item, app, delete_item)
}

#[tauri::command]
pub async fn add_item(item: NewItem, app: tauri::AppHandle) -> Result<String, String> {
    info!(
        "Adding item {} on [{},{}]",
        item.id.as_str(),
        item.location_x,
        item.location_y
    );
    track_event(&app, "add_item", Some(json!({"item_id": item.id.as_str()})));
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
pub async fn add_preset(item: NewItem, app: tauri::AppHandle) -> Result<String, String> {
    info!(
        "Adding preset id {} on [{},{}]",
        item.id.as_str(),
        item.location_x,
        item.location_y
    );
    track_event(
        &app,
        "add_preset",
        Some(json!({"item_id": item.id.as_str()})),
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
