<script lang="ts">
	import { eventKey } from '$lib/derived/page.derived';
	import { type Doc, getDoc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { appState } from '$lib/stores/app.store';
	import Title from '$lib/components/Title.svelte';
	import EventForm from '$lib/components/EventForm.svelte';

	let eventDoc: Doc<EventData> | undefined = $state();

	const loadEvent = async () => {
		if ($appState === undefined) {
			eventDoc = undefined;
			return;
		}

		if ($eventKey === undefined) {
			eventDoc = undefined;
			return;
		}

		eventDoc = await getDoc<EventData>({
			collection: 'events',
			key: $eventKey
		});
	};

	$effect(() => {
		// TODO: what's the proper Svelte v5 pattern for this?
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		$eventKey;
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		$appState;
		loadEvent();
	});
</script>

{#if eventDoc !== undefined}
	<Title>{eventDoc.data.title ?? 'Unknown'}</Title>

	<EventForm {eventDoc} />
{:else}
	<p>No corresponding event. 🤨</p>
{/if}
