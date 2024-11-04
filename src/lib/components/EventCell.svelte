<script lang="ts">
	import { deleteDoc, type Doc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { formatDate } from '$lib/utils/date.utils';
	import AnswersCount from '$lib/components/AnswersCount.svelte';
	import { emit } from '$lib/utils/events.utils';
	import { alertStore } from '$lib/stores/alert.store';

	interface Props {
		eventDoc: Doc<EventData>;
	}

	let { eventDoc }: Props = $props();

	let shareUrl: string = $derived(`http://localhost:5173/event/?key=${eventDoc.key}`);

	const onDelete = async () => {
		try {
			await deleteDoc({
				collection: 'events',
				doc: eventDoc
			});

			emit({ message: 'exampleReload' });

			alertStore.set({
				type: 'success',
				message: 'Document deleted!'
			});
		} catch (err: unknown) {
			alertStore.set({
				type: 'error',
				message: 'Cannot document the document!'
			});
		}
	};
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
		<button class="btn btn-sm btn-outline btn-error" onclick={onDelete}>Delete</button>
	</td>
</tr>
