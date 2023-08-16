export interface IProfile {
	profile?: Profile;
}

export interface Profile {
	name: string;
	sizeX: number;
	sizeY: number;
	items: Array<Item>;
	bsgItems: { [key: string]: BsgItem };
}

export interface Item {
	id: string;
	tpl: string;
	x: number;
	y: number;
	amount: number;
	isStockable: boolean;
}

export interface BsgItem {
	id: string;
	name: string;
}

export type Option = 'amount';
