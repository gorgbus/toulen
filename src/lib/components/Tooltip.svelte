<script lang="ts">
    export let content: string;
    export let shown: string;
    export let id: string;

    let visible = shown === id;

    console.log(visible);
</script>

<div
    {id}
    role="tooltip"
    class={`inline-block absolute ${
        visible ? "" : "invisible"
    } z-10 py-1 px-3 text-sm font-medium text-white bg-gray-900 rounded-lg shadow-sm opacity-0 transition-opacity duration-300 tooltip top-0 dark:bg-gray-700`}
>
    {content}
</div>
