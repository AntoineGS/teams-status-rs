mod ha_api;
mod teams_api;
mod teams_states;
mod utils;

use std::rc::Rc;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use {
    std::sync::mpsc,
    tray_item::{IconSource, TrayItem},
};

use crate::teams_api::TeamsAPI;
use dotenv::dotenv;
use ha_api::HAApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // let mut suspend = false;
    let ha_api = HAApi::new();
    let teams_api = Arc::new(TeamsAPI::new(ha_api));

    let mut tray = TrayItem::new(
        "Tray Example",
        IconSource::Resource("name-of-icon-in-rc-file"),
    )
    .unwrap();

    tray.add_label("Teams Status").unwrap();

    tray.add_menu_item("Quit", || {
        // suspend = true;
        Arc::try_unwrap(teams_api)
            .unwrap()
            .socket
            .close(None)
            .unwrap();
    })
    .unwrap();

    Arc::try_unwrap(teams_api).unwrap().listen_loop().await;

    // teams_api.socket.close(None).unwrap();
    // todo: wait for teams_api loop exit?

    Ok(())

    // todo: ensure Teams connection can be lost and reconnected since it is WS and not REST
    // todo: setup as a service
    // todo: logging
    // todo: implement back all configs from orig project
    // todo: try to trigger an initial status response
}
