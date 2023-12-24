use crate::config::{Config, ServerInfo};
use crate::error::Error;
use crate::minecraft::server::Server;

#[tauri::command]
pub fn get_servers() -> Vec<ServerInfo> {
    let config = Config::load().expect("Failed to get config.");
    config.servers
}

#[tauri::command]
pub fn get_server(name: String) -> Option<Server> {
    let config = Config::load().expect("Failed to get config.");
    config.get_server(&name)
}

#[tauri::command]
pub fn start_server(name: String) -> Result<(), Error> {
    let config = Config::load()?;
    let server = config.get_server(&name).ok_or(Error::ServerNotFound)?;
    server.run()
}