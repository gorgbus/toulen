import { writable } from "svelte/store";
import { BASE_STAMINA } from "./constants";
import { listen } from "@tauri-apps/api/event";
import type { Player } from "$bindings/Player";
import type { Prices } from "$bindings/Prices";
import type { Stats } from "$bindings/Stats";

export const launch = writable(false);
export const opened = writable(false);

export const player_store = writable({
    id: -1,
    money: 0,
    stamina_lvl: 0,
    regen_lvl: 0,
    auto_lvl: 0,
    stamina: BASE_STAMINA,
    can_breathe: true,
    can_sniff: true,
    toulen: 0,
    user: {
        name: "none"
    }
});

export const init_player = () => {
    listen<Player>("synced-state://player-update", (event) => {
        player_store.update(player => ({
            ...player,
            ...event.payload
        }));
    });
}

export const prices_store = writable({
    stamina: 0,
    regen: 0,
    auto: 0,
});

export const init_prices = () => {
    listen<Prices>("synced-state://prices-update", (event) => {
        prices_store.update(prices => ({
            ...prices,
            ...event.payload
        }));
    });
}

export const stats_store = writable({
    money: 0,
    spent_money: 0,
    playtime: 0,
    out_of_breath: 0,
    sniffed: 0,
});

export const init_stats = () => {
    listen<Stats>("synced-state://stats-update", (event) => {
        stats_store.update(stats => ({
            ...stats,
            ...event.payload
        }));
    });
}