use notch::servers::runner::Runner;
use notch::servers::server::Server;
use crate::config::{Config, ServerInfo};
use crate::error::Error;

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
    let dname_arg = format!("-Dname={}", name);
    let mut runner = Runner::new(&server, vec![dname_arg.as_str()], vec![]);
    let mut child = runner.start().expect("Failed to start server.");
    let new_thread = std::thread::spawn(move || {
        child.wait().expect("Failed to wait for server.");
    });
    Ok(())
}

#[tauri::command]
pub fn stop_server(name: String) -> Result<(), Error> {
    let config = Config::load()?;
    let server = config.get_server(&name).ok_or(Error::ServerNotFound)?;
    let jps_output = std::process::Command::new("jps").arg("-v").output()?;
    let jps_output = String::from_utf8(jps_output.stdout).unwrap_or_default();
    let running = jps_output.contains(format!("-Dname={}", server.name).as_str());
    if running {
        let mut command = std::process::Command::new("jps");
        command.arg("-v");
        let output = command.output()?;
        let output = String::from_utf8(output.stdout).unwrap_or_default();
        let pid = output
            .lines()
            .find(|l| l.contains(format!("-Dname={}", server.name).as_str()))
            .unwrap()
            .split(' ')
            .next()
            .unwrap();
        let process = std::process::Command::new("kill").arg("-SIGINT").arg(pid).spawn()?;
        let output = process.wait_with_output()?;
        println!("{}", String::from_utf8(output.stdout).unwrap_or_default());
    }
    Ok(())
}

#[tauri::command]
pub fn rename_server(old_name: String, new_name: String) -> Result<(), Error> {
    let mut config = Config::load()?;
    let server = config.servers.iter_mut().find(|s| s.server_name == old_name).ok_or(Error::ServerNotFound)?;
    server.server_name = new_name.clone();
    let mut physical_server = Server::from_path(&server.location)?;
    physical_server.name = new_name;
    physical_server.save()?;
    config.save()?;
    Ok(())
}

#[tauri::command]
pub fn is_server_running(name: String) -> Result<bool, Error> {
    let config = Config::load()?;
    let jps_output = std::process::Command::new("jps").arg("-v").output()?;
    let jps_output = String::from_utf8(jps_output.stdout).unwrap_or_default();
    let server = config.get_server(&name).ok_or(Error::ServerNotFound)?;
    let running = jps_output.contains(format!("-Dname={}", server.name).as_str());
    Ok(running)
}

#[tauri::command]
pub fn delete_server(name: String) -> Result<(), Error> {
    let mut config = Config::load()?;
    let server = config.servers.iter().find(|s| s.server_name == name).ok_or(Error::ServerNotFound)?;
    let physical_server = Server::from_path(&server.location)?;
    physical_server.delete()?;
    config.servers.remove(
        config.servers
            .iter()
            .position(|s| s.server_name == name)
            .ok_or(Error::ServerNotFound)?
    );
    config.save()?;
    Ok(())
}