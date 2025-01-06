<script lang="ts">
	import type { EventData } from '$lib/types/events';
	import { type Doc, listDocs } from '@junobuild/core';
	import { userSignedIn } from '$lib/derived/user.derived';
	import EventCell from '$lib/components/EventCell.svelte';
	import { userStore } from '$lib/stores/user.store';
	import Button from '$lib/components/ui/Button.svelte';
	import { last } from '$lib/utils/array.utils';
	import Title from '$lib/components/Title.svelte';

	let items: Doc<EventData>[] = $state([]);

	let previousEnabled = $state(true);
	let nextEnabled = $state(true);

	let startAfter: string[] = [];

	const list = async (signedIn: boolean) => {
		if (!signedIn) {
			items = [];
			return;
		}

		if ($userStore?.key === undefined) {
			items = [];
			return;
		}

		const limit = 10;

		const {
			items: data,
			items_page: itemsPage,
			items_length: itemsLength
		} = await listDocs<EventData>({
			collection: 'events',
			filter: {
				owner: $userStore.key,
				order: {
					desc: false,
					field: 'keys'
				},
				paginate: {
					startAfter: last(startAfter),
					limit
				}
			}
		});

		previousEnabled = (itemsPage ?? 0n) > 0n;
		nextEnabled = itemsLength === BigInt(limit);

		items = data;
	};

	const previous = async () => {
		startAfter.pop();

		await list($userSignedIn);
	};

	const next = async () => {
		const lastKey = last(items ?? [])?.key;

		startAfter = [...startAfter, ...(lastKey !== undefined ? [lastKey] : [])];

		await list($userSignedIn);
	};

	$effect(() => {
		list($userSignedIn);
	});
</script>

<svelte:window onexampleReload={async () => await list($userSignedIn)} />

<div class="flex justify-between items-center mb-8">
	<Title>Your Events</Title>
	<a class="btn btn-accent" href="/create-event" data-tid="btn-create-event">Create New Event</a>
</div>

<!-- Events Table -->
<div class="overflow-x-auto space-y-6 bg-base-100 p-6 border-2 border-black">
	<table class="table w-full min-w-[600px]">
		<thead>
			<tr>
				<th class="p-2 text-left">Title</th>
				<th class="p-2 text-left">Dates</th>
				<th class="p-2 text-left">Answers</th>
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

<div class="flex gap-2 pt-4">
	{#if previousEnabled}
		<Button onclick={previous}>Previous</Button>
	{/if}

	{#if nextEnabled}
		<Button onclick={next}>Next</Button>
	{/if}
</div>
