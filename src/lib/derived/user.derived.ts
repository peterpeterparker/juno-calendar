import { userStore } from '$lib/stores/user.store';
import { derived, type Readable } from 'svelte/store';

export const userSignedIn: Readable<boolean> = derived(
	userStore,
	(user) => user !== null && user !== undefined
);

export const userNotSignedIn: Readable<boolean> = derived(userSignedIn, (signedIn) => !signedIn);
