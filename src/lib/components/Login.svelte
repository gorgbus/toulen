<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { opened, player_store } from "$lib/stores";
    import {
        create_player,
        fn_login,
        get_player,
        register,
        update_player,
    } from "$lib/api";

    let loading = false;

    const click = (e: MouseEvent) => {
        if (e.target instanceof HTMLElement && e.target !== null)
            if (e.target?.classList.contains("pozadi") && !loading)
                opened.set(false);
    };

    onMount(() => {
        document.addEventListener("click", click);
    });

    onDestroy(() => {
        document.removeEventListener("click", click);
    });

    let login = true;
    let status = "";

    let name = "";
    let password = "";
    let password_again = "";

    const submit = async (e: Event) => {
        e.preventDefault();

        loading = true;

        if (login) {
            const res = await fn_login(name, password);

            if (res.status === 401) {
                loading = false;

                return (status = "Špatné jméno nebo heslo");
            }
            if (!res.status || !res.data) {
                loading = false;

                return (status = "Chyba při přihlašování");
            }

            localStorage.setItem("access_token", res.data.access_token);
            localStorage.setItem("user_id", res.data.id);

            const player = await get_player();

            if (!player) {
                loading = false;

                return (status = "Chyba při přihlašování");
            }

            player_store.set(player);

            localStorage.setItem("money", player.money.toString());
            localStorage.setItem("stamina", player.stamina.toString());
            localStorage.setItem("regen", player.regen.toString());
            localStorage.setItem("auto", player.auto.toString());

            loading = false;

            opened.set(false);

            return (status = "");
        }

        if (password !== password_again) {
            loading = false;

            return (status = "Hesla se neshodují");
        }

        const user = await register(name, password);

        if (user.status === 409) {
            loading = false;

            return (status = `Jméno ${name} není dostupné`);
        }

        if (!user.status || !user.data) {
            loading = false;

            return (status = "Chyba při registraci");
        }

        const token = await fn_login(name, password);

        if (!token.status || !token.data) {
            loading = false;

            return (status = "Chyba při registraci");
        }

        localStorage.setItem("access_token", token.data.access_token);
        localStorage.setItem("user_id", token.data.id);

        const player = await create_player(token.data.id);

        if (!player) {
            loading = false;

            return (status = "Chyba při registraci");
        }

        player_store.set(player);

        await update_player();

        loading = false;

        opened.set(false);

        return (status = "");
    };
</script>

<div
    class="flex items-center justify-center w-full h-full bg-black absolute top-0 left-0 z-20 bg-opacity-40 pozadi"
>
    <div class="w-64 h-80 bg-gray-800 rounded">
        <form
            method="POST"
            class="flex items-center justify-center w-full h-full"
            on:submit={submit}
        >
            <div
                class="flex flex-col items-center justify-center w-full h-full"
            >
                <h2 class="text-xl font-semibold text-white">
                    {login ? "Přihlášení" : "Registrace"}
                </h2>
                <input
                    class="w-3/4 h-8 my-2 text-sm rounded bg-gray-700 text-white focus:outline-none p-2"
                    type="text"
                    minlength={3}
                    maxlength={32}
                    bind:value={name}
                    placeholder="jméno"
                    required
                />
                <input
                    class="w-3/4 h-8 my-2 text-sm rounded bg-gray-700 text-white focus:outline-none p-2"
                    type="password"
                    minlength={6}
                    maxlength={32}
                    bind:value={password}
                    placeholder="heslo"
                    required
                />

                {#if !login}
                    <input
                        class="w-3/4 h-8 my-2 text-sm rounded bg-gray-700 text-white focus:outline-none p-2"
                        type="password"
                        minlength={6}
                        maxlength={32}
                        bind:value={password_again}
                        placeholder="heslo znovu"
                        required
                    />
                {/if}

                <div class="flex items-start w-3/4">
                    <button
                        on:click={() => (login = !login)}
                        type="button"
                        class="text-xs text-blue-400 transition-all hover:text-blue-500"
                        >{login ? "Registrovat" : "Přihlásit se"}</button
                    >
                </div>

                <button
                    class="w-3/4 h-10 my-2 rounded bg-gray-600 hover:bg-gray-500 disabled:hover:bg-gray-600 disabled:opacity-70 transition-all text-white"
                    type="submit"
                    disabled={loading}
                >
                    {#if loading}
                        <div
                            class="flex items-center justify-center space-x-2 animate-bounce"
                        >
                            <div class="w-2 h-2 bg-gray-400 rounded-full" />
                            <div class="w-2 h-2 bg-gray-400 rounded-full" />
                            <div class="w-2 h-2 bg-gray-400 rounded-full" />
                        </div>
                    {:else}
                        {!login ? "Registrovat" : "Přihlásit se"}
                    {/if}
                </button>

                <p class="text-xs text-red-500">{status}</p>
            </div>
        </form>
    </div>
</div>
