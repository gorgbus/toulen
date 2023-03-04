use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::inits::CanEmit;

pub const BASE_STAMINA: u32 = 50;
pub const BASE_STAMINA_INTERVAL: u64 = 100;
pub const STAMINA_PER_INTERVAL: u32 = 1;
pub const STAMINA_PER_LEVEL: u32 = 6;
pub const STAMINA_EACH_LEVEL: u32 = 3;

pub const STAMINA_PER_SNIFF: u32 = 10;
pub const STAMINA_PER_AUTO: u32 = 5;
pub const MONEY_PER_SNIFF: u32 = 1;

pub const BASE_AUTO_COST: u32 = 80;
pub const BASE_STAMINA_COST: u32 = 70;
pub const BASE_REGEN_COST: u32 = 50;

pub const CANT_BREATH_TIME: u64 = 10;

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export)]
pub struct TweakData {
    pub base_stamina: u32,
    pub base_stamina_interval: u64,
    pub stamina_per_interval: u32,
    pub stamina_per_level: u32,
    pub stamina_each_level: u32,

    pub stamina_per_sniff: u32,
    pub stamina_per_auto: u32,
    pub money_per_sniff: u32,

    pub base_auto_cost: u32,
    pub base_stamina_cost: u32,
    pub base_regen_cost: u32,

    pub cant_breath_time: u64,
}

impl Default for TweakData {
    fn default() -> Self {
        Self {
            base_stamina: BASE_STAMINA,
            base_stamina_interval: BASE_STAMINA_INTERVAL,
            stamina_per_interval: STAMINA_PER_INTERVAL,
            stamina_per_level: STAMINA_PER_LEVEL,
            stamina_each_level: STAMINA_EACH_LEVEL,

            stamina_per_sniff: STAMINA_PER_SNIFF,
            stamina_per_auto: STAMINA_PER_AUTO,
            money_per_sniff: MONEY_PER_SNIFF,

            base_auto_cost: BASE_AUTO_COST,
            base_stamina_cost: BASE_STAMINA_COST,
            base_regen_cost: BASE_REGEN_COST,

            cant_breath_time: CANT_BREATH_TIME,
        }
    }
}

impl CanEmit for TweakData {}
