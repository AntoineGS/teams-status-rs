use crate::traits::StopController;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tray_item::{IconSource, TrayItem};

pub struct TrayWindows {
    _tray: TrayItem,
}

impl TrayWindows {
    pub fn new(is_running: Arc<AtomicBool>, toggle_mute: Arc<AtomicBool>) -> Self {
        let mut tray = TrayItem::new("Teams Status", IconSource::Resource("default-icon")).unwrap();

        tray.add_menu_item("Toggle Mute", move || {
            toggle_mute.store(true, Ordering::Relaxed);
        })
        .unwrap();

        tray.add_menu_item("Quit", move || {
            is_running.store(false, Ordering::Relaxed);
        })
        .unwrap();

        TrayWindows { _tray: tray }
    }
}

impl StopController for TrayWindows {}

pub fn create_tray(
    is_running: Arc<AtomicBool>,
    toggle_mute: Arc<AtomicBool>,
) -> Box<dyn StopController> {
    let tray = TrayWindows::new(is_running, toggle_mute);
    Box::new(tray)
}
