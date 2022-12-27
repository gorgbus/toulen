<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { player_store } from "$lib/stores";
    import type { Unsubscriber } from "svelte/store";
    import { formatter } from "$lib/util";

    let current_money: number;
    let unsub: Unsubscriber;
    let anim: NodeJS.Timeout;
    let add_remove = 0;

    onMount(async () => {
        unsub = player_store.subscribe(({ money }) => {
            if (Number(money - current_money))
                add_remove = money - current_money;
            clearTimeout(anim);

            anim = setTimeout(() => {
                add_remove = 0;
            }, 1000);

            current_money = money;
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
            {formatter.format(current_money)}
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
