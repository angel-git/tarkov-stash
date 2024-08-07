use crate::prelude::{ClientBuilder, Deserialize, HttpRequestBuilder, Serialize};
use crate::spt::spt_profile_serializer::TarkovProfile;
use log::error;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::net::TcpStream;
use std::string::ToString;
use sysinfo::System;

const TARKOV_PROCESS: &str = "EscapeFromTarkov.exe";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerProps {
    host: String,
    port: u16,
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
        write!(fmt, "{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub id: String,
    pub username: String,
}

pub fn is_server_running(server_props: &ServerProps) -> bool {
    TcpStream::connect(server_props.to_string()).is_ok()
}

pub fn is_tarkov_running() -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.processes_by_name(TARKOV_PROCESS).count() > 0
}

pub async fn load_server_info(server_props: &ServerProps) -> Result<ServerInfo, tauri::Error> {
    let client = ClientBuilder::new().max_redirections(3).build().unwrap();

    let request = HttpRequestBuilder::new(
        "GET",
        format!("http://{}/tarkov-stash/server", server_props),
    )
    .unwrap();
    let request = request.header("responsecompressed", "0").unwrap();
    if let Ok(response) = serde_json::from_value(client.send(request).await?.read().await?.data) {
        Ok(response)
    } else {
        Err(tauri::Error::Io(std::io::Error::from(
            std::io::ErrorKind::NotFound,
        )))
    }
}

pub async fn load_sessions_from_server(
    server_props: &ServerProps,
) -> Result<Vec<Session>, tauri::Error> {
    let client = ClientBuilder::new().max_redirections(3).build().unwrap();

    let request = HttpRequestBuilder::new(
        "GET",
        format!("http://{}/tarkov-stash/profiles", server_props),
    )
    .unwrap();
    let request = request.header("responsecompressed", "0").unwrap();
    Ok(parse_sessions(
        client.send(request).await?.read().await?.data,
    ))
}

pub async fn load_bsg_items_from_server(
    server_props: &ServerProps,
) -> Result<HashMap<String, Value>, tauri::Error> {
    let client = ClientBuilder::new().max_redirections(3).build().unwrap();

    let request =
        HttpRequestBuilder::new("GET", format!("http://{}/tarkov-stash/items", server_props))
            .unwrap();
    let request = request.header("responsecompressed", "0").unwrap();
    Ok(parse_as_map(client.send(request).await?.read().await?.data))
}

pub async fn load_globals_from_server(
    server_props: &ServerProps,
) -> Result<HashMap<String, Value>, tauri::Error> {
    let client = ClientBuilder::new().max_redirections(3).build().unwrap();

    let request = HttpRequestBuilder::new(
        "GET",
        format!("http://{}/tarkov-stash/globals-presets", server_props),
    )
    .unwrap();
    let request = request.header("responsecompressed", "0").unwrap();
    let globals_value = serde_json::to_value(client.send(request).await?.read().await?.data)
        .expect("Can't parse globals from server");
    let mut globals: HashMap<String, Value> = HashMap::new();
    // for backwards compatibility, maybe to remove this in future
    globals.insert("ItemPresets".to_string(), globals_value);
    Ok(globals)
}

pub async fn load_profile_from_server(
    server_props: &ServerProps,
    session: &Session,
) -> Result<TarkovProfile, tauri::Error> {
    let client = ClientBuilder::new().max_redirections(3).build().unwrap();

    let request = HttpRequestBuilder::new(
        "GET",
        format!("http://{}/tarkov-stash/profile", server_props),
    )
    .unwrap();
    let request = request.header("responsecompressed", "0").unwrap();
    let request = request
        .header("Cookie", format!("PHPSESSID={}", session.id))
        .unwrap();
    Ok(
        serde_json::from_value(client.send(request).await?.read().await?.data)
            .expect("Can't parse profile"),
    )
}

pub async fn load_locale_from_server(
    server_props: &ServerProps,
    locale: &str,
) -> Result<HashMap<String, Value>, tauri::Error> {
    let client = ClientBuilder::new().max_redirections(3).build().unwrap();

    let request = HttpRequestBuilder::new(
        "GET",
        format!("http://{}/client/locale/{}", server_props, locale),
    )
    .unwrap();
    let request = request.header("responsecompressed", "0").unwrap();
    Ok(parse_locale(client.send(request).await?.read().await?.data))
}

pub async fn refresh_profile_on_server(
    server_props: &ServerProps,
    session_id: &String,
) -> Result<String, tauri::Error> {
    let client = ClientBuilder::new().max_redirections(3).build().unwrap();

    let request = HttpRequestBuilder::new(
        "GET",
        format!("http://{}/tarkov-stash/reload-profile", server_props),
    )
    .unwrap();
    let request = request.header("responsecompressed", "0").unwrap();
    let request = request
        .header("Cookie", format!("PHPSESSID={}", session_id))
        .unwrap();
    Ok(
        serde_json::from_value(client.send(request).await?.read().await?.data)
            .expect("Can't parse refresh profile"),
    )
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
