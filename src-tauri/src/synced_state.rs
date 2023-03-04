use std::{borrow::Borrow, sync::Arc, time::Duration};

use serde::Serialize;
use serde_json::json;
use tauri::{AppHandle, Manager};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    Mutex, MutexGuard,
};

use tokio::task::JoinHandle;

use crate::{
    api::{get, post, ApiUser},
    inits::Player,
    inits::{get_cost, get_last_id, save_last_id, CanEmit, LoginStatus, Prices, Stats, User},
    tweak_data::{self, TweakData},
};

use declarative_discord_rich_presence::activity::{Activity, Assets};
use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;

struct Handles {
    breath: Option<JoinHandle<()>>,
    sniff: Option<JoinHandle<()>>,
    save: Option<JoinHandle<()>>,
    main: Option<JoinHandle<()>>,
}

struct ServerProps {
    token: Option<String>,
    login_status: LoginStatus,
}

pub struct Synced {
    pub player: Arc<Mutex<Player>>,
    pub prices: Arc<Mutex<Prices>>,
    pub stats: Arc<Mutex<Stats>>,
    pub handle: AppHandle,
    tx: Arc<UnboundedSender<Event>>,
    props: Arc<Mutex<ServerProps>>,
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
    Close,
    Playtime,
    Buy(Ability),
    Token(String),
    Login,
    Logged,
    Logging,
    Logout,
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

        let id = get_last_id().await.unwrap_or(String::from("-1"));

        let (player, stats) = match (Player::load(&id).await, Stats::load(&id).await) {
            (Ok(player), Ok(stats)) => (player, stats),
            _ => match (Player::load("-1").await, Stats::load("-1").await) {
                (Ok(player), Ok(stats)) => (player, stats),
                _ => (Player::default(), Stats::default()),
            },
        };

        let prices = Prices::load(&player);

        let synced = Self {
            player: Arc::new(Mutex::new(player)),
            prices: Arc::new(Mutex::new(prices)),
            stats: Arc::new(Mutex::new(stats)),
            handle: handle.clone(),
            tx: Arc::new(tx),
            props: Arc::new(Mutex::new(ServerProps {
                token: None,
                login_status: LoginStatus {
                    success: false,
                    logged: false,
                    reason: String::from("Not logged in"),
                },
            })),
            handles: Mutex::new(Handles {
                breath: None,
                save: None,
                sniff: None,
                main: None,
            }),
            d_rpc: Mutex::new(client),
        };

        synced.handles.lock().await.main = Some(synced.start_event_loop(rx).await);

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

