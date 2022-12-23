<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import {
        auto_store,
        money_store,
        opened,
        regen_store,
        stamina_store,
    } from "$lib/stores";
    import type { Unsubscriber } from "svelte/store";

    export let upgrade: string;

    let formatter = new Intl.NumberFormat("cs-CZ", {
        notation: "compact",
        style: "currency",
        currency: "CZK",
    });

    let auto: number = 1,
        regen: number = 1,
        stamina: number = 1,
        money: number = 0;
    let auto_sub: Unsubscriber,
        regen_sub: Unsubscriber,
        stamina_sub: Unsubscriber,
        money_sub: Unsubscriber;

    let max = false;

    const click = (e: MouseEvent) => {
        if (e.target instanceof HTMLElement && e.target !== null)
            if (e.target?.classList.contains("pozadi")) opened.set(false);
    };

    onMount(() => {
        document.addEventListener("click", click);

        auto_sub = auto_store.subscribe((value) => {
            auto = value;
        });

        regen_sub = regen_store.subscribe((value) => {
            regen = value;
        });

        stamina_sub = stamina_store.subscribe((value) => {
            stamina = value;
        });

        money_sub = money_store.subscribe((value) => {
            money = value;
        });
    });

    onDestroy(() => {
        document.removeEventListener("click", click);

        auto_sub();
        regen_sub();
        stamina_sub();
        money_sub();
    });

    let cost = 0;

    $: {
        if (upgrade === "auto sniff") cost = 250 * auto;
        if (upgrade === "dech") cost = 200 * stamina;
        if (upgrade === "prachy") cost = 500 * regen;
    }

    const add_money = (amount: number) => {
        money_store.update((value) => {
            return value + amount;
        });
    };

    const buy = () => {
        if (upgrade === "auto sniff") {
            if (money >= 250 * auto) {
                if (5000 - (auto + 1) * 100 === 0) {
                    return (max = true);
                }

                add_money(-250 * auto);

                auto_store.update((value) => {
                    value++;

                    localStorage.setItem("auto", `${value}`);

                    return value;
                });

                if (5000 - auto * 100 === 0) {
                    return (max = true);
                }
            }
        }

        if (upgrade === "dech") {
            if (money >= 200 * stamina) {
                add_money(-200 * stamina);

                stamina_store.update((value) => {
                    value++;

                    localStorage.setItem("stamina", `${value}`);

                    return value;
                });
            }
        }

        if (upgrade === "prachy") {
            if (money >= 500 * regen) {
                add_money(-500 * regen);

                regen_store.update((value) => {
                    value++;

                    localStorage.setItem("regen", `${value}`);

                    return value;
                });
            }
        }
    };

    let can_buy = true;

    $: {
        if (cost > money) can_buy = false;
        else can_buy = true;

        if (max) can_buy = false;
    }
</script>

<div
    class="flex items-center justify-center w-full h-full bg-black absolute top-0 left-0 z-20 bg-opacity-40 pozadi"
>
    <div
        class="w-[400px] h-[400px] absolute bg-gray-800 rounded menu flex items-center justify-center flex-col"
    >
        <h1 class="font-semibold text-gray-100 text-lg mb-4">
            upgrade {upgrade}
        </h1>

        <img class="w-32 h-32" src="/toulen.svg" alt="toulen" />

        <button
            disabled={!can_buy}
            on:click={buy}
            class="rounded w-48 mt-4 h-12 bg-gray-700 font-semibold text-yellow-400 text-xl transition-all hover:bg-gray-600 disabled:opacity-60 disabled:text-red-600"
        >
            {formatter.format(cost)}
        </button>
    </div>
</div>
