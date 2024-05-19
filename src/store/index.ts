import { writable } from 'svelte/store';
import type { Profile, BsgItem, PresetItem, Item } from '../types';

interface AddNewItem {
  input: string;
  amount: number;
  item?: BsgItem;
}

interface AddNewPreset {
  input: string;
  item?: PresetItem;
}

export const profile = writable<Profile | undefined>();
export const loading = writable<boolean>(false);
export const addNewItem = writable<AddNewItem>({ input: '', amount: 1 });
export const addNewPreset = writable<AddNewPreset>({ input: '' });
export const contextMenu = writable<Item | undefined>();
