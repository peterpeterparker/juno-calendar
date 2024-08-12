<script lang="ts">
	import type { Doc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { formatDate } from '$lib/utils/date.utils';
	import AnswersCount from '$lib/components/AnswersCount.svelte';

	export let eventDoc: Doc<EventData>;

	let shareUrl: string;
	$: shareUrl = `http://localhost:5173/event/?key=${eventDoc.key}`;
</script>

<tr>
	<td class="p-2 text-left w-1/4">{eventDoc.data.title ?? ''}</td>
	<td class="p-2 text-left w-1/4">
		<span class="min-w-full truncate block max-w-[150px] whitespace-nowrap">
			{#each eventDoc.data.dates as date}
				<output class="pr-2">{formatDate(new Date(date))}</output>
			{/each}
		</span>
	</td>
	<td class="p-2 text-left w-1/4"><AnswersCount {eventDoc} /></td>
	<td class="p-2 text-left w-1/6"><a href={shareUrl} class="text-primary">View Link</a></td>
	<td class="p-2 text-left w-1/6">
		<button class="btn btn-sm btn-outline btn-error">Delete</button>
	</td>
</tr>
