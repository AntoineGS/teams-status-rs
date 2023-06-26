mod ha_api;
mod teams_api;
mod teams_states;
mod utils;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread::sleep;
use std::{thread, time};
use tray_item::{IconSource, TrayItem};

use crate::teams_api::{start_listening, TeamsAPI};
use dotenv::dotenv;
use ha_api::HAApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let ha_api = Arc::new(HAApi::new());
    let teams_api = TeamsAPI::new();
    // let (sender, receiver) = mpsc::sync_channel(1);
    let is_running = Arc::new(AtomicBool::new(true));

    let is_running_me = is_running.clone();

    #[cfg(target_os = "windows")]
    {
        let mut tray = TrayItem::new("Tray Example", IconSource::Resource("default-icon")).unwrap();
        tray.add_label("Teams Status").unwrap();

        tray.add_menu_item("Quit", move || {
            is_running_me.store(false, Ordering::Relaxed);
        })
        .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        let is_running_me = is_running.clone();
        thread::spawn(move || {
            let ten_seconds = time::Duration::from_millis(10000);
            sleep(ten_seconds);
            is_running_me.store(false, Ordering::Relaxed);
        });
    }

    start_listening(
        ha_api.clone(),
        teams_api.teams_states.clone(),
        is_running,
        teams_api.url,
    )
    .await;

    // teams_api.socket.close(None).unwrap();
    // todo: wait for teams_api loop exit?

    Ok(())

    // todo: ensure Teams connection can be lost and reconnected since it is WS and not REST
    // todo: setup as a service
    // todo: logging
    // todo: implement back all configs from orig project
    // todo: try to trigger an initial status response
}
