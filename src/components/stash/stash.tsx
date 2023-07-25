import { $, component$, useStore } from '@builder.io/qwik';
import type { Item, Profile } from '~/types';
import styles from './stash.module.css';
import type { ItemOptions } from '~/components/stash/cell/stash-cell';
import { StashCell } from '~/components/stash/cell/stash-cell';
import { Dialog } from '~/components/stash/dialog/dialog';

interface StashProps {
    profile: Profile;
}

export interface StashStore {
    item?: Item;
    option?: ItemOptions;
}

export const Stash = component$<StashProps>(({ profile }: StashProps) => {
    const store = useStore<StashStore>({ item: undefined, option: undefined });

    const handleOnClick = $((item: Item) => {
        store.option = undefined;
        if (item.id === store.item?.id) {
            store.item = undefined;
        } else {
            store.item = item;
        }
    });

    const handleOnOptionClick = $((option: ItemOptions) => {
        store.option = option;
    });

    const renderStash = () => {
        const orderedItems: Array<Item | undefined> = [];

        for (let col = 0; col < profile.sizeY; col++) {
            for (let row = 0; row < profile.sizeX; row++) {
                const maybeItem = profile.items.find((item) => item.x === row && item.y === col);
                orderedItems.push(maybeItem);
            }
        }
        return (
            <>
                {orderedItems.map((item, index) => {
                    return <StashCell store={store} key={item?.id ?? index} item={item} onClick$={handleOnClick} onOptionClick$={handleOnOptionClick} />;
                })}
            </>
        );
    };

    return (
        <>
            <div class="container container-center">
                <h3>
                    Editing <span class="highlight">{profile.name}</span>'s stash
                </h3>
                <h4>
                    Your current stash size is {profile.sizeX}x{profile.sizeY}
                </h4>
                {store.item && store.option && <Dialog item={store.item} option={store.option} />}
                <div class={styles.grid}>{renderStash()}</div>
            </div>
        </>
    );
});
