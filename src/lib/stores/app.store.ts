import { writable } from 'svelte/store';

export const appState = writable<'initialized' | undefined>(undefined);
