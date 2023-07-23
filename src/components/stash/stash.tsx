import type { Item, Profile } from '~/types';
import styles from './stash.module.css';
import { StashCell } from '~/components/stash/cell/stash-cell';

interface StashProps {
    profile: Profile;
}

export const Stash = ({ profile }: StashProps) => {
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
                    return <StashCell key={index} item={item} />;
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
                <div class={styles.grid}>{renderStash()}</div>
            </div>
        </>
    );
};
