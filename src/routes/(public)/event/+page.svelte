<script lang="ts">
	import { eventKey } from '$lib/derived/page.derived';
	import { type Doc, getDoc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import EventCell from '$lib/components/EventCell.svelte';
	import { appState } from '$lib/stores/app.store';

	let eventDoc: Doc<EventData> | undefined;

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

	$: $eventKey, $appState, (async () => await loadEvent())();
</script>

<div class="w-full max-w-2xl mt-8 dark:text-white" role="table">
	<div class="py-2" role="rowgroup">
		<div
			class="flex items-center gap-2 px-3 mb-4 border-black dark:border-lavender-blue-500 border-[3px] rounded bg-white dark:bg-black dark:text-white transition-all shadow-[8px_8px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_#7888FF]"
			role="row"
		>
			<span role="cell" aria-rowindex={0} class="p-1 flex align-center min-w-max">
				{0 + 1}
			</span>
			<div role="cell" class="line-clamp-3 overflow-hidden grow">
				{#if eventDoc !== undefined}
					<EventCell {eventDoc} />
				{/if}
			</div>
		</div>
	</div>
</div>
