<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import {
        opened,
        player_store,
        prices_store,
        tweakdata_store,
    } from "$lib/stores";
    import { formatter } from "$lib/util";
    import { invoke } from "@tauri-apps/api/tauri";

    export let upgrade: string;

    let max = false;

    const click = (e: MouseEvent) => {
        if (e.target instanceof HTMLElement && e.target !== null)
            if (e.target?.classList.contains("pozadi")) opened.set(false);
    };

    onMount(() => {
        document.addEventListener("click", click);
    });

    onDestroy(() => {
        document.removeEventListener("click", click);
    });

    let cost = 0;
    let desc = "";
    let reason = "";

    let can_buy = true;
    let too_big_diff = false;

    $: {
        switch (upgrade) {
            case "auto":
                cost = $prices_store.auto;

                max = $player_store.auto_lvl >= 100;

                const value =
                    $player_store.auto_lvl > 10
                        ? 1 + $player_store.auto_lvl - 10
                        : $player_store.auto_lvl === 0
                        ? 0
                        : 1;

                const time =
                    $player_store.auto_lvl < 10
                        ? $player_store.auto_lvl === 0
                            ? 10
                            : 11 - $player_store.auto_lvl
                        : 1;

                desc =
                    $player_store.auto_lvl === 0
                        ? "Momentálně nezískáváš nic."
                        : `Momentálně získáváš ${formatter.format(value)} za ${
                              time === 1
                                  ? "sekundu"
                                  : [2, 3, 4].includes(time)
                                  ? `${time} sekundy`
                                  : `${time} sekund`
                          }. S každým druhým upgradem se zvýší spotřeba dechu o ${
                              $tweakdata_store.stamina_per_auto
                          }.`;

                break;

            case "stamina":
                cost = $prices_store.stamina;

                max = $player_store.stamina_lvl >= 100;

                desc =
                    $player_store.stamina_lvl === 0
                        ? "Momentálně nemáš žádný dech navíc."
                        : `Momentálně máš bonus +${
                              $player_store.stamina_lvl *
                              $tweakdata_store.stamina_per_level
                          } dechu navíc.`;

                break;

            case "regen":
                cost = $prices_store.regen;

                max = $player_store.regen_lvl >= 100;

                desc =
                    $player_store.regen_lvl === 0
                        ? "Momentálně nedostáváš žádný bonusový dech při nádechu."
                        : `Momentálně dostáváš +${Math.floor(
                              $player_store.regen_lvl / 3
                          )} bonusového dechu při nádechu.`;

                break;
        }

        if (cost > $player_store.money) can_buy = false;
        else can_buy = true;

        if (
            upgrade === "auto" &&
            $player_store.auto_lvl - $player_store.stamina_lvl >= 2
        ) {
            too_big_diff = true;

            reason = `Nejdříve musíš zvýšit maximální dech aspoň o jeden level.`;
        } else {
            too_big_diff = false;
            reason = "";
        }

        if (max) can_buy = false;
    }

    const buy = async (ability: string) => {
        await invoke("buy", {
            ability,
        });
    };
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

        <p class="max-w-[35ch] text-center text-gray-300 text-sm font-semibold">
            {desc}
        </p>

        <button
            disabled={!can_buy || too_big_diff}
            on:click={async () => await buy(upgrade)}
            class={`rounded w-48 mt-4 h-12 bg-gray-700 font-semibold text-yellow-400 text-xl transition-all ${
                max
                    ? ""
                    : "disabled:opacity-60 disabled:text-red-600 hover:bg-gray-600"
            }`}
        >
            {max ? "max" : formatter.format(cost)}
        </button>

        <p class="text-red-500 text-xs mt-4 font-semibold">{reason}</p>
    </div>
</div>
