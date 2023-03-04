#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::prelude::Utc;
use declarative_discord_rich_presence::activity::{Activity, Assets, Timestamps};
use std::env;
use toulen_sniffer::{
    api::{get_players, LeaderboardPlayer},
    synced_state::{Ability, Event},
    PluginBuilder, SyncState,
};

#[tauri::command]
async fn stop_playtime(player: SyncState<'_>) -> Result<(), ()> {
    player.stop_playtime().await;

    Ok(())
}

#[tauri::command]
async fn playtime(player: SyncState<'_>) -> Result<(), ()> {
    player.playtime().await;

    Ok(())
}

#[tauri::command]
async fn get_players_cmd() -> Result<Vec<LeaderboardPlayer>, ()> {
    let players = get_players().await;

    match players {
        Ok(players) => Ok(players),
        Err(err) => {
            println!("{}", err);
            Err(())
        }
    }
}
#[tauri::command]
fn logout(player: SyncState<'_>) {
    player.send_event(Event::Logout);
}

#[tauri::command]
fn logging(player: SyncState<'_>) {
    player.send_event(Event::Logging);
}

#[tauri::command]
fn logged(player: SyncState<'_>) {
    player.send_event(Event::Logged);
}

#[tauri::command]
fn set_token(player: SyncState<'_>, token: &str) {
    player.send_event(Event::Token(String::from(token)));
}

#[tauri::command]
fn login(player: SyncState<'_>) {
    player.send_event(Event::Login);
}

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
fn get_api_url() -> String {
    env!("API_URL").to_string()
}

#[tauri::command]
fn get_auth0_domain() -> String {
    env!("AUTH0_DOMAIN").to_string()
}

#[tauri::command]
fn get_auth0_client_id() -> String {
    env!("AUTH0_CLIENT_ID").to_string()
}

#[tauri::command]
fn get_auth0_audience() -> String {
    env!("AUTH0_AUDIENCE").to_string()
}

#[tauri::command]
fn get_auth0_redir_url() -> String {
    let is_dev = option_env!("DEV");

    if is_dev.is_some() {
        String::from("http://localhost:1420")
    } else {
        String::from("https://tauri.localhost")
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::new().manage().build())
        .invoke_handler(tauri::generate_handler![
            get_api_url,
            start_breath,
            stop_breath,
            start_auto,
            stop_auto,
            activity_main_menu,
            activity_game,
            buy,
            init_states,
            sniff,
            playtime,
            stop_playtime,
            set_token,
            login,
            logged,
            logging,
            logout,
            get_players_cmd,
            get_auth0_domain,
            get_auth0_client_id,
            get_auth0_audience,
            get_auth0_redir_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
