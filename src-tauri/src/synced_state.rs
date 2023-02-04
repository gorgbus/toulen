use std::{borrow::Borrow, sync::Arc, time::Duration};

use serde::Serialize;
use tauri::{AppHandle, Manager};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex, MutexGuard,
};

use tokio::task::JoinHandle;

use crate::{
    inits::Player,
    inits::{CanEmit, Prices, Stats},
    tweak_data,
};

use declarative_discord_rich_presence::activity::{Activity, Assets};
use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
struct Handles {
    breath: Option<JoinHandle<()>>,
    sniff: Option<JoinHandle<()>>,
}

pub struct Synced {
    pub player: Arc<Mutex<Player>>,
    pub prices: Arc<Mutex<Prices>>,
    pub stats: Arc<Mutex<Stats>>,
    pub handle: AppHandle,
    tx: UnboundedSender<Event>,
    handles: Mutex<Handles>,
    d_rpc: Mutex<DeclarativeDiscordIpcClient>,
}

#[derive(Debug)]
pub enum Ability {
    Auto,
    Regen,
    Stamina,
}

#[derive(Debug)]
pub enum Event {
    Init,
    Sniff,
    Breath,
    CanBreath,
    AutoSniff,
    Buy(Ability),
}

impl Synced {
    pub async fn init(handle: impl Borrow<AppHandle>) -> Self {
        let handle = handle.borrow();
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Event>();

        let client = DeclarativeDiscordIpcClient::new("1054430080786509894");

        client.enable();

        if let Err(err) = client.set_activity(
            Activity::new().details("Ostrava Svinov").assets(
                Assets::new()
                    .large_image("toulen")
                    .large_text("Toulen Sniffer"),
            ),
        ) {
            println!("Error setting rich presence status: {}", err);
        };

        let synced = Self {
            player: Arc::new(Mutex::new(Player::default())),
            prices: Arc::new(Mutex::new(Prices::default())),
            stats: Arc::new(Mutex::new(Stats::default())),
            handle: handle.clone(),
            tx,
            handles: Mutex::new(Handles {
                breath: None,
                sniff: None,
            }),
            d_rpc: Mutex::new(client),
        };

        synced.start_event_loop(rx).await;

        synced
    }

    pub fn init_sync(handle: impl Borrow<AppHandle>) -> Self {
        tokio::task::block_in_place(|| tauri::async_runtime::block_on(Self::init(handle)))
    }

    pub async fn update_status(&self, activity: Activity) {
        let d_rpc = self.d_rpc.lock().await;

        if let Err(err) = d_rpc.set_activity(activity) {
            println!("Error updating rich presence status: {}", err);
        };
    }

    fn emit_update(&self, payload: Player) {
        let handle = &self.handle;
        let event = format!("synced-state://player-update");

        handle.emit_all(event.as_str(), payload).ok();
    }

