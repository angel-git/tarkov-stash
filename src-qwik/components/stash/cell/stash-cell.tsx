import type { PropFunction } from '@builder.io/qwik';
import { $ } from '@builder.io/qwik';
import type { Item } from '~/types';
import styles from './stash-cell.module.css';
import { SUPPORTED_ITEMS } from '~/helper/supported-items';
import type { StashStore } from '~/components/stash/stash';

export type ItemOptions = 'change_amount';

interface StashCellProps {
    item?: Item;
    onClick$: PropFunction<(item: Item) => void>;
    onOptionClick$: PropFunction<(option: ItemOptions) => void>;
    store: StashStore;
}

export const StashCell = ({ item, store, onClick$, onOptionClick$ }: StashCellProps) => {
    const handleOnClick = $((clickedItem: Item) => {
        if (SUPPORTED_ITEMS.includes(clickedItem.tpl)) {
            onClick$(clickedItem);
        }
    });

    const handleOptionOnClick = $((option: ItemOptions) => {
        onOptionClick$(option);
    });

    const renderOpen = () => {
        const isOpen = store.item && item ? store.item.id === item.id : false;
        if (!isOpen) return;
        return (
            <div class={styles.options}>
                <div class={styles.option} onClick$={() => handleOptionOnClick('change_amount')}>
                    Change amount
                </div>
            </div>
        );
    };

    const renderCell = () => {
        if (item) {
            return (
                <div class={[styles[`item-${item.tpl}`], styles['item-clickable']]} onClick$={() => handleOnClick(item)}>
                    <div class={styles['amount']}>{item.isStockable ? item.amount : ''}</div>
                </div>
            );
        } else {
            return <div class={styles['empty']}></div>;
        }
    };

    return (
        <div class={styles['grid-item']}>
            {renderCell()}
            {renderOpen()}
        </div>
    );
};
