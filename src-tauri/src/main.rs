// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use sys::Sys;
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

mod proc;
mod sys;
mod data;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            {
                let handle = app.handle();
                tauri::async_runtime::spawn(async move {
                    let mut sys = Sys::new().unwrap();
                    loop {
                        sys.update().unwrap();
                        handle.emit_all("testt", Vec::<data::Proc>::from(&sys)).unwrap();
                        async_std::task::sleep(Duration::from_secs(1)).await;
                    }
                });
            }

            {
                let handle = app.handle();
                tauri::async_runtime::spawn(async move {
                    let mut sys = Sys::new().unwrap();
                    loop {
                        sys.update().unwrap();
                        handle.emit_all("cpu", data::CpuStat::from(&sys)).unwrap();
                        async_std::task::sleep(Duration::from_secs(1)).await;
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .menu(Menu::with_items([
            CustomMenuItem::new("tttt", "TESTTTT").into(),
            CustomMenuItem::new("toggle", "Toggle visibility").into(),
            Submenu::new(
                "View",
                Menu::with_items([CustomMenuItem::new("test", "Toggle visbility").into()]),
            )
            .into(),
        ]))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
