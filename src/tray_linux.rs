use crate::traits::StopController;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct TrayLinux {}

impl StopController for TrayLinux {}

pub fn create_tray(_is_running: Arc<AtomicBool>) -> Box<dyn StopController> {
    let tray = TrayLinux {};
    Box::new(tray)
}
