<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
    import { getVersion } from "@tauri-apps/api/app";
    import { relaunch } from "@tauri-apps/api/process";

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

    (async () => {
        const version = await getVersion();
        const update = await checkUpdate();

        if (!update.shouldUpdate) {
            state = "Aktuální verze";
            return;
        }

        state = "Aktualizace ke stažení";
    })();
</script>

<div class="flex flex-col items-center justify-center w-full h-full">
    <button
        class="text-xl text-gray-100 transition-all hover:text-blue-400 disabled:text-gray-500"
        on:click={start_game}
        disabled={state !== "Aktuální verze"}
    >
        Hrát
    </button>

    <button
        on:click={update}
        disabled={state !== "Aktualizace ke stažení"}
        class="text-gray-400 mt-4">{state}</button
    >
</div>
