import { writable } from 'svelte/store';
import type { Profile } from '../types';

export const profile = writable<Profile | undefined>();
