<script lang="ts">
    import { opened, player_store } from "$lib/stores";
    import Upgrade from "$lib/components/Upgrade.svelte";
    import Money from "$lib/components/Money.svelte";
    import UpgradeMenu from "$lib/components/UpgradeMenu.svelte";
    import MenuIcon from "$lib/components/icons/MenuIcon.svelte";
    import ShopIcon from "$lib/components/icons/ShopIcon.svelte";
    import SettingsIcon from "$lib/components/icons/SettingsIcon.svelte";
    import { goto } from "$app/navigation";
    import { BASE_STAMINA, STAMINA_PER_LEVEL } from "$lib/constants";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onDestroy, onMount } from "svelte";

    let sniffing = false;
    let unavailable = "";

    let upgrade = "";

    let max_breath = BASE_STAMINA;

    onMount(async () => {
        await invoke("start_breath");
        await invoke("start_auto");
        await invoke("activity_game");
    });

    onDestroy(async () => {
        await invoke("stop_breath");
        await invoke("stop_auto");
    });

    $: {
        max_breath =
            BASE_STAMINA + STAMINA_PER_LEVEL * $player_store.stamina_lvl;
    }
</script>

<div
    class="bg-slate-700 h-full w-96 bg-opacity-25 p-4 flex flex-col justify-between"
>
    <ul class="text-gray-100 text-sm">
        <li>
            <Money />
        </li>
        <li><br /></li>
        <li
            on:click={() => (upgrade = "auto")}
            on:keydown={() => (upgrade = "auto")}
        >
            <Upgrade
                name="auto"
                value={$player_store.auto_lvl}
                name_class={unavailable === "auto_sniff"
                    ? "text-red-500"
                    : "group-hover:text-amber-400 text-gray-100"}
            />
        </li>
        <li
            on:click={() => (upgrade = "stamina")}
            on:keydown={() => (upgrade = "stamina")}
        >
            <Upgrade
                name="stamina"
                value={$player_store.stamina_lvl}
                name_class={unavailable === "boost_dech"
                    ? "text-red-500"
                    : "group-hover:text-blue-400 text-gray-100"}
            />
        </li>
        <li
            on:click={() => (upgrade = "regen")}
            on:keydown={() => (upgrade = "regen")}
        >
            <Upgrade
                name="regen"
                value={$player_store.regen_lvl}
                name_class={unavailable === "boost_prachy"
                    ? "text-red-500"
                    : "group-hover:text-green-400 text-gray-100"}
            />
        </li>
    </ul>

    {#if $opened}
        <UpgradeMenu {upgrade} />
    {/if}

    <div>
        <label class="text-gray-100 font-semibold" for="dech"
            >{!$player_store.can_breathe ? "kokot se dusí" : "dech"}</label
        >
        <div
            id="dech"
            class="w-full h-4 rounded bg-gray-500 text-center relative"
        >
            <div
                id="dech_bar"
                class={`rounded h-4 ${
                    !$player_store.can_breathe
                        ? "bg-red-600 cant_breath"
                        : "bg-blue-400"
                }`}
                style={`width: ${
                    !$player_store.can_breathe
                        ? 100
                        : ($player_store.stamina / max_breath) * 100
                }%`}
            />

            <span class="absolute top-0 left-2 text-black text-xs font-semibold"
                >{$player_store.stamina}/{max_breath}</span
            >
        </div>

        <div class="flex items-center pt-4">
            <div
                on:click={() => goto("/menu/svinov")}
                on:keydown={() => goto("/menu/svinov")}
                class="p-2 rounded cursor-pointer bg-gray-700 hover:bg-gray-600 transition-all relative"
            >
                <MenuIcon />
            </div>

            <div
                class="p-2 mx-4 rounded cursor-pointer bg-gray-700 hover:bg-gray-600 transition-all relative"
            >
                <ShopIcon />
            </div>

            <div
                class="p-2 rounded cursor-pointer bg-gray-700 hover:bg-gray-600 transition-all relative"
            >
                <SettingsIcon />
            </div>
        </div>
    </div>
</div>

<div
    on:click={async () => await invoke("sniff")}
    on:keypress={async () => await invoke("sniff")}
    class="w-full h-full relative flex flex-col items-center justify-end overflow-hidden cursor-pointer"
>
    <img
        class="w-full max-w-[50rem] -mb-[5%] h-full max-h-[50rem]"
        src="/cygos.svg"
        alt="cygos"
    />

    <img
        class={`absolute bottom-32 w-full h-full max-w-[25rem] max-h-[15rem] ${
            sniffing ? "sniff" : "chill"
        }`}
        src="/toulen.svg"
        alt="toulen"
    />
</div>
