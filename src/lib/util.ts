import { player_store, stats_store } from "./stores";
import { invoke } from "@tauri-apps/api/tauri";

export const load = async () => {
    const data: string = await invoke("get_save");
    const save = data.split(" ");

    if (save.length === 4) {
        localStorage.setItem("money", save[0]);
        localStorage.setItem("auto", save[1]);
        localStorage.setItem("stamina", save[2]);
        localStorage.setItem("regen", save[3]);

        // money_store.set(Number(save[0]));
        // stamina_store.set(Number(save[2]));
        // auto_store.set(Number(save[1]));
        // regen_store.set(Number(save[3]));
    } else {
        player_store.update(player => ({
            ...player,
            money: Number(localStorage.getItem("money_d")) || 0,
            stamina: Number(localStorage.getItem("stamina_d")) || 0,
            regen: Number(localStorage.getItem("regen_d")) || 0,
            auto: Number(localStorage.getItem("auto_d")) || 0
        }));

        stats_store.update(_ => ({
            money: Number(localStorage.getItem("money_s")) || 0,
            out_of_breath: Number(localStorage.getItem("out_of_breath_s")) || 0,
            sniffed: Number(localStorage.getItem("sniffed_s")) || 0,
            playtime: Number(localStorage.getItem("playtime_s")) || 0,
            spent_money: Number(localStorage.getItem("spent_money_s")) || 0
        }));

        // stamina_store.set(Number(localStorage.getItem("stamina")));
        // auto_store.set(Number(localStorage.getItem("auto")));
        // regen_store.set(Number(localStorage.getItem("regen")));
    }
}

export const formatter = new Intl.NumberFormat("cs-CZ", {
    notation: "compact",
    style: "currency",
    currency: "CZK",
});