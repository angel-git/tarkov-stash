export interface Session {
  id: string;
  username: string;
}

export interface Profile {
  name: string;
  sizeX: number;
  sizeY: number;
  items: Array<Item>;
  bsgItems: Record<string, BsgItem>;
  sptVersion: string;
  locale: Record<string, string>;
  presetItems: Array<PresetItem>;
}

export interface Item {
  id: string;
  tpl: string;
  parentId: string;
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
  slotItems?: Array<SlotItem>;
  presetImageId?: string;
  cacheImage?: string;
}

export interface SlotItem {
  id: string;
  tpl: string;
  slotId: string;
  parentId: string;
  upd: any;
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
  parentId: string;
  height: number;
  width: number;
  hideEntrails: boolean;
  unlootable: boolean;
  unbuyable: boolean;
  type: BsgItemType;
  backgroundColor: string;
  Slots: Array<Slot>;
  ergonomics: number;
  deviationMax: number;
  deviationCurve: number;
  sightingRange: number;
  recoil: number;
  recoilForceBack: number;
  recoilForceUp: number;
  velocity: number;
  initialSpeed: number;
  centerOfImpact: number;
  ammoAccr: number;
  accuracy: number;
  stackMaxSize: number;
}

export interface Slot {
  _id: string;
  _name: SlotKind;
  _parent: string;
  _props: SlotProps;
}

export interface SlotProps {
  filters: Array<SlotPropsFilter>;
}

export interface SlotPropsFilter {
  Filter: Array<string>;
}

export type SlotKind =
  | 'mod_barrel'
  | 'mod_bipod'
  | 'mod_catch'
  | 'mod_charge'
  | 'mod_equipment'
  | 'mod_equipment_000'
  | 'mod_equipment_001'
  | 'mod_equipment_002'
  | 'mod_flashlight'
  | 'mod_foregrip'
  | 'mod_gas_block'
  | 'mod_hammer'
  | 'mod_handguard'
  | 'mod_launcher'
  | 'mod_magazine'
  | 'mod_mount'
  | 'mod_mount_000'
  | 'mod_mount_001'
  | 'mod_mount_002'
  | 'mod_mount_003'
  | 'mod_mount_004'
  | 'mod_mount_005'
  | 'mod_mount_006'
  | 'mod_muzzle'
  | 'mod_muzzle_000'
  | 'mod_muzzle_001'
  | 'mod_nvg'
  | 'mod_pistol_grip'
  | 'mod_pistol_grip_akms'
  | 'mod_pistolgrip'
  | 'mod_pistolgrip_000'
  | 'mod_reciever'
  | 'mod_scope'
  | 'mod_scope_000'
  | 'mod_scope_001'
  | 'mod_scope_002'
  | 'mod_scope_003'
  | 'mod_sight_front'
  | 'mod_sight_rear'
  | 'mod_stock'
  | 'mod_stock_000'
  | 'mod_stock_001'
  | 'mod_stock_002'
  | 'mod_stock_akms'
  | 'mod_stock_axis'
  | 'mod_tactical'
  | 'mod_tactical001'
  | 'mod_tactical002'
  | 'mod_tactical_000'
  | 'mod_tactical_001'
  | 'mod_tactical_002'
  | 'mod_tactical_003'
  | 'mod_tactical_004'
  | 'mod_tactical_2'
  | 'mod_trigger'
  | 'patron_in_weapon';

export type Option = 'amount' | 'fir' | 'resource' | 'open' | 'delete' | 'details';

export interface NewItem {
  id: string;
  locationX: number;
  locationY: number;
}

export interface Stats {
  ergonomics: number;
  accuracy: number;
  sightingRange: number;
  verticalRecoil: number;
  verticalRecoilPercentage: number;
  horizontalRecoil: number;
  horizontalRecoilPercentage: number;
  velocity: number;
}

export interface PresetItem {
  id: string;
  encyclopedia: string;
  items: Array<PresetItemItem>;
  width: number;
  height: number;
}

export interface PresetItemItem {
  id: string;
  tpl: string;
  parentId: string;
  slotId: SlotKind;
  upd: unknown;
}
