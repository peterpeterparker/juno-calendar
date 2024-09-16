<script lang="ts">
	import { deleteDoc, deleteManyDocs, type Doc, listDocs } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { formatDate } from '$lib/utils/date.utils';
	import AnswersCount from '$lib/components/AnswersCount.svelte';
	import { emit } from '$lib/utils/events.utils';
	import { alertStore } from '$lib/stores/alert.store';

	export let eventDoc: Doc<EventData>;

	let shareUrl: string;
	$: shareUrl = `http://localhost:5173/event/?key=${eventDoc.key}`;

	const onDelete = async () => {
		try {
			// const answers = await listDocs({
			// 	collection: 'answers',
			// 	filter: {
			// 		matcher: {
			// 			description: eventDoc.key
			// 		}
			// 	}
			// });

			// await deleteManyDocs({
			// 	docs: [
			// 		// ...answers.items.map((doc) => ({
			// 		// 	collection: 'answers',
			// 		// 	doc
			// 		// })),
			// 		{
			// 			collection: 'events',
			// 			doc: eventDoc
			// 		}
			// 	]
			// });

			await deleteDoc({
				collection: 'events',
				doc: eventDoc
			})

			emit({ message: 'exampleReload' });

			alertStore.set({
				type: 'success',
				message: 'Document deleted!'
			});
		} catch (err: unknown) {
			console.log("ERRORORO", err);
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
		<button class="btn btn-sm btn-outline btn-error" on:click={onDelete}>Delete</button>
	</td>
</tr>
