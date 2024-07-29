<script lang="ts">
	import { eventKey } from '$lib/derived/page.derived';
	import { type Doc, getDoc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import EventCell from '$lib/components/EventCell.svelte';

	let eventDoc: Doc<EventData> | undefined;

	const loadEvent = async () => {
		if ($eventKey === undefined) {
			eventDoc = undefined;
			return;
		}

		eventDoc = await getDoc<EventData>({
			collection: 'events',
			key: $eventKey
		});
	};

	$: $eventKey, (async () => await loadEvent())();
</script>

{#if eventDoc !== undefined}
	<EventCell {eventDoc} />
{/if}
