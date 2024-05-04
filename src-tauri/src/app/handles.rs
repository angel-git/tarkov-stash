use crate::prelude::server::Session;
use crate::prelude::{
    add_new_item, add_new_preset, convert_profile_to_ui, delete_item, track_event,
    update_durability, update_item_amount, update_spawned_in_session, Item, NewItem, UIProfile,
    SETTING_LOCALE,
};
use crate::spt::server::{
    is_server_running, is_tarkov_running, load_bsg_items_from_server, load_globals_from_server,
    load_locale_from_server, load_profile_from_server, load_server_info, load_sessions_from_server,
    refresh_profile_on_server, ServerProps,
};
use crate::TarkovStashState;
use log::info;
use serde_json::{json, Error, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager, State};

const SPT_RUNNING_ERROR: &str = "Looks like Tarkov is running, quit the game before using this mod";

#[tauri::command]
pub async fn connect_to_server(
    server: ServerProps,
    app: AppHandle,
) -> Result<Vec<Session>, String> {
    if !is_server_running(&server) {
        Err(format!("Server is not running at {}", server))
    } else if is_tarkov_running() {
        Err(SPT_RUNNING_ERROR.to_string())
    } else {
        {
            let state: State<TarkovStashState> = app.state();
            let mut internal_state = state.state.lock().unwrap();
            internal_state.server_props = Some(server.clone());
        }
        let server_info_result = load_server_info(&server).await;
        if server_info_result.is_ok() {
            let server_info = server_info_result.unwrap();
            let tauri_app_version = app.package_info().version.to_string();
            let spt_mod_version = server_info.mod_version;
            if tauri_app_version != spt_mod_version {
                return Err(format!(
                    "The version of the server mod [{}] does not match with mine [{}]. Make sure you are running the proper .exe file",
                    spt_mod_version, tauri_app_version
                ));
            }
            {
                let state: State<TarkovStashState> = app.state();
                let mut internal_state = state.state.lock().unwrap();
                internal_state.server_file_path = Some(server_info.path);
                internal_state.server_spt_version = Some(server_info.version)
            }

            load_sessions_from_server(&server)
                .await
                .map_err(|e| e.to_string())
        } else {
            Err("tarkov-stash is not installed in your [SPT\\user\\mods] folder.".to_string())
        }
    }
}

