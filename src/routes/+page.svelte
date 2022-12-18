<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let koruny = 0;
    let sniffing = false;
    let breath = 100;
    let add_remove = 0;
    let unavailable = "";
    let cant_breathe = false;
    let auto_sniff = 1;
    let boost_dech = 1;
    let boost_prachy = 1;
    let max = false;

    let anim: NodeJS.Timeout;

    (async () => {
        const data: string = await invoke("get_save");

        const save = data.split(" ");

        if (save.length !== 4) return;

        koruny = Number(save[0]);
        auto_sniff = Number(save[1]);
        boost_dech = Number(save[2]);
        boost_prachy = Number(save[3]);
    })();

    const add = (amount: number) => {
        add_remove = amount;

        clearTimeout(anim);

        anim = setTimeout(() => {
            add_remove = 0;
        }, 1100);
    };

    const set_unavailable = (upgrade: string) => {
        unavailable = upgrade;

        setTimeout(() => {
            unavailable = "";
        }, 250);
    };

    const dech = () => {
        if (breath < 100) Math.max((breath += 1 * boost_dech), 100);

        if (!cant_breathe)
            document
                .querySelector("#dech_bar")
                ?.setAttribute("style", `width: ${breath}%`);

        setTimeout(() => {
            dech();
        }, 250);
    };

    dech();

    const sniff = async () => {
        if (cant_breathe) return;

        koruny += 1 * boost_prachy;
        add(1 * boost_prachy);

        await invoke("save", {
            koruny,
            autoSniff: auto_sniff,
            boostPrachy: boost_prachy,
            boostDech: boost_dech,
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
            }, 15000);
        }
    };

    const auto_sniff_int = () => {
        if (auto_sniff > 1) sniff();

        setTimeout(() => {
            auto_sniff_int();
        }, 5000 - 100 * auto_sniff);
    };

    auto_sniff_int();

    const buy = async (item: string) => {
        if (item === "auto_sniff") {
            if (koruny >= 250 * auto_sniff) {
                if (5000 - 100 * auto_sniff <= 0) {
                    max = true;
                    set_unavailable("auto_sniff");
                    return;
                }

                add(-250 * auto_sniff);

                koruny -= 250 * auto_sniff;
                auto_sniff += 1;

                if (5000 - 100 * auto_sniff <= 0) max = true;

                await invoke("save", {
                    koruny,
                    autoSniff: auto_sniff,
                    boostPrachy: boost_prachy,
                    boostDech: boost_dech,
                });
            } else set_unavailable("auto_sniff");
        }

        if (item === "boost_dech") {
            if (koruny >= 200 * boost_dech) {
                add(-200 * boost_dech);

                koruny -= 200 * boost_dech;
                boost_dech += 1;

                await invoke("save", {
                    koruny,
                    autoSniff: auto_sniff,
                    boostPrachy: boost_prachy,
                    boostDech: boost_dech,
                });
            } else set_unavailable("boost_dech");
        }

        if (item === "boost_prachy") {
            if (koruny >= 500 * boost_prachy) {
                add(-500 * boost_prachy);

                koruny -= 500 * boost_prachy;
                boost_prachy += 1;

                await invoke("save", {
                    koruny,
                    autoSniff: auto_sniff,
                    boostPrachy: boost_prachy,
                    boostDech: boost_dech,
                });
            } else set_unavailable("boost_prachy");
        }
    };
</script>

<div
    class="bg-slate-700 h-full w-96 bg-opacity-25 p-4 flex flex-col justify-between"
>
    <ul class="text-gray-100 font-thin text-sm">
        <li class="text-yellow-400 w-full flex justify-between">
            {koruny} CZK
            <span
                class={`${add_remove === 0 && "hidden"} ${
                    add_remove > 0
                        ? "text-green-500 add"
                        : "text-red-400 remove"
                }`}>{add_remove > 0 ? "+" : ""} {add_remove} CZK</span
            >
        </li>
        <li><br /></li>
        <li
            on:keypress={() => buy("auto_sniff")}
            on:click={() => buy("auto_sniff")}
            class="text-yellow-500"
        >
            ({max || 5000 - 100 * auto_sniff <= 0 ? "max" : auto_sniff})
            <span
                class={`cursor-pointer transition-all ${
                    unavailable === "auto_sniff"
                        ? "text-red-500"
                        : "hover:text-amber-400 text-gray-100"
                }`}>auto sniff ({250 * auto_sniff}CZK)</span
            >
        </li>
        <li
            on:keypress={() => buy("boost_dech")}
            on:click={() => buy("boost_dech")}
            class="text-yellow-500"
        >
            ({boost_dech})
            <span
                class={`cursor-pointer transition-all ${
                    unavailable === "boost_dech"
                        ? "text-red-500"
                        : "hover:text-blue-400 text-gray-100"
                }`}>boost dech ({200 * boost_dech}CZK)</span
            >
        </li>
        <li
            on:keypress={() => buy("boost_prachy")}
            on:click={() => buy("boost_prachy")}
            class="text-yellow-500"
        >
            ({boost_prachy})
            <span
                class={`cursor-pointer transition-all ${
                    unavailable === "boost_prachy"
                        ? "text-red-500"
                        : "hover:text-green-400 text-gray-100"
                }`}>boost prachy ({500 * boost_prachy}CZK)</span
            >
        </li>
    </ul>

    <div>
        <label class="text-gray-100" for="dech"
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
