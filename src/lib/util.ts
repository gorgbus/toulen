import { stamina_store, auto_store, regen_store } from "./stores";
import { invoke } from "@tauri-apps/api/tauri";

export const load = async () => {
    const data: string = await invoke("get_save");
    const save = data.split(" ");

    if (save.length === 4) {
        localStorage.setItem("money", save[0]);
        localStorage.setItem("auto", save[1]);
        localStorage.setItem("stamina", save[2]);
        localStorage.setItem("regen", save[3]);
    } else {
        stamina_store.set(Number(localStorage.getItem("stamina")));
        auto_store.set(Number(localStorage.getItem("auto")));
        regen_store.set(Number(localStorage.getItem("regen")));
    }
}