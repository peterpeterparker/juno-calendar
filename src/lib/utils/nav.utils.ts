import { browser } from '$app/environment';
import type { LoadEvent } from '@sveltejs/kit';

export type RouteKey = { key: string | null | undefined };

export const loadEventKey = ($event: LoadEvent): RouteKey => {
	if (!browser) {
		return {
			key: undefined
		};
	}

	const {
		url: { searchParams }
	} = $event;

	return {
		key: searchParams?.get('key')
	};
};
