<script lang="ts">
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { get_players, type Player } from "$lib/util";

    let players: Player[] = [];

    onMount(async () => {
        const res = await get_players();

        if (res) players = res;

        players = players.sort((a, b) => b.money - a.money);
    });

    let formatter = new Intl.NumberFormat("cs-CZ", {
        notation: "compact",
        style: "currency",
        currency: "CZK",
    });
</script>

<img
    class="opacity-10 w-full h-[calc(100%-1rem)] absolute z-0"
    src="/bg.jpg"
    alt="bg"
/>

<div class="w-full h-full flex items-center justify-center z-10">
    <div
        class="bg-zinc-800 bg-opacity-70 w-full h-[80%] rounded-lg shadow-xl m-32 p-4"
    >
        <div class="flex items-baseline justify-between">
            <h1 class="text-amber-500 font-bold text-xl">Top hráči</h1>

            <button
                on:click={() => goto("/svinov")}
                class="text-gray-100 font-bold text-sm transition-all hover:text-red-500 hover:underline"
            >
                zpět
            </button>
        </div>

        <div class="w-full h-[calc(100%-2rem)] relative">
            <table class="flex flex-col w-full max-h-full">
                <thead class="block w-full z-10">
                    <tr
                        class="text-gray-100 font-bold w-full h-4 table table-fixed"
                    >
                        <th class="text-left">Jméno</th>
                        <th class="text-left">Peníze</th>
                    </tr>
                </thead>

                <tbody
                    class="w-full block max-h-full overflow-hidden overflow-y-auto scrollbar"
                >
                    {#each players as player}
                        <tr
                            class="text-gray-300 font-semibold w-full h-4 table table-fixed"
                        >
                            <td>{player.user.name}</td>
                            <td>{formatter.format(player.money)}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </div>
</div>
