use crate::traits::StopController;
use image::GenericImageView;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tray_icon::{
    menu::{Menu, MenuId, MenuItem},
    Icon, TrayIconBuilder,
};

pub struct TrayWindows {
    _tray: tray_icon::TrayIcon,
    _menu: Menu,
}

impl TrayWindows {
    pub fn new(_is_running: Arc<AtomicBool>, _toggle_mute: Arc<AtomicBool>) -> Self {
        // Create menu and items
        let menu = Menu::new();
        let toggle_mute_item =
            MenuItem::with_id(MenuId::new("toggle_mute"), "Toggle Mute", true, None);
        let quit_item = MenuItem::with_id(MenuId::new("quit"), "Quit", true, None);
        menu.append(&toggle_mute_item).unwrap();
        menu.append(&quit_item).unwrap();

        // Load icon
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

        // Create tray icon with menu
        let tray = TrayIconBuilder::new()
            .with_icon(icon)
            .with_tooltip("Teams Status")
            .with_menu(Box::new(menu))
            .build()
            .unwrap();

        // Create a dummy menu to store (since the original was moved)
        let dummy_menu = Menu::new();

        TrayWindows {
            _tray: tray,
            _menu: dummy_menu,
        }
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
