// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem, Manager, window};
use tauri::api::dialog::MessageDialogBuilder;
use tauri::api::dialog::MessageDialogKind as kind;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "Quit");
    let test: CustomMenuItem = CustomMenuItem::new("test".to_string(), "Test");
    let tray_menu: SystemTrayMenu = SystemTrayMenu::new()
        .add_item(test)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
          SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
          } => {
            println!("system tray received a left click");
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
          }
          SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
              "quit" => {
                std::process::exit(0);
              }
              "test" => {
                MessageDialogBuilder::new("Test", "Test message")
                  .kind(kind::Error)
                  .show(|_ok| {});
              }
              _ => {}
            }
          }
          _ => {}
        })
        .on_window_event(|event| match event.event() {
          tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
          }
          _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
