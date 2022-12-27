<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import {
        add_auto,
        add_money,
        add_regen,
        add_stamina,
        opened,
        player_store,
    } from "$lib/stores";
    import { formatter } from "$lib/util";
    import Decimal from "decimal.js";
    import {
        BASE_AUTO_COST,
        BASE_REGEN_COST,
        BASE_STAMINA_COST,
        STAMINA_PER_AUTO,
        STAMINA_PER_LEVEL,
    } from "$lib/constants";

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

    const get_cost = (base: number, level: number) => {
        if (level === 0) return base;

        const powers = [0, 0.03, 0.06];

        const power = new Decimal(level)
            .dividedToIntegerBy(3)
            .dividedBy(10)
            .plus(powers[level % 3])
            .plus(1);

        return new Decimal(base).pow(power).floor().toNumber();
    };

    let can_buy = true;

    $: {
        switch (upgrade) {
            case "auto":
                cost = get_cost(BASE_AUTO_COST, $player_store.auto);

                max = $player_store.auto >= 100;

                const value =
                    $player_store.auto > 10
                        ? 1 + $player_store.auto - 10
                        : $player_store.auto === 0
                        ? 0
                        : 1;

                const time =
                    $player_store.auto < 10
                        ? $player_store.auto === 0
                            ? 10
                            : 11 - $player_store.auto
                        : 1;

                desc =
                    $player_store.auto === 0
                        ? "Momentálně nezískáváš nic."
                        : `Momentálně získáváš ${formatter.format(value)} za ${
                              time === 1
                                  ? "sekundu"
                                  : [2, 3, 4].includes(time)
                                  ? `${time} sekundy`
                                  : `${time} sekund`
                          }. S každým druhým upgradem se zvýší spotřeba dechu o ${STAMINA_PER_AUTO}.`;

                break;

            case "stamina":
                cost = get_cost(BASE_STAMINA_COST, $player_store.stamina);

                max = $player_store.stamina >= 100;

                desc =
                    $player_store.stamina === 0
                        ? "Momentálně nemáš žádný dech navíc."
                        : `Momentálně máš bonus +${
                              $player_store.stamina * STAMINA_PER_LEVEL
                          } dechu navíc.`;

                break;

            case "regen":
                cost = get_cost(BASE_REGEN_COST, $player_store.regen);

                max = $player_store.regen >= 100;

                desc =
                    $player_store.regen === 0
                        ? "Momentálně nemáš žádnou bonusovou rychlost nádechu."
                        : `Momentálně máš bonus +${$player_store.regen}% k rychlosti nádechu.`;

                break;
        }

        if (cost > $player_store.money) can_buy = false;
        else if (
            upgrade === "auto" &&
            $player_store.auto - $player_store.stamina >= 2
        ) {
            can_buy = false;

            reason = `Nejdříve musíš zvýšit maximální dech aspoň o jeden level.`;
        } else can_buy = true;

        if (max) can_buy = false;
    }

    const buy = () => {
        if ($player_store.money >= cost) {
            add_money(-cost);

            switch (upgrade) {
                case "auto":
                    add_auto(1);
                    break;

                case "stamina":
                    add_stamina(1);
                    break;

                case "regen":
                    add_regen(1);
                    break;
            }
        }
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
            disabled={!can_buy}
            on:click={buy}
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
