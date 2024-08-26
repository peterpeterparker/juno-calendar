<script lang="ts">
	import { countDocs, type Doc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { onMount } from 'svelte';

	export let eventDoc: Doc<EventData>;

	let count: bigint | undefined = undefined;

	onMount(async () => {
		// TODO: catch errors
		count = await countDocs({
			collection: 'answers',
			filter: {
				matcher: {
					description: eventDoc.key
				}
			}
		});
	});
</script>

{#if count !== undefined}
	<span>{count}</span>
{/if}
