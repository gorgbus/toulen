import { writable } from "svelte/store";

export let money_store = writable(0);
export let stamina_store = writable(1);
export let regen_store = writable(1);
export let auto_store = writable(1);

export let opened = writable(false);