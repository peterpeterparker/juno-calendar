<script lang="ts">
	import type { Doc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';

	export let eventDoc: Doc<EventData>;

	let count: number | undefined = undefined;
	$: count = eventDoc?.data.count_answers;
</script>

{#if count !== undefined}
	<span>{count}</span>
{/if}
