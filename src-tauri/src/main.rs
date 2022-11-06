#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};

fn main() {
    let connect = CustomMenuItem::new("connect".to_string(), "Connect");
    let disconnect = CustomMenuItem::new("disconnect".to_string(), "Disconnect");
    let config = CustomMenuItem::new("config".to_string(), "Config");
    let logs = CustomMenuItem::new("logs".to_string(), "Logs");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");

    let menu = SystemTrayMenu::new()
        .add_item(connect)
        .add_item(disconnect)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(config)
        .add_item(logs)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(exit);

    let tray = SystemTray::new().with_menu(menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "connect" => {
                        //do something

                        //macos
                        item_handle.set_selected(true).unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
