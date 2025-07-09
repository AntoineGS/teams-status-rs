use crate::traits::StopController;
use auto_launch::AutoLaunch;
use image::GenericImageView;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::sync::{Arc, RwLock};
use tray_icon::{
    menu::{Menu, MenuId, MenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

pub struct TrayWindows {
    tray: TrayIcon,
    auto_launch: RwLock<AutoLaunch>,
    needs_recreation: Arc<AtomicBool>,
}

// Store only the AutoLaunch instance globally, not the entire tray
static GLOBAL_AUTO_LAUNCH: Mutex<Option<Arc<RwLock<AutoLaunch>>>> = Mutex::new(None);
static GLOBAL_TRAY_RECREATION_FLAG: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

impl TrayWindows {
    pub fn new(_is_running: Arc<AtomicBool>, _toggle_mute: Arc<AtomicBool>) -> Self {
        // Setup auto-launch
        let exe_path = std::env::current_exe().unwrap();
        let exe_str = exe_path.to_str().unwrap();

        // Try with quotes around the path in case of spaces
        let quoted_exe_str = format!("\"{}\"", exe_str);

        let auto_launch = AutoLaunch::new("Teams Status", &quoted_exe_str, &[] as &[&str]);

        // Check initial state with better error handling
        match auto_launch.is_enabled() {
            Ok(enabled) => log::info!("Initial auto-launch state: {}", enabled),
            Err(e) => log::error!("Failed to check auto-launch state: {:?}", e),
        }

        let auto_launch_arc = Arc::new(RwLock::new(auto_launch));
        let recreation_flag = Arc::new(AtomicBool::new(false));

        // Store globals
        *GLOBAL_AUTO_LAUNCH.lock().unwrap() = Some(auto_launch_arc.clone());
        *GLOBAL_TRAY_RECREATION_FLAG.lock().unwrap() = Some(recreation_flag.clone());

        let tray_icon = Self::create_tray_icon(&auto_launch_arc.read().unwrap());

        TrayWindows {
            tray: tray_icon,
            auto_launch: Arc::<std::sync::RwLock<AutoLaunch>>::try_unwrap(auto_launch_arc)
                .unwrap_or_else(|arc| RwLock::from((*arc).read().unwrap().clone())),
            needs_recreation: recreation_flag,
        }
    }

    fn create_tray_icon(auto_launch: &AutoLaunch) -> TrayIcon {
        let menu = Menu::new();
        let toggle_mute_item =
            MenuItem::with_id(MenuId::new("toggle_mute"), "Toggle Mute", true, None);
        let quit_item = MenuItem::with_id(MenuId::new("quit"), "Quit", true, None);

        // Create launch at startup menu item with current state
        let launch_enabled = auto_launch.is_enabled().unwrap_or(false);
        let launch_label = if launch_enabled {
            "Disable launch at startup"
        } else {
            "Enable launch at startup"
        };
        let launch_item =
            MenuItem::with_id(MenuId::new("launch_startup"), launch_label, true, None);

        menu.append(&toggle_mute_item).unwrap();
        menu.append(&launch_item).unwrap();
        menu.append(&quit_item).unwrap();

        let icon_data = include_bytes!("../../microsoft-teams.ico");
        let icon = {
            use image::ImageFormat;
            use std::io::Cursor;
            let cursor = Cursor::new(icon_data);
            let dyn_img =
                image::load(cursor, ImageFormat::Ico).expect("Failed to decode icon image");
            let rgba = dyn_img.to_rgba8();
            let (width, height) = dyn_img.dimensions();
            Icon::from_rgba(rgba.into_raw(), width, height).expect("Failed to create icon")
        };

        TrayIconBuilder::new()
            .with_icon(icon)
            .with_tooltip("Teams Status")
            .with_menu(Box::new(menu))
            .build()
            .unwrap()
    }

    pub fn check_and_recreate_if_needed(&mut self) {
        if self.needs_recreation.load(Ordering::Relaxed) {
            let auto_launch = self.auto_launch.read().unwrap();
            self.tray = Self::create_tray_icon(&auto_launch);
            self.needs_recreation.store(false, Ordering::Relaxed);
        }
    }

    pub fn toggle_auto_launch_global() {
        if let Some(auto_launch_arc) = GLOBAL_AUTO_LAUNCH.lock().unwrap().as_ref() {
            let auto_launch = auto_launch_arc.read().unwrap();
            let enabled = auto_launch.is_enabled().unwrap_or(false);

            let result = if enabled {
                auto_launch.disable()
            } else {
                auto_launch.enable()
            };

            match result {
                Ok(_) => {
                    // Signal that tray needs recreation
                    if let Some(flag) = GLOBAL_TRAY_RECREATION_FLAG.lock().unwrap().as_ref() {
                        flag.store(true, Ordering::Relaxed);
                    }
                }
                Err(e) => {
                    log::error!("Failed to toggle auto-launch: {:?}", e);
                }
            }
        } else {
            log::error!("Could not access global auto-launch instance");
        }
    }
}

impl StopController for TrayWindows {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl StopController for Arc<TrayWindows> {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub fn create_tray(
    is_running: Arc<AtomicBool>,
    toggle_mute: Arc<AtomicBool>,
) -> Box<dyn StopController> {
    let tray = TrayWindows::new(is_running, toggle_mute);
    Box::new(tray)
}
