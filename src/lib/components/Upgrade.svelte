<script lang="ts">
    import { opened, player_store } from "$lib/stores";
    import { formatter } from "$lib/util";

    export let name: string;
    export let value: number | string;
    export let name_class: string;

    let max = false;

    $: {
        switch (name) {
            case "auto":
                max = $player_store.auto >= 100;
                break;
            case "regen":
                max = $player_store.regen >= 100;
                break;
            case "stamina":
                max = $player_store.stamina >= 100;
                break;
        }
    }
</script>

<div
    on:keypress={() => opened.set(true)}
    on:click={() => opened.set(true)}
    class="w-full h-16 rounded bg-gray-700 group cursor-pointer my-2"
>
    <div class="px-3 py-1">
        <h2 class={`font-semibold transition-all ${name_class}`}>{name}</h2>

        <h3 class="font-semibold text-sm text-yellow-500 inline">
            {max ? "lvl. max" : `lvl. ${value}`}
        </h3>
    </div>
</div>
