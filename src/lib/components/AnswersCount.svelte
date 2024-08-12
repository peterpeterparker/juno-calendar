<script lang="ts">
	import { type Doc, listDocs } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { onMount } from 'svelte';
	import type { AnswersData } from '$lib/types/answers';

	export let eventDoc: Doc<EventData>;

	let count: bigint | undefined = undefined;

	onMount(async () => {
		// TODO: catch errors
		const { items_length } = await listDocs<AnswersData>({
			collection: 'answers',
			filter: {
				matcher: {
					description: eventDoc.key
				}
			}
		});

		count = items_length;
	});
</script>

{#if count !== undefined}
	<span>{count}</span>
{/if}
