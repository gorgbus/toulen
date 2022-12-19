#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::prelude::Utc;
use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use platform_dirs::AppDirs;
use std::fs;
use tauri::Manager;

#[tauri::command]
fn save(koruny: u32, auto_sniff: u32, boost_dech: u32, boost_prachy: u32) {
    let save = format!("{} {} {} {}", koruny, auto_sniff, boost_dech, boost_prachy);

    let app_data = AppDirs::new(Some("toulen_sniffer"), false)
        .unwrap()
        .data_dir;

    if fs::read_dir(&app_data).is_err() {
        fs::create_dir(&app_data).unwrap();
    }

    fs::write(format!("{}/save.txt", app_data.to_string_lossy()), save).unwrap();
}

#[tauri::command]
fn get_save() -> String {
    let app_data = AppDirs::new(Some("toulen_sniffer"), false)
        .unwrap()
        .data_dir;

    String::from_utf8_lossy(
        &fs::read(format!("{}/save.txt", app_data.to_string_lossy()))
            .unwrap_or("".to_string().into_bytes()),
    )
    .to_string()
}

#[tauri::command]
async fn open_game(window: tauri::Window) {
    if let Some(launcher) = window.get_window("Ostrava-Svinov") {
        launcher.close().unwrap();
    }

    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
async fn close_game(window: tauri::Window) {
    window.get_window("main").unwrap().close().unwrap();
    window
        .get_window("Ostrava-Svinov")
        .unwrap()
        .close()
        .unwrap();
}

fn main() {
    let now = Utc::now();
    let ts: i64 = now.timestamp();

    let payload = Activity::new()
        .timestamps(Timestamps::new().start(ts))
        .details("Ostrava Svinov")
        .state("Sniffuje toluen")
        .assets(
            Assets::new()
                .large_image("toulen")
                .large_text("Toulen Sniffer"),
        );

    if let Ok(mut client) = DiscordIpcClient::new("1054430080786509894") {
        println!("Connected to Discord");

        match client.connect() {
            Ok(_) => match client.set_activity(payload) {
                Ok(_) => println!("Rich presence set"),
                _ => (),
            },
            _ => (),
        }
    };

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save, get_save, open_game, close_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
