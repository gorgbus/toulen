import axios, { AxiosError } from "axios";
import { invoke } from "@tauri-apps/api/tauri";

invoke("get_api_url").then((url) => axios.defaults.baseURL = url as string);

// axios.interceptors.request.use(
//     (config) => {
//         const token = localStorage.getItem("access_token");

//         if (token && config.headers) {
//             config.headers["Authorization"] = `Bearer ${token}`;
//         }

//         return config;
//     },
//     (error) => {
//         return Promise.reject(error);
//     }
// );

export type Player = {
    id: number,
    money: number,
    stamina: number,
    regen: number,
    auto: number,
    toulen: number,
    user: {
        name: string
    }
}

export const register = async (name: string, password: string): Promise<{
    status: number | undefined,
    data: { id: string } | undefined
}> => {
    try {
        const res = await axios.post("/users/create", {
            name,
            password
        });

        return { status: res.status, data: res.data };
    } catch (err) {
        console.error(err);

        const err_ax = err as AxiosError;
        const data = err_ax.response?.data as { code: string };

        if (data.code === "P2002") return { status: 409, data: undefined };

        return { status: err_ax.response?.status, data: undefined };
    }
}

export const create_player = async (userId: string): Promise<Player | undefined> => {
    try {
        const res = await axios.post("/players/create", { userId });

        localStorage.setItem("player_id", res.data.id);

        return res.data;
    } catch (err: any) {
        console.error(err);

        return undefined;
    }
}

export const fn_login = async (name: string, password: string): Promise<{
    status: number | undefined,
    data: { access_token: string, id: string } | undefined
}> => {
    try {
        const res = await axios.post("/users/login", { name, password });

        return { data: res.data, status: res.status };
    } catch (err: any) {
        console.error(err);

        let err_ax = err as AxiosError;

        return { data: undefined, status: err_ax.response?.status };
    }
}

export const get_player = async (): Promise<Player | undefined> => {
    try {
        const user_id = localStorage.getItem("user_id");

        const res = await axios.get(`/players/${user_id}`);

        return res.data;
    } catch (err: any) {
        console.error(err);

        return undefined;
    }
}

export const get_players = async (): Promise<Player[] | undefined> => {
    try {
        const res = await axios.get("/players?limit=0");

        return res.data.players;
    } catch (err) {
        console.error(err);

        return undefined;
    }
}

export const update_player = async () => {
    try {
        const userId = localStorage.getItem("user_id");
        const money = localStorage.getItem("money");
        const stamina = localStorage.getItem("stamina");
        const regen = localStorage.getItem("regen");
        const auto = localStorage.getItem("auto");

        const res = await axios.patch(`/players`, { userId, money, stamina, regen, auto });

        return res.data;
    } catch (err) {
        console.error(err);
    }
}