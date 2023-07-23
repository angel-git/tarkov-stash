export interface IProfile {
    profile?: Profile;
}

export interface Profile {
    name: string;
    sizeX: number;
    sizeY: number;
    items: Array<Item>
}

export interface Item {
    id: string;
    tpl: string;
    x: number;
    y: number;
    amount: number;
    isStockable: boolean;
}
