import { page } from '$app/stores';
import { derived, type Readable } from 'svelte/store';

export const eventKey: Readable<string | undefined> = derived([page], ([page]) => {
	const { data } = page;

	if (data.key === undefined || data.key === null) {
		return undefined;
	}

	return data.key;
});
