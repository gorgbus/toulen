#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::prelude::Utc;
use declarative_discord_rich_presence::activity::{Activity, Assets, Timestamps};
use platform_dirs::AppDirs;
use std::{env, fs};
use toulen_sniffer::{
    synced_state::{Ability, Event},
    PluginBuilder, SyncState,
};

#[tauri::command]
fn sniff(player: SyncState<'_>) {
    player.send_event(Event::Sniff);
}

#[tauri::command]
fn init_states(player: SyncState<'_>) {
    player.send_event(Event::Init);
}

#[tauri::command]
fn buy(player: SyncState<'_>, ability: &str) {
    match ability {
        "stamina" => player.send_event(Event::Buy(Ability::Stamina)),
        "regen" => player.send_event(Event::Buy(Ability::Regen)),
        "auto" => player.send_event(Event::Buy(Ability::Auto)),
        _ => {}
    }
}

#[tauri::command]
async fn activity_main_menu(player: SyncState<'_>) -> Result<(), ()> {
    let activity = Activity::new().details("Ostrava Svinov").assets(
        Assets::new()
            .large_image("toulen")
            .large_text("Toulen Sniffer"),
    );

    player.update_status(activity).await;

    Ok(())
}

#[tauri::command]
async fn activity_game(player: SyncState<'_>) -> Result<(), ()> {
    let now = Utc::now();
    let ts: i64 = now.timestamp();

    let activity = Activity::new()
        .timestamps(Timestamps::new().start(ts))
        .details("Svinov Mosty")
        .state("Sniffuje toluen")
        .assets(
            Assets::new()
                .large_image("toulen")
                .large_text("Toulen Sniffer"),
        );

    player.update_status(activity).await;

    Ok(())
}

#[tauri::command]
async fn start_breath(player: SyncState<'_>) -> Result<(), ()> {
    player.breathe().await;

    Ok(())
}

#[tauri::command]
async fn stop_breath(player: SyncState<'_>) -> Result<(), ()> {
    player.stop_breating().await;

    Ok(())
}

#[tauri::command]
async fn start_auto(player: SyncState<'_>) -> Result<(), ()> {
    player.auto_sniff().await;

    Ok(())
}

#[tauri::command]
async fn stop_auto(player: SyncState<'_>) -> Result<(), ()> {
    player.stop_auto_sniff().await;

    Ok(())
}

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
    tauri::Builder::default()
        .plugin(PluginBuilder::new().manage().build())
        .invoke_handler(tauri::generate_handler![
            get_save,
            get_api_url,
            start_breath,
            stop_breath,
            start_auto,
            stop_auto,
            activity_main_menu,
            activity_game,
            buy,
            init_states,
            sniff
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