    async fn start_event_loop(&self, mut rx: UnboundedReceiver<Event>) -> JoinHandle<()> {
        fn emit_update<T>(handle: &AppHandle, payload: &T, event: &str)
        where
            T: CanEmit + Serialize,
        {
            let event = format!("synced-state://{}-update", event);

            handle.emit_all(event.as_str(), payload).ok();
        }

        let handle = self.handle.clone();
        let player_p = Arc::clone(&self.player);
        let prices_p = Arc::clone(&self.prices);
        let stats_p = Arc::clone(&self.stats);
        let props_p = Arc::clone(&self.props);

        let tx = Arc::clone(&self.tx);

        tokio::spawn(async move {
            let mut num_of_sniffs = 0;

            let mut player = player_p.lock().await;
            let mut prices = prices_p.lock().await;
            let mut stats = stats_p.lock().await;
            let mut props = props_p.lock().await;

            while let Some(event) = rx.recv().await {
                match event {
                    Event::Token(token) => {
                        props.token = Some(token);
                    }

                    Event::Login => {
                        match &props.token {
                            Some(token) => match get::<ApiUser>("/auth/login", token).await {
                                Ok(res) => {
                                    println!("Logged in as {:#?}", res);

                                    if let Err(err) = save_last_id(&res.id) {
                                        println!("Failed to save last id: {:#?}", err);
                                    };

                                    player.id = String::from(&res.id);
                                    stats.id = String::from(&res.id);

                                    player.user = Some(User {
                                        id: res.id,
                                        name: res.name,
                                        tag: res.tag,
                                    });

                                    if !res.assigned {
                                        if let Err(err) = post(
                                            "/auth/assign",
                                            token,
                                            json!({
                                                "player": {
                                                    "money": player.money,
                                                    "auto_lvl": player.auto_lvl,
                                                    "regen_lvl": player.regen_lvl,
                                                    "stamina_lvl": player.stamina_lvl,
                                                },
                                                "stats": {
                                                    "money": stats.money,
                                                    "spent_money": stats.spent_money,
                                                    "playtime": stats.playtime,
                                                    "out_of_breath": stats.out_of_breath,
                                                    "sniffed": stats.sniffed,
                                                }
                                            }),
                                        )
                                        .await
                                        {
                                            println!("Failed to assign: {:#?}", err);
                                        };
                                    } else {
                                        player.money = res.player.money;
                                        player.auto_lvl = res.player.auto_lvl;
                                        player.regen_lvl = res.player.regen_lvl;
                                        player.stamina_lvl = res.player.stamina_lvl;

                                        stats.money = res.stats.money;
                                        stats.spent_money = res.stats.spent_money;
                                        stats.playtime = res.stats.playtime;
                                        stats.out_of_breath = res.stats.out_of_breath;
                                        stats.sniffed = res.stats.sniffed;
                                    }

                                    emit_update(&handle, &player.to_owned(), "player");
                                    emit_update(&handle, &stats.to_owned(), "stats");

                                    props.login_status = LoginStatus {
                                        success: true,
                                        logged: false,
                                        reason: String::from("Logged in"),
                                    }
                                }
                                Err(err) => {
                                    println!("{:#?}", err);

                                    props.login_status = LoginStatus {
                                        success: false,
                                        logged: false,
                                        reason: String::from("Failed to login"),
                                    }
                                }
                            },
                            _ => {
                                props.login_status = LoginStatus {
                                    success: false,
                                    logged: false,
                                    reason: String::from("No token provided"),
                                }
                            }
                        }

                        emit_update(&handle, &props.login_status, "logged-in");
                    }

                    Event::Logout => {
                        props.login_status = LoginStatus {
                            success: false,
                            logged: false,
                            reason: String::from("Not logged in"),
                        };

                        props.token = None;

                        if let Err(err) = save_last_id("-1") {
                            println!("Failed to save last id: {:#?}", err);
                        };

                        emit_update(&handle, &props.login_status, "logged-in");
                    }

                    Event::Logging => {
                        props.login_status.logged = true;

                        emit_update(&handle, &props.login_status, "logged-in");
                    }

                    Event::Logged => {
                        props.login_status.success = true;

                        emit_update(&handle, &props.login_status, "logged-in");
                    }

                    Event::Close => {
                        if let Err(err) = player.save() {
                            println!("Failed to save player: {:?}", err);
                        }

                        if let Err(err) = stats.save() {
                            println!("Failed to save player: {:?}", err);
                        }

                        if let Some(token) = &props.token {
                            let player_to_save = json!({
                                "money": player.money,
                                "auto_lvl": player.auto_lvl,
                                "regen_lvl": player.regen_lvl,
                                "stamina_lvl": player.stamina_lvl,
                            });

                            let stats_to_save = json!({
                                "money": stats.money,
                                "spent_money": stats.spent_money,
                                "playtime": stats.playtime,
                                "out_of_breath": stats.out_of_breath,
                                "sniffed": stats.sniffed,
                            });

                            if let Err(err) = post("/players", &token, player_to_save).await {
                                println!("Failed to save player: {:#?}", err);
                            };

                            if let Err(err) = post("/stats", &token, stats_to_save).await {
                                println!("Failed to save stats: {:#?}", err);
                            };
                        }

                        rx.close();

                        break;
                    }

                    Event::Init => {
                        emit_update(&handle, &TweakData::default(), "tweakdata");
                        emit_update(&handle, &player.to_owned(), "player");
                        emit_update(&handle, &prices.to_owned(), "prices");
                        emit_update(&handle, &stats.to_owned(), "stats");
                        emit_update(&handle, &props.login_status, "logged-in");
                    }

                    Event::Playtime => {
                        stats.playtime += 1;

                        if let Some(token) = &props.token {
                            let player_to_save = json!({
                                "money": player.money,
                                "auto_lvl": player.auto_lvl,
                                "regen_lvl": player.regen_lvl,
                                "stamina_lvl": player.stamina_lvl,
                            });

                            let token = token.to_owned();

                            tokio::spawn(async move {
                                if let Err(err) = post("/players", &token, player_to_save).await {
                                    println!("Failed to save player: {:#?}", err);
                                };
                            });
                        }

                        emit_update(&handle, &stats.to_owned(), "stats");
                    }

                    Event::CanBreath => {
                        player.can_breathe = true;

                        emit_update(&handle, &player.to_owned(), "player");
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

                            let tx = Arc::clone(&tx);

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

                        emit_update(&handle, &player.to_owned(), "player");
                        emit_update(&handle, &stats.to_owned(), "stats");
                    }

                    Event::Buy(ability) => match ability {
                        Ability::Auto => {
                            if player.money >= prices.auto {
                                player.money -= prices.auto;
                                player.auto_lvl += 1;

                                stats.spent_money += prices.auto;

                                prices.auto = get_cost(tweak_data::BASE_AUTO_COST, player.auto_lvl);

                                emit_update(&handle, &player.to_owned(), "player");
                                emit_update(&handle, &prices.to_owned(), "prices");
                                emit_update(&handle, &stats.to_owned(), "stats");
                            }
                        }

                        Ability::Regen => {
                            if player.money >= prices.regen {
                                player.money -= prices.regen;
                                player.regen_lvl += 1;

                                stats.spent_money += prices.regen;

                                prices.regen =
                                    get_cost(tweak_data::BASE_REGEN_COST, player.regen_lvl);

                                emit_update(&handle, &player.to_owned(), "player");
                                emit_update(&handle, &prices.to_owned(), "prices");
                                emit_update(&handle, &stats.to_owned(), "stats");
                            }
                        }

                        Ability::Stamina => {
                            if player.money >= prices.stamina {
                                player.money -= prices.stamina;
                                player.stamina_lvl += 1;

                                stats.spent_money += prices.stamina;

                                prices.stamina =
                                    get_cost(tweak_data::BASE_STAMINA_COST, player.stamina_lvl);

                                emit_update(&handle, &player.to_owned(), "player");
                                emit_update(&handle, &prices.to_owned(), "prices");
                                emit_update(&handle, &stats.to_owned(), "stats");
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

                            emit_update(&handle, &player.to_owned(), "player");
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

                                let tx = Arc::clone(&tx);

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

                            emit_update(&handle, &player.to_owned(), "player");
                            emit_update(&handle, &stats.to_owned(), "stats");
                        }
                    }
                }
            }
        })
    }

    pub fn abort_handles(&self) {
        loop {
            match self.handles.try_lock() {
                Ok(mut handle) => {
                    if let Some(sniff) = handle.sniff.take() {
                        sniff.abort();
                    }

                    if let Some(breath) = handle.breath.take() {
                        breath.abort();
                    }

                    if let Some(main) = handle.main.take() {
                        main.abort();
                    }

                    break;
                }
                Err(_) => std::thread::sleep(Duration::from_millis(50)),
            };
        }
    }

    pub async fn playtime(&self) {
        let tx = Arc::clone(&self.tx);
        let mut handle = self.handles.lock().await;

        let mut interval = tokio::time::interval(Duration::from_secs(60));

        interval.tick().await;

        handle.save = Some(tokio::spawn(async move {
            loop {
                interval.tick().await;

                if let Err(err) = tx.send(Event::Playtime) {
                    println!("Error sending playtime event: {:?}", err);
                };
            }
        }));
    }

    pub async fn stop_playtime(&self) {
        let mut handle = self.handles.lock().await;

        if let Some(save) = handle.save.take() {
            save.abort();
        }
    }

    pub async fn breathe(&self) {
        let tx = Arc::clone(&self.tx);
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
        let tx = Arc::clone(&self.tx);
        let mut handle = self.handles.lock().await;

        let mut interval = tokio::time::interval(Duration::from_secs(1));

        interval.tick().await;

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
