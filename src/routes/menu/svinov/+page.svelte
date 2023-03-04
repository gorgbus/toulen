<script lang="ts">
    import { goto } from "$app/navigation";
    import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
    import { getVersion } from "@tauri-apps/api/app";
    import { relaunch } from "@tauri-apps/api/process";
    import { onMount } from "svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import {
        init_login,
        init_player,
        init_prices,
        init_stats,
        init_tweakdata,
        launch,
        player_store,
    } from "$lib/stores";
    import { invoke } from "@tauri-apps/api/tauri";
    import { with_auth } from "$lib/util";

    let state = "Vyhledávání aktualizací...";
    let logout = false;

    const update = async () => {
        state = "Stahuji aktualizaci...";

        await installUpdate();

        state = "Instaluji aktualizaci...";

        await relaunch();

        state = "Aktualizace nainstalována";
    };

    onMount(async () => {
        await invoke("activity_main_menu");

        if (!$launch) {
            launch.set(true);

            init_player();
            init_prices();
            init_stats();
            init_login();
            init_tweakdata();

            await invoke("init_states");
        }

        const version = await getVersion();
        const update = await checkUpdate();

        if (!update.shouldUpdate) {
            state = `Aktuální verze v${version}`;
            return;
        }

        state = "Dostupná aktualizace ke stažení";
    });

    const auth = with_auth();

    let logged_in = auth.logged_in;
</script>

<div class="flex flex-col items-center justify-center w-full h-full z-10">
    <div class="flex items-center mb-32 -mt-16">
        <img class="w-24 h-16" src="/toulen.png" alt="logo" />

        <h1 class="text-4xl text-gray-100">Sniffer</h1>
    </div>

    <button
        class="text-xl text-gray-100 transition-all font-semibold hover:text-blue-400 disabled:text-gray-500"
        on:click={() => goto("/game")}
        disabled={!state.startsWith("Aktuální verze ")}
    >
        Hrát
    </button>

    <button
        on:click={() => goto("/menu/leaderboards")}
        class="text-gray-100 transition-all font-semibold hover:text-blue-400"
        disabled={!state.startsWith("Aktuální verze ")}
    >
        Žebříčky
    </button>

    <button
        on:click={() => goto("/menu/stats")}
        class="text-gray-100 transition-all font-semibold hover:text-blue-400"
        disabled={!state.startsWith("Aktuální verze ")}
    >
        Statistiky
    </button>

    <button
        on:click={appWindow.close}
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
        on:click={auth.login}
        disabled={$logged_in.success}
        class="text-gray-100 absolute left-4 bottom-4"
    >
        {#if !$logged_in.success}
            Přihlásit se
        {:else}
            <span>Přihlášen jako </span>
            <button
                on:click={() => (logout = !logout)}
                class="font-semibold text-green-500 relative"
            >
                {$player_store.user.name || "chyba"}
                <span
                    on:click={auth.logout}
                    on:keydown={auth.logout}
                    class={`${
                        logout ? "block" : "hidden"
                    } bottom-full absolute left-1/2 -translate-x-1/2 w-24 p-1 text-xs bg-gray-700 rounded text-red-500 shadow-lg`}
                    >Odhlásit se</span
                >
            </button>
        {/if}
    </button>
</div>
