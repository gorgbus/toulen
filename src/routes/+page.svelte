<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import {
        money_store,
        auto_store,
        regen_store,
        stamina_store,
        opened,
    } from "$lib/stores";
    import { load } from "$lib/util";
    import Upgrade from "$lib/components/Upgrade.svelte";
    import Money from "$lib/components/Money.svelte";
    import type { Unsubscriber } from "svelte/store";
    import UpgradeMenu from "$lib/components/UpgradeMenu.svelte";
    import MenuIcon from "$lib/components/icons/MenuIcon.svelte";
    import ShopIcon from "$lib/components/icons/ShopIcon.svelte";
    import SettingsIcon from "$lib/components/icons/SettingsIcon.svelte";
    import { goto } from "$app/navigation";
    import Tooltip from "$lib/components/Tooltip.svelte";

    let sniffing = false;
    let breath = 100;
    let unavailable = "";
    let cant_breathe = false;
    let auto_sniff = 1;
    let boost_dech = 1;
    let boost_prachy = 1;
    let max = false;
    let auto: Unsubscriber, regen: Unsubscriber, stamina: Unsubscriber;

    let dech_interval: NodeJS.Timer;
    let sniff_interval: NodeJS.Timeout;

    let upgrade = "";
    let tooltip = "";

    onMount(() => {
        load();

        auto = auto_store.subscribe((value) => {
            auto_sniff = value;
        });

        regen = regen_store.subscribe((value) => {
            boost_prachy = value;
        });

        stamina = stamina_store.subscribe((value) => {
            boost_dech = value;
        });
    });

    onDestroy(() => {
        auto();
        regen();
        stamina();

        clearInterval(dech_interval);
        clearTimeout(sniff_interval);
    });

    dech_interval = setInterval(() => {
        if (breath < 100) (breath += 1 * boost_dech) > 100 && (breath = 100);

        if (!cant_breathe)
            document
                .querySelector("#dech_bar")
                ?.setAttribute("style", `width: ${breath}%`);
    }, 200);

    const sniff = async () => {
        if (cant_breathe) return;

        money_store.update((value) => {
            return value + 1 * boost_prachy;
        });

        if (!sniffing)
            setTimeout(() => {
                sniffing = false;
            }, 1000);

        sniffing = true;
        breath -= 10;

        if (breath <= 0) {
            cant_breathe = true;

            document
                .querySelector("#dech_bar")
                ?.setAttribute("style", `width: 100%`);

            setTimeout(() => {
                cant_breathe = false;
                breath = 100;
            }, 20000);
        }
    };

    const auto_sniff_int = () => {
        if (auto_sniff > 1) sniff();

        sniff_interval = setTimeout(() => {
            auto_sniff_int();
        }, 5000 - 100 * auto_sniff);
    };

    auto_sniff_int();
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
            on:click={() => (upgrade = "auto sniff")}
            on:keydown={() => (upgrade = "auto sniff")}
        >
            <Upgrade
                name="auto sniff"
                value={max || 5000 - 100 * auto_sniff <= 0 ? "max" : auto_sniff}
                name_class={unavailable === "auto_sniff"
                    ? "text-red-500"
                    : "group-hover:text-amber-400 text-gray-100"}
            />
        </li>
        <li
            on:click={() => (upgrade = "dech")}
            on:keydown={() => (upgrade = "dech")}
        >
            <Upgrade
                name="dech"
                value={boost_dech}
                name_class={unavailable === "boost_dech"
                    ? "text-red-500"
                    : "group-hover:text-blue-400 text-gray-100"}
            />
        </li>
        <li
            on:click={() => (upgrade = "prachy")}
            on:keydown={() => (upgrade = "prachy")}
        >
            <Upgrade
                name="prachy"
                value={boost_prachy}
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
            >{cant_breathe ? "kokot se dusí" : "dech"}</label
        >
        <div id="dech" class="w-full h-4 rounded bg-gray-500">
            <div
                id="dech_bar"
                class={`rounded h-4 ${
                    cant_breathe ? "bg-red-600 cant_breath" : "bg-blue-400"
                }`}
            />
        </div>

        <div class="flex items-center pt-4">
            <div
                on:click={() => goto("/svinov")}
                on:keydown={() => goto("/svinov")}
                on:mouseover={() => (tooltip = "menu")}
                on:mouseout={() => (tooltip = "")}
                on:focus={() => (tooltip = "menu")}
                on:blur={() => (tooltip = "")}
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
    on:click={sniff}
    on:keypress={sniff}
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
