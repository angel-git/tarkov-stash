use crate::prelude::{Deserialize, Serialize};
use crate::spt::spt_profile_serializer::TarkovProfile;
use log::error;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::string::ToString;
use sysinfo::System;

const TARKOV_PROCESS: &str = "EscapeFromTarkov.exe";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerProps {
    address: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerInfo {
    pub version: String,
    pub path: String,
    #[serde(rename = "modVersion")]
    pub mod_version: String,
}

impl fmt::Display for ServerProps {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.address)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub id: String,
    pub username: String,
}

fn get_client() -> reqwest::Client {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
}

pub async fn is_server_running(server_props: &ServerProps) -> bool {
    get_client()
        .get(server_props.address.to_string())
        .send()
        .await
        .is_ok()
}

pub fn is_tarkov_running() -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.processes_by_name(TARKOV_PROCESS).count() > 0
}

pub async fn load_server_info(server_props: &ServerProps) -> Result<ServerInfo, tauri::Error> {
    let value = do_get(server_props, "tarkov-stash/server", None).await?;
    match serde_json::from_value::<ServerInfo>(value) {
        Ok(server_info) => Ok(server_info),
        Err(_) => Err(tauri::Error::Io(std::io::Error::from(
            std::io::ErrorKind::NotFound,
        ))),
    }
}

pub async fn load_sessions_from_server(
    server_props: &ServerProps,
) -> Result<Vec<Session>, tauri::Error> {
    let value = do_get(server_props, "tarkov-stash/profiles", None).await?;
    Ok(parse_sessions(value))
}

pub async fn load_bsg_items_from_server(
    server_props: &ServerProps,
) -> Result<HashMap<String, Value>, tauri::Error> {
    let value = do_get(server_props, "tarkov-stash/items", None).await?;
    Ok(parse_as_map(value))
}

pub async fn load_globals_from_server(
    server_props: &ServerProps,
) -> Result<HashMap<String, Value>, tauri::Error> {
    let value = do_get(server_props, "tarkov-stash/globals-presets", None).await?;
    let mut globals: HashMap<String, Value> = HashMap::new();
    // for backwards compatibility, maybe to remove this in future
    globals.insert("ItemPresets".to_string(), value);
    Ok(globals)
}

pub async fn load_profile_from_server(
    server_props: &ServerProps,
    session: &Session,
) -> Result<TarkovProfile, tauri::Error> {
    let value = do_get(server_props, "tarkov-stash/profile", Some(&session.id)).await?;
    Ok(serde_json::from_value(value).expect("Can't parse profile"))
}

pub async fn load_locale_from_server(
    server_props: &ServerProps,
    locale: &str,
) -> Result<HashMap<String, Value>, tauri::Error> {
    let value = do_get(
        server_props,
        format!("client/locale/{}", locale).as_str(),
        None,
    )
    .await?;
    Ok(parse_locale(value))
}

pub async fn refresh_profile_on_server(
    server_props: &ServerProps,
    session_id: &String,
) -> Result<String, tauri::Error> {
    let value = do_get(
        server_props,
        "tarkov-stash/reload-profile",
        Some(session_id),
    )
    .await?;
    Ok(serde_json::from_value(value).expect("Can't parse refresh profile"))
}

async fn do_get(
    server_props: &ServerProps,
    path: &str,
    session: Option<&String>,
) -> Result<Value, tauri::Error> {
    let client = get_client();

    let mut headers = HeaderMap::new();
    headers.insert("responsecompressed", "0".parse().unwrap());
    if session.is_some() {
        headers.insert(
            "Cookie",
            format!("PHPSESSID={}", session.unwrap()).parse().unwrap(),
        );
    }
    let response = client
        .get(format!("{}/{}", server_props, path))
        .header("responsecompressed", "0")
        .headers(headers)
        .send()
        .await;

    match response {
        Ok(response) => Ok(serde_json::from_slice::<Value>(
            &response.bytes().await.unwrap(),
        )?),
        Err(_) => Err(tauri::Error::Io(std::io::Error::from(
            std::io::ErrorKind::NotFound,
        ))),
    }
}

fn parse_sessions(json_value: Value) -> Vec<Session> {
    let objects = json_value.as_object().expect("Whops, can't read profiles!");
    let mut sessions = Vec::new();
    for (_, value) in objects {
        let profile_result: serde_json::Result<TarkovProfile> =
            serde_json::from_value(value.clone());
        match profile_result {
            Ok(profile) => {
                let session = Session {
                    id: profile.info.id,
                    username: profile.info.username,
                };
                sessions.push(session);
            }
            Err(e) => {
                error!("Couldn't not load your profile/s, be sure that you have 'menu played' it/them before: {}", e);
            }
        }
    }
    sessions
}

fn parse_as_map(json_value: Value) -> HashMap<String, Value> {
    serde_json::from_value(json_value).expect("Whops, can't read bsg_items from api!")
}

fn parse_locale(json_value: Value) -> HashMap<String, Value> {
    serde_json::from_value(json_value.as_object().unwrap().get("data").unwrap().clone())
        .expect("Whops, can't read locales from api")
}