    async fn start_event_loop(&self, mut rx: UnboundedReceiver<Event>) {
        fn emit_update<T>(handle: &AppHandle, payload: &T, event: &str)
        where
            T: CanEmit + Serialize,
        {
            let event = format!("synced-state://{}-update", event);

            handle.emit_all(event.as_str(), payload).ok();
        }

        fn get_cost(base_cost: u32, curr_level: u32) -> u32 {
            let powers = [0.0, 0.03, 0.06];

            let power = Decimal::from_u32(curr_level).unwrap() / dec!(3) / dec!(10)
                + Decimal::from_f64(powers[(curr_level % 3) as usize]).unwrap()
                + dec!(1);

            let cost = base_cost as f64;

            cost.powf(power.to_f64().unwrap()).round() as u32
        }

        let handle = self.handle.clone();
        let mut player = self.player.lock().await.clone();
        let mut prices = self.prices.lock().await.clone();
        let mut stats = self.stats.lock().await.clone();

        let tx = self.tx.clone();

        tokio::spawn(async move {
            // let mut num_of_breaths = 0;
            let mut num_of_sniffs = 0;

            while let Some(event) = rx.recv().await {
                match event {
                    Event::Init => {
                        emit_update(&handle, &player, "player");
                        emit_update(&handle, &prices, "prices");
                        emit_update(&handle, &stats, "stats");
                    }

                    Event::CanBreath => {
                        player.can_breathe = true;

                        emit_update(&handle, &player, "player");
                    }

                    Event::Sniff => {
                        if !player.can_sniff {
                            continue;
                        }

                        if player.stamina > tweak_data::STAMINA_PER_SNIFF {
                            player.stamina -= tweak_data::STAMINA_PER_SNIFF;
                        } else {
                            player.stamina = 0;

                            player.can_breathe = false;
                            player.can_sniff = false;

                            stats.out_of_breath += 1;

                            let tx = tx.clone();

                            tokio::spawn(async move {
                                tokio::time::sleep(Duration::from_secs(
                                    tweak_data::CANT_BREATH_TIME,
                                ))
                                .await;

                                if let Err(err) = tx.send(Event::CanBreath) {
                                    println!("Error sending CanBreath event: {}", err);
                                };
                            });
                        }

                        player.money += 1;
                        stats.sniffed += 1;

                        emit_update(&handle, &player, "player");
                        emit_update(&handle, &stats, "stats");
                    }

                    Event::Buy(ability) => match ability {
                        Ability::Auto => {
                            if player.money >= prices.auto {
                                player.money -= prices.auto;
                                player.auto_lvl += 1;

                                stats.spent_money += prices.auto;

                                prices.auto = get_cost(tweak_data::BASE_AUTO_COST, player.auto_lvl);

                                emit_update(&handle, &player, "player");
                                emit_update(&handle, &prices, "prices");
                                emit_update(&handle, &stats, "stats");
                            }
                        }

                        Ability::Regen => {
                            if player.money >= prices.regen {
                                player.money -= prices.regen;
                                player.regen_lvl += 1;

                                stats.spent_money += prices.regen;

                                prices.regen =
                                    get_cost(tweak_data::BASE_REGEN_COST, player.regen_lvl);

                                emit_update(&handle, &player, "player");
                                emit_update(&handle, &prices, "prices");
                                emit_update(&handle, &stats, "stats");
                            }
                        }

                        Ability::Stamina => {
                            if player.money >= prices.stamina {
                                player.money -= prices.stamina;
                                player.stamina_lvl += 1;

                                stats.spent_money += prices.stamina;

                                prices.stamina =
                                    get_cost(tweak_data::BASE_STAMINA_COST, player.stamina_lvl);

                                emit_update(&handle, &player, "player");
                                emit_update(&handle, &prices, "prices");
                                emit_update(&handle, &stats, "stats");
                            }
                        }
                    },

                    Event::Breath => {
                        let max_stamina = tweak_data::BASE_STAMINA
                            + tweak_data::STAMINA_PER_LEVEL * player.stamina_lvl;
                        let stamina_per_interval = tweak_data::STAMINA_PER_INTERVAL
                            + player.regen_lvl / tweak_data::STAMINA_EACH_LEVEL;

                        if player.stamina < max_stamina && player.can_breathe {
                            player.stamina += stamina_per_interval;

                            if player.stamina >= max_stamina {
                                player.stamina = max_stamina;

                                player.can_sniff = true;
                            }

                            emit_update(&handle, &player, "player");
                        }
                    }

                    Event::AutoSniff => {
                        if player.auto_lvl < 1 || !player.can_sniff {
                            continue;
                        }

                        num_of_sniffs += 1;

                        let sniffs = if player.auto_lvl < 10 {
                            11 - player.auto_lvl
                        } else {
                            1
                        };

                        if sniffs == num_of_sniffs {
                            num_of_sniffs = 0;

                            stats.sniffed += 1;

                            let sniffed_stamina = tweak_data::STAMINA_PER_SNIFF
                                + tweak_data::STAMINA_PER_AUTO * (player.auto_lvl / 2);

                            let mut added_money = tweak_data::MONEY_PER_SNIFF;

                            if player.auto_lvl > 10 {
                                added_money += player.auto_lvl - 10
                            }

                            if player.stamina > sniffed_stamina {
                                player.stamina -= sniffed_stamina;
                            } else {
                                player.stamina = 0;

                                player.can_breathe = false;
                                player.can_sniff = false;

                                stats.out_of_breath += 1;

                                let tx = tx.clone();

                                tokio::spawn(async move {
                                    tokio::time::sleep(Duration::from_secs(
                                        tweak_data::CANT_BREATH_TIME,
                                    ))
                                    .await;

                                    if let Err(err) = tx.send(Event::CanBreath) {
                                        println!("Error sending CanBreath event: {}", err);
                                    };
                                });
                            }

                            player.money += added_money;
                            stats.money += added_money;

                            emit_update(&handle, &player, "player");
                            emit_update(&handle, &stats, "stats");
                        }
                    }
                }
            }
        });
    }

    pub async fn breathe(&self) {
        let tx = self.tx.clone();
        let mut handle = self.handles.lock().await;

        let mut interval =
            tokio::time::interval(Duration::from_millis(tweak_data::BASE_STAMINA_INTERVAL));

        handle.breath = Some(tokio::spawn(async move {
            loop {
                interval.tick().await;

                if let Err(err) = tx.send(Event::Breath) {
                    println!("Error sending breath event: {:?}", err);
                };
            }
        }));
    }

    pub async fn stop_breating(&self) {
        let mut handle = self.handles.lock().await;

        if let Some(breath) = handle.breath.take() {
            breath.abort();
        }
    }

    pub async fn auto_sniff(&self) {
        let tx = self.tx.clone();
        let mut handle = self.handles.lock().await;

        let mut interval = tokio::time::interval(Duration::from_secs(1));

        handle.sniff = Some(tokio::spawn(async move {
            loop {
                interval.tick().await;

                if let Err(err) = tx.send(Event::AutoSniff) {
                    println!("Error sending sniff event: {:?}", err);
                };
            }
        }));
    }

    pub async fn stop_auto_sniff(&self) {
        let mut handle = self.handles.lock().await;

        if let Some(sniff) = handle.sniff.take() {
            sniff.abort();
        }
    }

    pub fn send_event(&self, event: Event) {
        if let Err(err) = self.tx.send(event) {
            println!("error sending event: {}", err);
        };
    }

    pub async fn mutate(&self, function: impl FnOnce(&mut Player)) {
        let mut state = self.player.lock().await;

        function(&mut state);

        self.emit_update(state.to_owned());
    }

    pub async fn get(&self) -> Player {
        let lock = self.player.lock().await;
        lock.clone()
    }

    pub async fn set(&self, new_value: Player) {
        self.mutate(|value| {
            *value = new_value.clone();
        })
        .await;
    }

    pub async fn lock(&self) -> MutexGuard<Player> {
        self.player.lock().await
    }

    pub async fn reset(&self) {
        self.set(Player::default()).await;
    }
}
