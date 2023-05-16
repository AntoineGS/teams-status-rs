mod ha_api;
mod teams_api;
mod teams_states;
mod utils;

use crate::teams_api::TeamsAPI;
use dotenv::dotenv;
use ha_api::HAApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let ha_api = HAApi::new();
    let mut teams_api = TeamsAPI::new(ha_api);
    teams_api.listen_loop().await;
    Ok(())

    // todo: ensure Teams connection can be lost and reconnected since it is WS and not REST
    // todo: setup as a service
    // todo: logging
    // todo: implement back all configs from orig project
    // todo: try to trigger an initial status response
}
