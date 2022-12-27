<script>
    import { page } from "$app/stores";
    import { appWindow } from "@tauri-apps/api/window";
    import "../app.css";
    import { update_player } from "$lib/api";

    document.onkeydown = function (e) {
        if (e.key === "F5") {
            return false;
        }
    };

    document.oncontextmenu = function (e) {
        e.preventDefault();
    };

    const close = async () => {
        // await update_player();

        appWindow.close();
    };

    const minimize = () => {
        appWindow.minimize();
    };

    const maximize = () => {
        appWindow.toggleMaximize();
    };
</script>

<div
    data-tauri-drag-region
    id="title-bar"
    class="w-full h-4 fixed top-0 left-0 z-50 bg-zinc-800 flex items-center justify-between"
>
    <span class="text-sm font-semibold ml-4 text-zinc-400"
        >{$page.route.id === "/game"
            ? "Toulen Sniffer"
            : "Ostrava Svinov"}</span
    >

    <div class="flex items-center">
        <div
            on:click={minimize}
            on:keydown={minimize}
            class="hover:bg-zinc-700"
        >
            <img
                class="px-1 cursor-pointer title_bar_icons"
                src="/min.svg"
                alt="minimize"
            />
        </div>

        <div
            on:click={maximize}
            on:keydown={maximize}
            class="hover:bg-zinc-700"
        >
            <img
                class="px-1 cursor-pointer title_bar_icons"
                src="/max.svg"
                alt="maximize"
            />
        </div>

        <div on:click={close} on:keydown={close} class="hover:bg-red-600">
            <img
                class="px-1 cursor-pointer title_bar_icons"
                src="/close.svg"
                alt="close"
            />
        </div>
    </div>
</div>

<main class="w-full h-[calc(100vh-1rem)] mt-4 bg-slate-800 flex">
    <slot />
</main>
