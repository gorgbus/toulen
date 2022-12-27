<script lang="ts">
    import {
        MONEY_PER_SNIFF,
        STAMINA_PER_AUTO,
        STAMINA_PER_SNIFF,
    } from "$lib/constants";
    import {
        add_money,
        add_sniffed_stats,
        player_store,
        stamina_store,
        subtract_stamina,
    } from "$lib/stores";
    import { onMount, onDestroy } from "svelte";

    let interval: NodeJS.Timeout;

    const sniff = () => {
        const auto = $player_store.auto;
        const stamina = $stamina_store;

        let sniffed_stamina =
            STAMINA_PER_SNIFF + STAMINA_PER_AUTO * Math.floor(auto / 2);

        if (stamina > 0) {
            subtract_stamina(sniffed_stamina);

            let added_money = MONEY_PER_SNIFF;

            if (auto > 10) added_money += auto - 10;

            add_sniffed_stats(1);

            add_money(added_money);
        }
    };

    onMount(() => {
        const auto_sniff_int = () => {
            if ($player_store.auto > 0) sniff();

            let timeout = 1000;

            if ($player_store.auto < 10) timeout *= 11 - $player_store.auto;

            interval = setTimeout(() => {
                auto_sniff_int();
            }, timeout);
        };

        auto_sniff_int();
    });

    onDestroy(() => {
        clearTimeout(interval);
    });
</script>
