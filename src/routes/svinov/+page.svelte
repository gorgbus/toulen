<script lang="ts">
    import { goto } from "$app/navigation";
    import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
    import { getVersion } from "@tauri-apps/api/app";
    import { relaunch } from "@tauri-apps/api/process";
    import { onMount } from "svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import Login from "$lib/components/Login.svelte";
    import { opened, player_store } from "$lib/stores";
    import { get_player, update_player } from "$lib/util";

    let state = "Vyhledávání aktualizací...";
    let logout = false;

    const update = async () => {
        state = "Stahuji aktualizaci...";

        await installUpdate();

        state = "Instaluji aktualizaci...";

        await relaunch();

        state = "Aktualizace nainstalována";
    };

    const start_game = async () => {
        goto("/");
    };

    const close_game = async () => {
        await update_player();

        appWindow.close();
    };

    const fn_logout = () => {
        player_store.update((player) => ({
            ...player,
            id: -1,
        }));
        logout = false;

        localStorage.removeItem("access_token");
    };

    onMount(async () => {
        if ($player_store.id === -1) {
            const player = await get_player();

            if (player) {
                player_store.set(player);
            }
        }

        const version = await getVersion();
        const update = await checkUpdate();

        if (!update.shouldUpdate) {
            state = `Aktuální verze v${version}`;
            return;
        }

        state = "Dostupná aktualizace ke stažení";
    });
</script>

<img
    class="opacity-10 w-full h-[calc(100%-1rem)] absolute z-0"
    src="/bg.jpg"
    alt="bg"
/>

<div class="flex flex-col items-center justify-center w-full h-full z-10">
    <div class="flex items-center mb-32 -mt-16">
        <img class="w-24 h-16" src="/toulen.png" alt="logo" />

        <h1 class="text-4xl text-gray-100">Sniffer</h1>
    </div>

    <button
        class="text-xl text-gray-100 transition-all font-semibold hover:text-blue-400 disabled:text-gray-500"
        on:click={start_game}
        disabled={!state.startsWith("Aktuální verze ")}
    >
        Hrát
    </button>

    <button
        on:click={() => goto("/leaderboards")}
        class="text-gray-100 transition-all font-semibold hover:text-blue-400 disabled:text-gray-500"
    >
        Žebříčky
    </button>

    <button
        on:click={close_game}
        class="text-gray-100 transition-all font-semibold hover:text-red-500"
    >
        Odejít
    </button>

    <button
        on:click={update}
        disabled={state !== "Dostupná aktualizace ke stažení"}
        class="text-green-500 mt-4 disabled:text-gray-400">{state}</button
    >

    <button
        on:click={() => opened.set(true)}
        disabled={$player_store.id !== -1}
        class="text-gray-100 absolute left-4 bottom-4"
    >
        {#if $player_store.id === -1}
            Přihlásit se
        {:else}
            <span>Přihlášen jako </span>
            <button
                on:click={() => (logout = !logout)}
                class="font-semibold text-green-500 relative"
            >
                {$player_store.user.name}
                <span
                    on:click={fn_logout}
                    on:keydown={fn_logout}
                    class={`${
                        logout ? "block" : "hidden"
                    } bottom-full absolute left-1/2 -translate-x-1/2 w-24 p-1 text-xs bg-gray-700 rounded text-red-500 shadow-lg`}
                    >Odhlásit se</span
                >
            </button>
        {/if}
    </button>

    {#if $opened}
        <Login />
    {/if}
</div>
