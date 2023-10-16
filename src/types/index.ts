export interface Profile {
	name: string;
	sizeX: number;
	sizeY: number;
	items: Array<Item>;
	bsgItems: Record<string, BsgItem>;
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
	isContainer: boolean;
	gridItems: Array<GridItem> | null;
}

export interface GridItem {
	name: string;
	cellsH: number;
	cellsV: number;
	items: Array<Item>;
}

type BsgItemType = 'Node' | 'Item';

export interface BsgItem {
	id: string;
	name: string;
	shortName: string;
	height: number;
	width: number;
	hideEntrails: boolean;
	unlootable: boolean;
	unbuyable: boolean;
	type: BsgItemType;
	backgroundColor: string;
}

export type Option = 'amount' | 'fir' | 'resource' | 'open';

export interface NewItem {
	id: string;
	locationX: number;
	locationY: number;
}
