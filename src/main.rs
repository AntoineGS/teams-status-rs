#![windows_subsystem = "windows"]
mod configuration;
mod error;
mod home_assistant;
mod mqtt;
mod teams;
mod traits;
mod tray;
mod utils;

use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time;

use crate::configuration::get_configuration;
use crate::error::Error;
use crate::mqtt::api::MqttApi;
use crate::teams::api::TeamsAPI;
use crate::traits::Listener;
use crate::tray::create_tray;
use home_assistant::api::HaApi;
use log::{error, info, LevelFilter};
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    info!("--------------------");
    info!("Application starting");

    // to toggle mute from the tray icon, and let Teams allow the application to listen to its websocket
    let toggle_mute = Arc::new(AtomicBool::new(false));
    // used by tray icon to allow exiting the application
    let is_running = Arc::new(AtomicBool::new(true));
    let _tray = create_tray(is_running.clone(), toggle_mute.clone());
    let five_seconds = time::Duration::from_secs(5);
    let mut save_configuration = true;

    // Aggressive to re-create the connections, but it will handle all APIs, ideal way would
    // be to structure to app so that each API has its own loop and message queue, so when it
    // comes back online it would pickup the items from the queue and process them.
    while is_running.load(Ordering::Relaxed) {
        let result = run_apis(is_running.clone(), toggle_mute.clone(), save_configuration).await;
        save_configuration = false;

        if result.is_err() {
            result.unwrap_or_else(|error| error!("Error encountered: {}", error));

            if is_running.load(Ordering::Relaxed) {
                tokio::time::sleep(five_seconds).await;
            }
        }
    }

    info!("Application closing");

    exit(0);
}

async fn run_apis(
    is_running: Arc<AtomicBool>,
    toggle_mute: Arc<AtomicBool>,
    save_configuration: bool,
) -> Result<(), Error> {
    let conf = get_configuration(save_configuration);
    let teams_api = TeamsAPI::new(&conf.teams);
    let listener: &dyn Listener;
    let ha_api: HaApi;
    let mqtt_api: MqttApi;

    if conf.mqtt.url.is_empty() {
        ha_api = HaApi::new(conf.ha)?;
        listener = &ha_api;
    } else {
        mqtt_api = MqttApi::new(conf.mqtt)?;
        listener = &mqtt_api;
    }

    teams_api
        .start_listening(Arc::new(listener), is_running.clone(), toggle_mute.clone())
        .await?;

    Ok(())
}

fn init_logging() {
    let fixed_window_roller = FixedWindowRoller::builder()
        .build("output_old{}.log", 1)
        .unwrap();
    let size_limit = 10 * 1024 * 1024;
    let size_trigger = SizeTrigger::new(size_limit);
    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d:<36} {l} {t} - {m}{n}")))
        .build("output.log", Box::new(compound_policy))
        .unwrap();

    let log_config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(log_config).unwrap();
}

// todo: translations & language config?
// todo: get a better icon
// todo: auto create versions and packages when creating tags on GitHub (if doable)
// todo: write new tests and pass existing ones
// todo: improve utils.rs encryption
