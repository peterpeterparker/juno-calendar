<script lang="ts">
	import type { Doc } from '@junobuild/core';
	import type { EventData } from '$lib/types/events';

	interface Props {
		eventDoc: Doc<EventData>;
	}

	let { eventDoc }: Props = $props();

	let count: number | undefined = $derived(eventDoc?.data.count_answers);
</script>

{#if count !== undefined}
	<span>{count}</span>
{/if}
