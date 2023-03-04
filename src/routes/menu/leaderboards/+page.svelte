<script lang="ts">
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { LeaderboardPlayer as Player } from "$bindings/LeaderboardPlayer";

    let players: Player[] = [];

    type LPlayer = Player & {
        level: number;
    };

    let sorted_players: LPlayer[] = [];

    onMount(async () => {
        const res = await invoke<Player[]>("get_players_cmd");

        if (res) players = res;

        sorted_players = players
            .map((player) => ({
                ...player,
                level: player.auto_lvl + player.regen_lvl + player.stamina_lvl,
            }))
            .sort((a, b) => b.level - a.level);
    });
</script>

<div class="w-full h-full flex items-center justify-center z-10">
    <div
        class="bg-zinc-800 bg-opacity-70 w-full h-[80%] rounded-lg shadow-xl m-32 p-4"
    >
        <div class="flex items-baseline justify-between">
            <h1 class="text-amber-500 font-bold text-xl">Top hráči</h1>

            <button
                on:click={() => goto("/menu/svinov")}
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
                        <th class="text-left">Level</th>
                    </tr>
                </thead>

                <tbody
                    class="w-full block max-h-full overflow-hidden overflow-y-auto scrollbar"
                >
                    {#each sorted_players as player}
                        <tr
                            class="text-gray-300 font-semibold w-full h-4 table table-fixed"
                        >
                            <td>{player.user.name}</td>
                            <td>lvl. {player.level}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </div>
</div>
