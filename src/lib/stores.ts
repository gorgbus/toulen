import { writable, type Writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import type { Player } from "$bindings/Player";
import type { Prices } from "$bindings/Prices";
import type { Stats } from "$bindings/Stats";
import type { LoginStatus } from "$bindings/LoginStatus";
import type { TweakData } from "$bindings/TweakData";

export const launch = writable(false);
export const opened = writable(false);

export const player_store: Writable<Player> = writable();

export const logged_in: Writable<LoginStatus> = writable({
    success: false,
    logged: false,
    reason: "Not logged in",
});

export const init_login = () => {
    listen<LoginStatus>("synced-state://logged-in-update", (event) => logged_in.set(event.payload));
}

export const init_player = () => {
    listen<Player>("synced-state://player-update", (event) => {
        player_store.update(player => ({
            ...player,
            ...event.payload
        }));
    });
}

export const prices_store: Writable<Prices> = writable();

export const init_prices = () => {
    listen<Prices>("synced-state://prices-update", (event) => {
        prices_store.update(prices => ({
            ...prices,
            ...event.payload
        }));
    });
}

export const tweakdata_store: Writable<TweakData> = writable();

export const init_tweakdata = () => {
    listen<TweakData>("synced-state://tweakdata-update", (event) => {
        tweakdata_store.set(event.payload);
    });
}

export const stats_store: Writable<Stats> = writable();

export const init_stats = () => {
    listen<Stats>("synced-state://stats-update", (event) => {
        stats_store.update(stats => ({
            ...stats,
            ...event.payload
        }));
    });
}