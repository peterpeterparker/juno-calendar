<script lang="ts">
	import { eventKey } from '$lib/derived/page.derived';
	import { type Doc, getDoc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { appState } from '$lib/stores/app.store';
	import Title from '$lib/components/Title.svelte';
	import EventForm from '$lib/components/EventForm.svelte';
	import Card from '$lib/components/Card.svelte';
	import EventSuccess from '$lib/components/EventSuccess.svelte';

	let eventDoc: Doc<EventData> | undefined = $state();

	let step: 'loading' | 'form' | 'success' | 'notfound' = $state('loading');

	const loadEvent = async () => {
		if ($appState === undefined) {
			eventDoc = undefined;
			return;
		}

		if ($eventKey === undefined) {
			eventDoc = undefined;
			step = 'notfound';
			return;
		}

		try {
			eventDoc = await getDoc<EventData>({
				collection: 'events',
				key: $eventKey
			});

			step = 'form';
		} catch (_err: unknown) {
			step = 'notfound';
		}
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

	<article class="space-y-4 mt-4">
		{#if step === 'success'}
			<EventSuccess />
		{:else}
			<EventForm {eventDoc} onSuccess={() => step = "success"} />
		{/if}
	</article>
{:else}
	<p>No corresponding event. 🤨</p>
{/if}
