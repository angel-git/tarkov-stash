import type { Item } from '~/types';
import EmptyIcon from '~/media/items/empty.png?jsx';
import UnknownIcon from '~/media/items/unknown.png?jsx';
import RoublesIcon from '~/media/items/roubles.png?jsx';

interface StashCellProps {
    item?: Item;
}

export const StashCell = ({ item }: StashCellProps) => {
    if (item) {
        if (item.tpl === '5449016a4bdc2d6f028b456f') {
            return <RoublesIcon />
        }
        return <UnknownIcon />;
    } else {
        return <EmptyIcon />;
    }
};
