<script lang="ts">
	import type { EventData } from '$lib/types/events';
	import { type Doc, listDocs } from '@junobuild/core-peer';
	import { userNotSignedIn, userSignedIn } from '$lib/derived/user.derived';
	import EventCell from '$lib/components/EventCell.svelte';
	import { userStore } from '$lib/stores/user.store';

	let items: Doc<EventData>[] = [];

	const list = async () => {
		if ($userNotSignedIn) {
			items = [];
			return;
		}

		if ($userStore?.key === undefined) {
			items = [];
			return;
		}

		const { items: data } = await listDocs<EventData>({
			collection: 'events',
			filter: {
				owner: $userStore.key
			}
		});

		items = data;
	};

	$: $userSignedIn, (async () => await list())();
</script>

<svelte:window on:exampleReload={list} />

<div class="flex justify-between items-center mb-8">
	<h1 class="text-2xl font-semibold">Your Events</h1>
	<a class="btn btn-accent" href="/create-event">Create New Event</a>
</div>

<!-- Events Table -->
<div class="overflow-x-auto space-y-6 bg-base-100 p-6 rounded-lg shadow-md">
	<table class="table w-full min-w-[600px]">
		<thead>
			<tr>
				<th class="p-2 text-left">Title</th>
				<th class="p-2 text-left">Dates</th>
				<th class="p-2 text-left">Link</th>
				<th class="p-2 text-left">Actions</th>
			</tr>
		</thead>
		<tbody>
			{#each items as item}
				<EventCell eventDoc={item} />
			{/each}
		</tbody>
	</table>
</div>
