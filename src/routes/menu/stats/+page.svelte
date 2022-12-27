<script lang="ts">
    import { goto } from "$app/navigation";
    import { stats_store } from "$lib/stores";
    import { formatter } from "$lib/util";
    import Decimal from "decimal.js";

    let stats: { name: string; value: string }[] = [];

    const loc_name = (name: string) => {
        const names = {
            playtime: "Celková doba hraní",
            money: "Celkem získáno peněz",
            spent_money: "Celkem utraceno peněz",
            sniffed: "Počet snifnutí",
            out_of_breath: "Počet zadušení",
        } as { [key: string]: string };

        return names[name] || name;
    };

    const format_time = (time: number) => {
        if (time < 60) {
            switch (time) {
                case 1:
                    return "1 minuta";
                case 2:
                case 3:
                case 4:
                    return `${time} minuty`;
                default:
                    return `${time} minut`;
            }
        }

        const hours = new Decimal(time)
            .dividedBy(60)
            .mul(10)
            .floor()
            .dividedBy(10)
            .toNumber();

        if (hours === 1) return "1 hodina";

        return `${hours} hodin`;
    };

    const format_value = (name: string, value: number) => {
        switch (name) {
            case "playtime":
                return format_time(value);
            case "money":
            case "spent_money":
                return formatter.format(value);
            default:
                return value.toString();
        }
    };

    $: {
        stats = Object.entries($stats_store).map(([name, value]) => ({
            name: loc_name(name),
            value: format_value(name, value),
        }));

        stats = stats.sort((a, b) => a.name.length - b.name.length);
    }
</script>

<div class="w-full h-full flex items-center justify-center z-10">
    <div
        class="bg-zinc-800 bg-opacity-70 w-full h-[80%] rounded-lg shadow-xl m-32 p-4"
    >
        <div class="flex items-baseline justify-between">
            <h1 class="text-amber-500 font-bold text-xl">Statistiky</h1>

            <button
                on:click={() => goto("/menu/svinov")}
                class="text-gray-100 font-bold text-sm transition-all hover:text-red-500 hover:underline"
            >
                zpět
            </button>
        </div>

        <div
            class="w-full h-[calc(100%-2rem)] overflow-hidden overflow-y-auto scrollbar"
        >
            {#each stats as stat}
                <div
                    class="w-full flex items-center justify-between font-semibold text-sm"
                >
                    <h2 class="text-gray-100">{stat.name}</h2>
                    <span class="text-gray-400">{stat.value}</span>
                </div>
            {/each}
        </div>
    </div>
</div>
