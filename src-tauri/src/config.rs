#![warn(
    clippy::cognitive_complexity,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::enum_glob_use,
    clippy::pedantic,
    clippy::complexity
)]

use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use notch::servers::server::Server;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub servers: Vec<ServerInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerInfo {
    pub server_name: String,
    pub location: PathBuf,
}

impl ServerInfo {
    pub fn from_server(server: Server) -> Self {
        Self {
            server_name: server.name,
            location: server.location,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let mut config: Self = confy::load("boxes", None)?;
        let mut changed = false;
        for server in &config.clone().servers {
            if !server.location.exists() {
                println!(
                    "⚠️ Server {} does not exist! Removing from config...",
                    server.server_name
                );
                config.servers.remove(
                    config
                        .servers
                        .iter()
                        .position(|s| s.location == server.location)
                        .unwrap(),
                );
                println!("⚠️ Server {} was removed from config!", server.server_name);
                changed = true;
            }
        }
        if changed {
            confy::store("boxes", None, config.clone())?;
        }
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Error> {
        println!("📝 Saving config...");
        confy::store("boxes", None, self)?;
        println!("📝 Saved config!");
        Ok(())
    }

    pub fn delete() -> Result<(), Error> {
        println!("📝 Deleting config...");
        let path = confy::get_configuration_file_path("config", None)?;
        fs::remove_file(path).expect("🚨 Config file could not be deleted!");
        println!("📝 Deleted config!");
        Ok(())
    }

    pub fn add_server(&mut self, server: Server) {
        println!("📝 Adding server to config...");
        if self.servers.iter().any(|s| s.location == server.location) {
            println!("⚠️ A server with the same location already exists! Overriding...");
            self.servers.remove(
                self.servers
                    .iter()
                    .position(|s| s.location == server.location)
                    .ok_or(Error::ResourceNotFound("Server not found".to_string()))
                    .unwrap(),
            );
            println!("⚠️ A server was overridden!");
        }
        let server_info = ServerInfo::from_server(server);
        self.servers.push(server_info);
        confy::store("boxes", None, self).expect("🚨 Config file could not be saved!");
        println!("📝 Added server to config!");
    }

    pub fn get_server(&self, server_name: &str) -> Option<Server> {
        let server_info = self
            .servers
            .iter()
            .find(|s| s.server_name.to_lowercase() == server_name.to_lowercase())?;
        Server::from_path(server_info.location.as_path()).ok()
    }

    pub fn save_server(&mut self, server: Server) -> Result<(), Error> {
        println!("📝 Saving server to config...");
        let index = self
            .servers
            .iter()
            .position(|s| s.server_name == server.name)
            .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
        self.servers.remove(index);
        let server_info = ServerInfo::from_server(server);
        self.servers.push(server_info);
        confy::store("boxes", None, self)?;
        println!("📝 Saved server to config!");
        Ok(())
    }
}
