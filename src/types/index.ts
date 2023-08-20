export interface Profile {
	name: string;
	sizeX: number;
	sizeY: number;
	items: Array<Item>;
	bsgItems: { [key: string]: BsgItem };
	sptVersion: string;
}

export interface Item {
	id: string;
	tpl: string;
	x: number;
	y: number;
	sizeX: number;
	sizeY: number;
	amount: number;
	stackMaxSize: number;
	isStockable: boolean;
	isFir: boolean;
	rotation: 'Horizontal' | 'Vertical';
	backgroundColor: string;
	resource: number | null;
	maxResource: number | null;
}

export interface BsgItem {
	id: string;
	name: string;
	shortName: string;
}

export type Option = 'amount' | 'fir' | 'resource';
