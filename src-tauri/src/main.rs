#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save, get_save, open_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
