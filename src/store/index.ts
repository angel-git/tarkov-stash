import { writable } from 'svelte/store';
import type { Profile, BsgItem, PresetItem } from '../types';

interface AddNewItem {
  input: string;
  item?: BsgItem;
}

interface AddNewPreset {
  input: string;
  item?: PresetItem;
}

export const profile = writable<Profile | undefined>();
export const loading = writable<boolean>(false);
export const addItemFeatureEnabled = writable<boolean>(true);
export const addNewItem = writable<AddNewItem>({ input: '' });
export const addNewPreset = writable<AddNewPreset>({ input: '' });
