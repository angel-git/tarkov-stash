import { writable } from 'svelte/store';
import type { Profile, BsgItem } from '../types';

interface AddNewItem {
	input: string;
	item?: BsgItem;
}

export const profile = writable<Profile | undefined>();
export const loading = writable<boolean>(false);
export const addNewItem = writable<AddNewItem>({ input: '' });
