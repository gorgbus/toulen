use std::{marker::PhantomData, ops::Not};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use platform_dirs::AppDirs;

use std::fs;
use tokio::fs::read;

use crate::{synced_state::Synced, tweak_data};

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use orion::aead;

const ENC_KEY: &[u8; 32] = b"d217851a0c9f0bb5f203b3ffa5e19f95";

fn decrypt<T>(data: Vec<u8>) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let bytes = aead::open(&aead::SecretKey::from_slice(ENC_KEY).unwrap(), &data)
        .map_err(|_| "couldn't decrypt data")?;

    Ok(serde_json::from_slice(&bytes).map_err(|_| "couldn't parse from json str")?)
}

fn encrypt<T>(data: &T) -> Result<Vec<u8>, String>
where
    T: Serialize,
{
    let bytes = serde_json::to_vec(data).map_err(|_| "couldn't serialize to json str")?;

    Ok(
        aead::seal(&aead::SecretKey::from_slice(ENC_KEY).unwrap(), &bytes)
            .map_err(|_| "couldn't encrypt data")?,
    )
}
pub fn save_last_id(id: &str) -> Result<(), String> {
    let app_data = AppDirs::new(Some("toulen_sniffer"), false);

    if app_data.is_some() {
        let app_data = app_data
            .ok_or("couldn't get app data dir")?
            .data_dir
            .join("session.sav");

        let enc_str = encrypt(&LastId {
            id: String::from(id),
        })
        .map_err(|_| "couldn't encrypt last id")?;

        fs::write(app_data, enc_str).map_err(|_| "couldn't write last id")?;
    }

    Ok(())
}

pub async fn get_last_id() -> Result<String, String> {
    let app_data = AppDirs::new(Some("toulen_sniffer"), false);

    if app_data.is_some() {
        let app_data = app_data
            .ok_or("couldn't get app data dir")?
            .data_dir
            .join("session.sav");

        if app_data.exists() {
            let data = read(app_data).await.map_err(|_| "couldn't read last id")?;

            let last_id: LastId = decrypt(data).map_err(|_| "couldn't decrypt last id")?;

            return Ok(last_id.id);
        }
    }

    Ok(String::from("-1"))
}

#[derive(Debug, Serialize, Deserialize)]
struct LastId {
    id: String,
}

#[derive(Debug, Clone)]
pub struct StateInit {
    phantom: PhantomData<Player>,
}

impl StateInit {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn manage(&self, handle: &AppHandle) {
        let state = Synced::init_sync(handle);
        handle.manage(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct User {
    pub id: String,
    pub name: String,
    pub tag: String,
}

#[derive(Serialize, Deserialize, Clone, TS, Debug)]
#[ts(export)]
pub struct Player {
    pub id: String,
    pub money: u32,
    pub stamina_lvl: u32,
    pub regen_lvl: u32,
    pub auto_lvl: u32,
    pub stamina: u32,
    pub can_breathe: bool,
    pub can_sniff: bool,
    pub user: Option<User>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: String::from("-1"),
            money: 0,
            stamina_lvl: 0,
            regen_lvl: 0,
            auto_lvl: 0,
            stamina: tweak_data::BASE_STAMINA,
            can_breathe: true,
            can_sniff: true,
            user: None,
        }
    }
}

impl Player {
    pub async fn load(id: &str) -> Result<Self, ()> {
        let app_data = AppDirs::new(Some("toulen_sniffer"), false);

        if app_data.is_some() {
            let app_data = app_data
                .ok_or(())?
                .data_dir
                .join(format!("{}/player.sav", id));

            if app_data.exists() {
                let enc_str = read(app_data).await.map_err(|_| ())?;

                let mut player: Self = decrypt(enc_str).map_err(|_| ())?;

                if player.id != id {
                    return Err(());
                }

                player.stamina =
                    tweak_data::BASE_STAMINA + player.stamina_lvl * tweak_data::STAMINA_PER_LEVEL;

                player.can_breathe = true;
                player.can_sniff = true;

                return Ok(player);
            }
        }

        Err(())
    }

    pub fn save(&self) -> Result<(), String> {
        let app_data = AppDirs::new(Some("toulen_sniffer"), false);

        if app_data.is_some() {
            let app_data = app_data.ok_or("")?.data_dir.join(&self.id);

            if app_data.exists().not() {
                fs::create_dir_all(&app_data).map_err(|_| "couldn't create dir")?;
            }

            let enc_str = encrypt(self).map_err(|_| "couldn't encrypt player")?;

            fs::write(app_data.join("player.sav"), enc_str)
                .map_err(|_| "couldn't save to player.sav")?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, TS, Debug)]
#[ts(export)]
pub struct Prices {
    pub stamina: u32,
    pub auto: u32,
    pub regen: u32,
}

pub fn get_cost(base_cost: u32, curr_level: u32) -> u32 {
    let powers = [0.0, 0.03, 0.06];

    let power = Decimal::from_u32(curr_level / 3).unwrap() / dec!(10)
        + Decimal::from_f64(powers[(curr_level % 3) as usize]).unwrap()
        + dec!(1);

    let cost = base_cost as f64;

    cost.powf(power.to_f64().unwrap()).round() as u32
}

impl Default for Prices {
    fn default() -> Self {
        Self {
            stamina: tweak_data::BASE_STAMINA_COST,
            auto: tweak_data::BASE_AUTO_COST,
            regen: tweak_data::BASE_REGEN_COST,
        }
    }
}

impl Prices {
    pub fn load(player: &Player) -> Self {
        Self {
            stamina: get_cost(tweak_data::BASE_STAMINA_COST, player.stamina_lvl),
            auto: get_cost(tweak_data::BASE_AUTO_COST, player.auto_lvl),
            regen: get_cost(tweak_data::BASE_REGEN_COST, player.regen_lvl),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, TS, Debug)]
#[ts(export)]
pub struct Stats {
    pub id: String,
    pub money: u32,
    pub spent_money: u32,
    pub playtime: u32,
    pub out_of_breath: u32,
    pub sniffed: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            id: String::from("-1"),
            money: 0,
            spent_money: 0,
            playtime: 0,
            out_of_breath: 0,
            sniffed: 0,
        }
    }
}

impl Stats {
    pub async fn load(id: &str) -> Result<Self, ()> {
        let app_data = AppDirs::new(Some("toulen_sniffer"), false);

        if app_data.is_some() {
            let app_data = app_data
                .ok_or(())?
                .data_dir
                .join(format!("{}/stats.sav", id));

            if app_data.exists() {
                let enc_str = read(app_data).await.map_err(|_| ())?;

                let stats: Self = decrypt(enc_str).map_err(|_| ())?;

                if stats.id != id {
                    return Err(());
                }

                return Ok(stats);
            }
        }

        Err(())
    }

    pub fn save(&self) -> Result<(), ()> {
        let app_data = AppDirs::new(Some("toulen_sniffer"), false);

        if app_data.is_some() {
            let app_data = app_data.ok_or(())?.data_dir.join(&self.id);

            if app_data.exists().not() {
                fs::create_dir_all(&app_data).map_err(|_| ())?;
            }

            let enc_str = encrypt(self).map_err(|_| ())?;

            fs::write(app_data.join("stats.sav"), enc_str).map_err(|_| ())?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct LoginStatus {
    pub success: bool,
    pub logged: bool,
    pub reason: String,
}

pub trait CanEmit {}

impl CanEmit for Player {}
impl CanEmit for Prices {}
impl CanEmit for Stats {}
impl CanEmit for LoginStatus {}
