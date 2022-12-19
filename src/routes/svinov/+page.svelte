<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
    import { getVersion } from "@tauri-apps/api/app";
    import { relaunch } from "@tauri-apps/api/process";
    import { appWindow } from "@tauri-apps/api/window";

    let state = "Vyhledávání aktualizací...";

    const update = async () => {
        state = "Stahuji aktualizaci...";

        await installUpdate();

        state = "Instaluji aktualizaci...";

        await relaunch();

        state = "Aktualizace nainstalována";
    };

    const start_game = async () => {
        await invoke("open_game");
    };

    const close_game = async () => {
        await invoke("close_game");
    };

    (async () => {
        const version = await getVersion();
        const update = await checkUpdate();

        if (!update.shouldUpdate) {
            state = `Aktuální verze v${version}`;
            return;
        }

        state = "Dostupná aktualizace ke stažení";
    })();
</script>

<img class="opacity-10 w-full h-full absolute z-0" src="/bg.jpg" alt="bg" />

<div class="flex flex-col items-center justify-center w-full h-full z-10">
    <div class="flex items-center mb-32 -mt-16">
        <img class="w-24 h-16" src="/toulen.png" alt="logo" />

        <h1 class="text-4xl text-gray-100">Sniffer</h1>
    </div>

    <button
        class="text-xl text-gray-100 transition-all hover:text-blue-400 disabled:text-gray-500"
        on:click={start_game}
        disabled={!state.startsWith("Aktuální verze ")}
    >
        Hrát
    </button>

    <button
        on:click={close_game}
        class="text-gray-100 transition-all hover:text-red-500"
    >
        Odejít
    </button>

    <button
        on:click={update}
        disabled={state !== "Dostupná aktualizace ke stažení"}
        class="text-green-500 mt-4 disabled:text-gray-400">{state}</button
    >
</div>
