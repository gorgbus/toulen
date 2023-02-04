use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use crate::{synced_state::Synced, tweak_data};

pub trait StateManage {
    fn manage(&self, app: &AppHandle);
}

pub struct StateInit {
    phantom: PhantomData<Player>,
}

impl StateInit {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl StateManage for StateInit {
    fn manage(&self, handle: &AppHandle) {
        let state = Synced::init_sync(handle);
        handle.manage(state);
    }
}
#[derive(Serialize, Deserialize, Clone, TS, Debug)]
#[ts(export)]
pub struct Player {
    pub money: u32,
    pub stamina_lvl: u32,
    pub regen_lvl: u32,
    pub auto_lvl: u32,
    pub stamina: u32,
    pub can_breathe: bool,
    pub can_sniff: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            money: 0,
            stamina_lvl: 3,
            regen_lvl: 9,
            auto_lvl: 12,
            stamina: tweak_data::BASE_STAMINA + tweak_data::STAMINA_PER_LEVEL * 3,
            can_breathe: true,
            can_sniff: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, TS, Debug)]
#[ts(export)]
pub struct Prices {
    pub stamina: u32,
    pub auto: u32,
    pub regen: u32,
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

#[derive(Serialize, Deserialize, Clone, TS, Debug)]
#[ts(export)]
pub struct Stats {
    pub money: u32,
    pub spent_money: u32,
    pub playtime: u32,
    pub out_of_breath: u32,
    pub sniffed: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            money: 0,
            spent_money: 0,
            playtime: 0,
            out_of_breath: 0,
            sniffed: 0,
        }
    }
}

pub trait CanEmit {}

impl CanEmit for Player {}
impl CanEmit for Prices {}
impl CanEmit for Stats {}
