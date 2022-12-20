<script lang="ts">
    import { opened } from "$lib/stores";

    export let name: string;
    export let value: number | string;
    export let name_class: string;
</script>

<div
    on:keypress={() => opened.set(true)}
    on:click={() => opened.set(true)}
    class="w-full h-16 rounded bg-gray-700 group cursor-pointer my-2"
>
    <div class="px-3 py-2">
        <h2 class={`font-semibold transition-all ${name_class}`}>{name}</h2>
        <h3 class="font-semibold text-sm text-yellow-500">lvl. {value}</h3>
    </div>
</div>