#[tauri::command]
pub async fn load_profile_from_spt(session: Session, app: AppHandle) -> Result<UIProfile, String> {
    if is_tarkov_running() {
        return Err(SPT_RUNNING_ERROR.to_string());
    }
    let state: State<TarkovStashState> = app.state();

    let server_props = {
        let internal_state = state.state.lock().unwrap();
        internal_state
            .server_props
            .clone()
            .expect("Server details not found!")
    };

    {
        let mut internal_state = state.state.lock().unwrap();
        internal_state.session_id = Some(session.id.clone());
    }

    let locale_from_settings = {
        let internal_state = state.state.lock().unwrap();
        let store = internal_state.store.as_ref().unwrap();
        store
            .get(SETTING_LOCALE)
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned()
    };

    let bsg_items = load_bsg_items_from_server(&server_props).await.unwrap();
    let globals = load_globals_from_server(&server_props).await.unwrap();
    let profile = load_profile_from_server(&server_props, &session)
        .await
        .unwrap();
    let locale = load_locale_from_server(&server_props, locale_from_settings.as_str())
        .await
        .unwrap();

    let ui_profile_result = convert_profile_to_ui(profile, &bsg_items, &locale, &globals);

    {
        let mut internal_state = state.state.lock().unwrap();
        internal_state.bsg_items = Some(bsg_items);
        internal_state.globals = Some(globals);
    }

    match ui_profile_result {
        Ok(mut ui_profile) => {
            let internal_state = state.state.lock().unwrap();
            ui_profile
                .spt_version
                .clone_from(&internal_state.server_spt_version);
            let server_file_path = internal_state.server_file_path.as_ref().unwrap();
            let session_id = internal_state.session_id.as_ref().unwrap();
            let profile_file = get_profile_filename(server_file_path, session_id);
            create_backup(profile_file.as_path().display().to_string().as_str());
            Ok(ui_profile)
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn refresh_profile_from_spt(
    session: Session,
    app: AppHandle,
) -> Result<UIProfile, String> {
    if is_tarkov_running() {
        return Err(SPT_RUNNING_ERROR.to_string());
    }
    let state: State<TarkovStashState> = app.state();

    let server_props = {
        let internal_state = state.state.lock().unwrap();
        internal_state
            .server_props
            .clone()
            .expect("Server details not found!")
    };

    let locale_from_settings = {
        let internal_state = state.state.lock().unwrap();
        let store = internal_state.store.as_ref().unwrap();
        store
            .get(SETTING_LOCALE)
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned()
    };

    let locale_root = {
        load_locale_from_server(&server_props, locale_from_settings.as_str())
            .await
            .unwrap()
    };

    {
        let mut internal_state = state.state.lock().unwrap();
        internal_state.session_id = Some(session.id.clone());
    }

    let profile = load_profile_from_server(&server_props, &session)
        .await
        .unwrap();

    let ui_profile_result = {
        let internal_state = state.state.lock().unwrap();
        let bsg_items = internal_state.bsg_items.as_ref().unwrap();
        let globals = internal_state.globals.as_ref().unwrap();
        convert_profile_to_ui(profile, bsg_items, &locale_root, globals)
    };

    match ui_profile_result {
        Ok(mut ui_profile) => {
            let internal_state = state.state.lock().unwrap();
            ui_profile
                .spt_version
                .clone_from(&internal_state.server_spt_version);
            Ok(ui_profile)
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn change_amount(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    if is_tarkov_running() {
        return Err(SPT_RUNNING_ERROR.to_string());
    }
    info!(
        "Changing amount to id {} and tpl {}",
        item.id.as_str(),
        item.tpl.as_str()
    );
    track_event(
        &app,
        "change_amount",
        Some(json!({"item_id": item.tpl.as_str()})),
    );
    with_state_do(item, &app, update_item_amount).await
}

#[tauri::command]
pub async fn change_fir(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    if is_tarkov_running() {
        return Err(SPT_RUNNING_ERROR.to_string());
    }
    info!(
        "Setting fir to item id {} and tpl {}",
        item.id.as_str(),
        item.tpl.as_str()
    );
    track_event(
        &app,
        "change_fir",
        Some(json!({"item_id": item.tpl.as_str()})),
    );
    with_state_do(item, &app, update_spawned_in_session).await
}

#[tauri::command]
pub async fn restore_durability(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    if is_tarkov_running() {
        return Err(SPT_RUNNING_ERROR.to_string());
    }
    info!(
        "Restoring durability to item {} and tpl {}",
        item.id.as_str(),
        item.tpl.as_str()
    );
    track_event(
        &app,
        "restore_durability",
        Some(json!({"item_id": item.tpl.as_str()})),
    );
    with_state_do(item, &app, update_durability).await
}

#[tauri::command]
pub async fn remove_item(item: Item, app: tauri::AppHandle) -> Result<String, String> {
    if is_tarkov_running() {
        return Err(SPT_RUNNING_ERROR.to_string());
    }
    info!(
        "Deleting item {} and tpl {}",
        item.id.as_str(),
        item.tpl.as_str()
    );
    track_event(
        &app,
        "remove_item",
        Some(json!({"item_id": item.tpl.as_str()})),
    );
    with_state_do(item, &app, delete_item).await
}

#[tauri::command]
pub async fn add_item(item: NewItem, app: tauri::AppHandle) -> Result<String, String> {
    if is_tarkov_running() {
        return Err(SPT_RUNNING_ERROR.to_string());
    }
    info!(
        "Adding {} item {} on [{},{}]",
        item.amount,
        item.id.as_str(),
        item.location_x,
        item.location_y,
    );
    track_event(&app, "add_item", Some(json!({"item_id": item.id.as_str()})));

    let profile_file_path = get_profile_file_path(&app);

    let response = {
        let state: State<TarkovStashState> = app.state();
        let internal_state = state.state.lock().unwrap();
        let profile_content = fs::read_to_string(profile_file_path.clone()).unwrap();
        let bsg_items_option = &internal_state.bsg_items;
        let bsg_items = bsg_items_option.as_ref().unwrap();

        add_new_item(
            profile_content.as_str(),
            item.id.as_str(),
            item.location_x,
            item.location_y,
            item.amount,
            bsg_items,
        )
    };

    match response {
        Ok(new_content) => {
            fs::write(&profile_file_path, new_content).expect("Cant write profile file!");
            refresh_profile(&app).await;
            Ok("done".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn add_preset(item: NewItem, app: tauri::AppHandle) -> Result<String, String> {
    if is_tarkov_running() {
        return Err(SPT_RUNNING_ERROR.to_string());
    }
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

    let profile_file_path = get_profile_file_path(&app);

    let response = {
        let state: State<TarkovStashState> = app.state();
        let internal_state = state.state.lock().unwrap();
        let profile_content = fs::read_to_string(profile_file_path.clone()).unwrap();
        let globals_option = &internal_state.globals;
        let globals = globals_option.as_ref().unwrap();
        add_new_preset(
            profile_content.as_str(),
            item.id.as_str(),
            item.location_x,
            item.location_y,
            globals,
        )
    };

    match response {
        Ok(new_content) => {
            fs::write(&profile_file_path, new_content).expect("Cant write profile file!");
            refresh_profile(&app).await;
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

async fn with_state_do(
    item: Item,
    app: &tauri::AppHandle,
    f: UpdateFunction,
) -> Result<String, String> {
    let result = {
        let state: State<TarkovStashState> = app.state();
        let internal_state = state.state.lock().unwrap();
        let server_file_path_option = internal_state.server_file_path.clone();
        let session_id_option = internal_state.session_id.clone();
        let bsg_items_option = internal_state.bsg_items.clone();
        if server_file_path_option.is_none()
            || bsg_items_option.is_none()
            || session_id_option.is_none()
        {
            return Err("Could not find file inside app state".to_string());
        }
        let session_id = session_id_option.as_ref().unwrap();
        let server_file_path = server_file_path_option.as_ref().unwrap();
        let profile_file_path = get_profile_filename(server_file_path, session_id);
        let bsg_items = bsg_items_option.as_ref().unwrap();
        let profile_content = fs::read_to_string(&profile_file_path).unwrap();
        let new_profile = f(profile_content.as_str(), &item, bsg_items);
        match new_profile {
            Ok(new_content) => {
                fs::write(&profile_file_path, new_content).expect("Cant write profile file!");
                Ok("updated".to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    };

    if result.is_ok() {
        refresh_profile(app).await;
    }
    result
}

fn get_profile_filename(server_file_path: &String, session_id: &String) -> PathBuf {
    Path::new(server_file_path)
        .join("user")
        .join("profiles")
        .join(format!("{}.json", session_id))
}

fn get_session_and_server(app: &tauri::AppHandle) -> (String, ServerProps) {
    let state: State<TarkovStashState> = app.state();
    let internal_state = state.state.lock().unwrap();

    let server_props_option = internal_state.server_props.clone();
    let session_id_option = internal_state.session_id.clone();

    (session_id_option.unwrap(), server_props_option.unwrap())
}

async fn refresh_profile(app: &tauri::AppHandle) {
    let (session_id, server_props) = get_session_and_server(app);
    let _ = refresh_profile_on_server(&server_props, &session_id).await;
    app.emit_all("profile_loaded", "")
        .expect("Can't emit event to window!");
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

fn get_profile_file_path(app: &AppHandle) -> PathBuf {
    let profile_file_path = {
        let state: State<TarkovStashState> = app.state();
        let internal_state = state.state.lock().unwrap();
        let session_id = internal_state.session_id.as_ref().unwrap();
        let server_file_path = internal_state.server_file_path.as_ref().unwrap();
        get_profile_filename(server_file_path, session_id)
    };
    profile_file_path
}
