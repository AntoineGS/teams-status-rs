#![windows_subsystem = "windows"]

mod configuration;
mod home_assistant;
mod logging;
mod mqtt;
mod mutex;
mod teams_ws;
mod traits;
mod tray;
mod utils;

use mutex::{create_mutex, release_mutex};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time;
use tokio::sync::Mutex;

use crate::configuration::get_configuration;
use crate::logging::initialize_logging;
use crate::mqtt::api::MqttApi;
use crate::teams_ws::api::TeamsAPI;
use crate::traits::Listener;
use crate::tray::create_tray;
use anyhow::Result;
use home_assistant::api::HaApi;
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logging();
    info!("--------------------");
    info!("Application starting");

    let mutex = create_mutex();

    if mutex.is_none() {
        exit(1)
    }

    // to toggle mute from the tray icon, and let Teams allow the application to listen to its websocket
    let toggle_mute = Arc::new(AtomicBool::new(false));
    // used by tray icon to allow exiting the application
    let is_running = Arc::new(AtomicBool::new(true));
    let _tray = create_tray(is_running.clone(), toggle_mute.clone());
    let five_seconds = time::Duration::from_secs(5);
    let mut save_configuration = true;

    // Aggressive to re-create the connections, but it will handle all APIs, ideal way would
    // be to structure to app so that each API has its own loop and message queue, so when it
    // comes back online it would pick up the items from the queue and process them.
    while is_running.load(Ordering::Relaxed) {
        let result = run_apis(is_running.clone(), toggle_mute.clone(), save_configuration).await;
        save_configuration = false;

        if result.is_err() {
            result.unwrap_or_else(|error| error!("Error encountered: {}", error));

            // Give the CPU/user/APIs some time to recover
            if is_running.load(Ordering::Relaxed) {
                tokio::time::sleep(five_seconds).await;
            }
        }
    }

    info!("Application closing");

    release_mutex(mutex);
    exit(0);
}

async fn run_apis(
    is_running: Arc<AtomicBool>,
    toggle_mute: Arc<AtomicBool>,
    save_configuration: bool,
) -> Result<()> {
    let conf = get_configuration(save_configuration);
    let teams_api = TeamsAPI::new(&conf.teams);
    let listener: Box<dyn Listener> = if conf.mqtt.url().is_empty() {
        Box::new(HaApi::new(conf.ha)?)
    } else {
        Box::new(MqttApi::new(conf.mqtt)?)
    };

    teams_api
        .start_listening(
            Arc::new(Mutex::new(listener)),
            is_running.clone(),
            toggle_mute.clone(),
        )
        .await?;

    Ok(())
}
