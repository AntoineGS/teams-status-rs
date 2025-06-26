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
use tray_icon::{menu::MenuEvent, TrayIconEvent};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

use crate::configuration::get_configuration;
use crate::logging::initialize_logging;
use crate::mqtt::api::MqttApi;
use crate::teams_ws::api::TeamsAPI;
use crate::traits::Listener;
use crate::tray::create_tray;
use anyhow::Result;
use home_assistant::api::HaApi;
use log::{error, info};

#[derive(Debug)]
enum UserEvent {
    TrayIconEvent(TrayIconEvent),
    MenuEvent(MenuEvent),
}

struct Application {
    is_running: Arc<AtomicBool>,
    toggle_mute: Arc<AtomicBool>,
    _tray: Option<Box<dyn traits::StopController>>,
}

impl Application {
    fn new(is_running: Arc<AtomicBool>, toggle_mute: Arc<AtomicBool>) -> Self {
        Self {
            is_running,
            toggle_mute,
            _tray: None,
        }
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        if matches!(cause, winit::event::StartCause::Init) {
            // Create tray icon when event loop starts
            self._tray = Some(create_tray(
                self.is_running.clone(),
                self.toggle_mute.clone(),
            ));
        }

        // Check if application should exit
        if !self.is_running.load(Ordering::Relaxed) {
            event_loop.exit();
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::MenuEvent(event) => {
                if event.id.0.as_str() == "toggle_mute" {
                    self.toggle_mute.store(true, Ordering::Relaxed);
                } else if event.id.0.as_str() == "quit" {
                    self.is_running.store(false, Ordering::Relaxed);
                }
            }
            UserEvent::TrayIconEvent(event) => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging first
    initialize_logging();
    info!("--------------------");
    info!("Application starting");

    let mutex = create_mutex();
    if mutex.is_none() {
        exit(1)
    }

    let toggle_mute = Arc::new(AtomicBool::new(false));
    let is_running = Arc::new(AtomicBool::new(true));

    // Create winit event loop
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;

    // Set up event handlers for tray events
    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::TrayIconEvent(event));
    }));

    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let mut app = Application::new(is_running.clone(), toggle_mute.clone());

    // Spawn async tasks in background - don't capture mutex
    let rt = tokio::runtime::Runtime::new()?;
    let is_running_clone = is_running.clone();
    let toggle_mute_clone = toggle_mute.clone();

    std::thread::spawn(move || {
        rt.block_on(async {
            let five_seconds = time::Duration::from_secs(5);
            let mut save_configuration = true;

            while is_running_clone.load(Ordering::Relaxed) {
                let result = run_apis(
                    is_running_clone.clone(),
                    toggle_mute_clone.clone(),
                    save_configuration,
                )
                .await;
                save_configuration = false;

                if result.is_err() {
                    result.unwrap_or_else(|error| error!("Error encountered: {}", error));
                    if is_running_clone.load(Ordering::Relaxed) {
                        tokio::time::sleep(five_seconds).await;
                    }
                }
            }

            info!("Application closing");
            // Don't access mutex here - it will be cleaned up in main thread
        });
    });

    // Run the winit event loop - this is required for tray menu to work
    event_loop.run_app(&mut app)?;

    // Clean up mutex after event loop exits
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
