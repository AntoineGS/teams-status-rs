mod ha_api;
mod teams_api;
mod teams_states;
mod utils;

use tray_item::{IconSource, TrayItem};

use crate::teams_api::TeamsAPI;
use dotenv::dotenv;
use ha_api::HAApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // let mut suspend = false;
    let ha_api = HAApi::new();
    let mut teams_api = TeamsAPI::new(ha_api);

    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("default-icon")).unwrap();

    tray.add_label("Teams Status").unwrap();

    tray.add_menu_item("Quit", || {
        // suspend = true;
        // let teams_api_ref = &Arc::try_unwrap(teams_api).expect("");
        // teams_api.socket.close(None).unwrap();
    })
    .unwrap();

    teams_api.listen_loop().await;

    // teams_api.socket.close(None).unwrap();
    // todo: wait for teams_api loop exit?

    Ok(())

    // todo: ensure Teams connection can be lost and reconnected since it is WS and not REST
    // todo: setup as a service
    // todo: logging
    // todo: implement back all configs from orig project
    // todo: try to trigger an initial status response
}
