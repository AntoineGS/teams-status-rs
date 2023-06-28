#![windows_subsystem = "windows"] // not sure how to get this working on linux atm
mod ha_api;
mod teams_api;
mod teams_states;
mod traits;
#[cfg(target_os = "linux")]
mod tray_linux;
#[cfg(target_os = "windows")]
mod tray_windows;
mod utils;

use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::teams_api::{start_listening, TeamsAPI};
#[cfg(target_os = "linux")]
use crate::tray_linux::create_tray;
#[cfg(target_os = "windows")]
use crate::tray_windows::create_tray;
use dotenv::dotenv;
use ha_api::HAApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let ha_api = Arc::new(HAApi::new());
    let teams_api = TeamsAPI::new();
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_me = is_running.clone();
    let _tray = create_tray(is_running_me);

    start_listening(
        ha_api.clone(),
        teams_api.teams_states.clone(),
        is_running,
        teams_api.url,
    )
    .await;

    exit(0)

    // todo: ensure Teams connection can be lost and reconnected since it is WS and not REST
    // todo: logging
    // todo: implement back all configs from orig project
    // todo: try to trigger an initial status response
}
