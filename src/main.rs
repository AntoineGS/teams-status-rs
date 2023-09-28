mod ha_api;
mod teams_api;
mod teams_states;
mod traits;
mod tray_windows;
mod utils;

use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::teams_api::{start_listening, TeamsAPI};
use crate::tray_windows::create_tray;
use dotenv::dotenv;
use ha_api::HAApi;
use log::{info, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    info!("--------------------");
    info!("Application starting");
    dotenv().ok();

    run().await;

    info!("Application closing");

    exit(0)

    // todo: ensure Teams connection can be lost and reconnected since it is WS and not REST
    // todo: logging
    // todo: implement back all configs from orig project
    // todo: try to trigger an initial status response
}

pub async fn run() {
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
}
