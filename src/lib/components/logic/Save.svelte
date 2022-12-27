<script lang="ts">
    import { update_player } from "$lib/api";
    import { add_playtime_stats, player_store } from "$lib/stores";
    import { onMount, onDestroy } from "svelte";

    let interval: NodeJS.Timer;

    onMount(() => {
        interval = setInterval(async () => {
            add_playtime_stats(1);

            // if ($player_store.id !== -1) await update_player();
        }, 60000);
    });

    onDestroy(() => {
        clearInterval(interval);
    });
</script>
