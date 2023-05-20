mod ha_api;
mod teams_api;
mod teams_states;
mod utils;

use std::sync::{mpsc, Arc};
use tray_item::{IconSource, TrayItem};

use crate::teams_api::{start_listening, TeamsAPI};
use dotenv::dotenv;
use ha_api::HAApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let ha_api = Arc::new(HAApi::new());
    let teams_api = TeamsAPI::new();
    let (sender, receiver) = mpsc::sync_channel(1);

    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("default-icon")).unwrap();

    tray.add_label("Teams Status").unwrap();

    tray.add_menu_item("Quit", move || {
        sender.send(true).unwrap();
    })
    .unwrap();

    start_listening(
        ha_api.clone(),
        teams_api.teams_states.clone(),
        receiver,
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
