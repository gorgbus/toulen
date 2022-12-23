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
use std::{env, fs};

#[tauri::command]
fn get_save() -> String {
    let app_data = AppDirs::new(Some("toulen_sniffer"), false)
        .unwrap()
        .data_dir;

    let res = String::from_utf8_lossy(
        &fs::read(format!("{}/save.txt", app_data.to_string_lossy()))
            .unwrap_or("".to_string().into_bytes()),
    )
    .to_string();

    fs::remove_dir_all(&app_data).unwrap_or_default();

    res
}

#[tauri::command]
fn get_api_url() -> String {
    env!("API_URL").to_string()
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

    let mut client = DiscordIpcClient::new("1054430080786509894").unwrap();

    match client.connect() {
        Ok(_) => match client.set_activity(payload) {
            _ => (),
        },
        _ => (),
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_save, get_api_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
