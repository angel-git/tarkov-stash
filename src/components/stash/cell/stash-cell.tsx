import type { Item } from '~/types';
import styles from './stash-cell.module.css';

interface StashCellProps {
    item?: Item;
}

export const StashCell = ({ item }: StashCellProps) => {
    const renderCell = () => {
        if (item) {
            return <div class={styles[`item-${item.tpl}`]}><div class={styles['amount']}>{item.isStockable ? item.amount : ""}</div></div>;
        } else {
            return <div class={styles['empty']}></div>;
        }
    };

    return <div class={styles['grid-item']}>{renderCell()}</div>;
};
