import { createAuth0Client } from "@auth0/auth0-spa-js";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import type { Writable } from "svelte/store";
import type { LoginStatus } from "$bindings/LoginStatus";
import { logged_in } from "./stores";

export const formatter = new Intl.NumberFormat("cs-CZ", {
    notation: "compact",
    style: "currency",
    currency: "CZK",
});

export const with_auth = (): {
    login: () => Promise<void>;
    logout: () => Promise<void>;
    logged_in: Writable<LoginStatus>;
} => {
    const get_client = async () => {
        const domain = await invoke<string>("get_auth0_domain");
        const clientId = await invoke<string>("get_auth0_client_id");
        const audience = await invoke<string>("get_auth0_audience");
        const url = await invoke<string>("get_auth0_redir_url");

        return createAuth0Client({
            domain,
            clientId,
            authorizationParams: {
                redirect_uri: `${url}/menu/svinov`,
                audience,
                scope: "openid profile",
            },
        });
    }

    const get_user = async () => {
        try {
            const client = await get_client();

            const token = await client.getTokenSilently();

            await invoke("set_token", { token });

            logged_in.subscribe(async (logged) => {
                if (!logged.logged) {
                    await invoke("logged");

                    return
                };

                await invoke("login");
            })();
        } catch (err) {
            console.error(err);
        }
    }

    const login = async () => {
        const client = await get_client();

        await invoke("logging");

        client.loginWithRedirect();
    }

    const logout = async () => {
        const client = await get_client();

        client.logout();

        await invoke("logout");
    }

    get_user();

    return { login, logout, logged_in }
}
