import { add_money, add_sniffed_stats, stamina_store, subtract_stamina } from "$lib/stores"
import { MONEY_PER_SNIFF, STAMINA_PER_SNIFF } from "$lib/constants";

export const sniff = () => {
    let stamina = 0;

    const unsub = stamina_store.subscribe(value => {
        stamina = value;
    });

    if (stamina > 0) {
        subtract_stamina(STAMINA_PER_SNIFF);

        add_sniffed_stats(1);

        add_money(MONEY_PER_SNIFF);
    }

    unsub();
}