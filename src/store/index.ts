import { writable } from 'svelte/store';
import type { Profile, BsgItem, PresetItem, UserPresetItem, Item } from '../types';

interface AddNewItem {
  input: string;
  amount: number;
  item?: BsgItem;
}

interface AddNewPreset {
  input: string;
  item?: PresetItem;
}

interface AddNewUserPreset {
  input: string;
  item?: UserPresetItem;
}

export const profile = writable<Profile | undefined>();
export const loading = writable<boolean>(false);
export const addNewItem = writable<AddNewItem>({ input: '', amount: 1 });
export const addNewPreset = writable<AddNewPreset>({ input: '' });
export const addNewUserPreset = writable<AddNewUserPreset>({ input: '' });
export const stashGrid = writable<Array<Array<Item | undefined>> | undefined>();
