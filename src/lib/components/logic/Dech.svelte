<script lang="ts">
    import { stamina_store, player_store, cant_breathe } from "$lib/stores";
    import {
        BASE_STAMINA,
        BASE_STAMINA_INTERVAL,
        STAMINA_PER_INTERVAL,
        STAMINA_PER_LEVEL,
        STAMINA_PER_REGEN,
    } from "$lib/constants";
    import { onDestroy, onMount } from "svelte";

    let interval: NodeJS.Timeout;

    onMount(() => {
        const dech_int = () => {
            const timeout =
                BASE_STAMINA_INTERVAL - STAMINA_PER_REGEN * $player_store.regen;

            interval = setTimeout(() => {
                stamina_store.update((stamina) => {
                    let max = BASE_STAMINA;
                    let is_breathing = false;

                    max += STAMINA_PER_LEVEL * $player_store.stamina;
                    is_breathing = $cant_breathe;

                    const new_stamina = stamina + STAMINA_PER_INTERVAL;

                    if (stamina < max && !is_breathing)
                        return new_stamina > max ? max : new_stamina;

                    return stamina;
                });

                dech_int();
            }, timeout);
        };

        dech_int();
    });

    onDestroy(() => {
        clearTimeout(interval);
    });
</script>
