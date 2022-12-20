<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { money_store } from "$lib/stores";
    import type { Unsubscriber } from "svelte/store";

    let money: number;
    let unsub: Unsubscriber;
    let anim: NodeJS.Timeout;
    let add_remove = 0;

    let formatter = new Intl.NumberFormat("cs-CZ", {
        notation: "compact",
        style: "currency",
        currency: "CZK",
    });

    onMount(async () => {
        if (!Number(localStorage.getItem("money")))
            localStorage.setItem("money", "0");
        else money_store.set(Number(localStorage.getItem("money")));

        unsub = money_store.subscribe((value) => {
            localStorage.setItem("money", value.toString());

            add_remove = value - money;
            clearTimeout(anim);

            anim = setTimeout(() => {
                add_remove = 0;
            }, 1000);

            money = value;
        });
    });

    onDestroy(() => {
        unsub();
    });
</script>

<div
    class="w-full h-12 flex items-center rounded bg-gray-700 group cursor-pointer my-2"
>
    <div class="px-3 flex justify-between items-center w-full">
        <h1 class="font-semibold text-yellow-400 text-xl">
            {formatter.format(money)}
        </h1>

        <span
            class={`font-semibold ${add_remove === 0 && "hidden"} ${
                add_remove > 0 ? "text-green-500 add" : "text-red-400 remove"
            }`}
            >{add_remove > 0 ? "+" : ""}
            {formatter.format(add_remove)}</span
        >
    </div>
</div>
