import { writable } from "svelte/store";
import { BASE_STAMINA } from "./constants";


export const cant_breathe = writable(false);
export const stamina_store = writable(BASE_STAMINA);

export const subtract_stamina = (amount: number) => {
    stamina_store.update(stamina => {
        if (stamina - amount <= 0) {
            cant_breathe.set(true);

            add_out_of_breath_stats(1);

            setTimeout(() => {
                cant_breathe.set(false);
            }, 20000);
        }

        if (stamina - amount < 0) return 0;

        return stamina - amount;
    });
}

export const opened = writable(false);

export const player_store = writable({
    id: -1,
    money: 0,
    stamina: 0,
    regen: 0,
    auto: 0,
    toulen: 0,
    user: {
        name: "none"
    }
});

export const add_money = (amount: number) => {
    localStorage.setItem("money_d", (Number(localStorage.getItem("money_d") || 0) + amount).toString());

    if (amount > 0) add_money_stats(amount);
    else add_spent_money_stats(amount);

    player_store.update(player => ({
        ...player,
        money: player.money + amount
    }));
}

export const add_stamina = (amount: number) => {
    localStorage.setItem("stamina_d", (Number(localStorage.getItem("stamina_d") || 0) + amount).toString());

    player_store.update(player => ({
        ...player,
        stamina: player.stamina + amount
    }));
}

export const add_regen = (amount: number) => {
    localStorage.setItem("regen_d", (Number(localStorage.getItem("regen_d") || 0) + amount).toString());

    player_store.update(player => ({
        ...player,
        regen: player.regen + amount
    }));
}

export const add_auto = (amount: number) => {
    localStorage.setItem("auto_d", (Number(localStorage.getItem("auto_d") || 0) + amount).toString());

    player_store.update(player => ({
        ...player,
        auto: player.auto + amount
    }));
}

export const stats_store = writable({
    money: 0,
    spent_money: 0,
    playtime: 0,
    out_of_breath: 0,
    sniffed: 0,
});

export const add_money_stats = (amount: number) => {
    localStorage.setItem("money_s", (Number(localStorage.getItem("money_s") || 0) + amount).toString());

    stats_store.update(stats => ({
        ...stats,
        money: stats.money + amount
    }));
}

export const add_spent_money_stats = (amount: number) => {
    localStorage.setItem("spent_money_s", (Number(localStorage.getItem("spent_money_s") || 0) + amount).toString());

    stats_store.update(stats => ({
        ...stats,
        spent_money: stats.spent_money + amount
    }));
}

export const add_playtime_stats = (amount: number) => {
    localStorage.setItem("playtime_s", (Number(localStorage.getItem("playtime_s") || 0) + amount).toString());

    stats_store.update(stats => ({
        ...stats,
        playtime: stats.playtime + amount
    }));
}

export const add_out_of_breath_stats = (amount: number) => {
    localStorage.setItem("out_of_breath_s", (Number(localStorage.getItem("out_of_breath_s") || 0) + amount).toString());

    stats_store.update(stats => ({
        ...stats,
        out_of_breath: stats.out_of_breath + amount
    }));
}

export const add_sniffed_stats = (amount: number) => {
    localStorage.setItem("sniffed_s", (Number(localStorage.getItem("sniffed_s") || 0) + amount).toString());

    stats_store.update(stats => ({
        ...stats,
        sniffed: stats.sniffed + amount
    }));
}