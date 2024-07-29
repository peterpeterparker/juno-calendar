<script lang="ts">
	import type { EventData } from '$lib/types/events';
	import { type Doc, listDocs } from '@junobuild/core-peer';
	import { userNotSignedIn, userSignedIn } from '$lib/derived/user.derived';

	let items: Doc<EventData>[] = [];

	const list = async () => {
		if ($userNotSignedIn) {
			items = [];
			return;
		}

		const { items: data } = await listDocs<EventData>({
			collection: 'events',
			filter: {}
		});

		items = data;
	};

	$: $userSignedIn, (async () => await list())();
</script>

<svelte:window on:exampleReload={list} />

<div class="w-full max-w-2xl mt-8 dark:text-white" role="table">
	<div role="row">
		<span role="columnheader" aria-sort="none"> Events </span>
	</div>

	<div class="py-2" role="rowgroup">
		{#each items as item, index}
			<div
				class="flex items-center gap-2 px-3 mb-4 border-black dark:border-lavender-blue-500 border-[3px] rounded bg-white dark:bg-black dark:text-white transition-all shadow-[8px_8px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_#7888FF]"
				role="row"
			>
				<span role="cell" aria-rowindex={index} class="p-1 flex align-center min-w-max">
					{index + 1}
				</span>
				<div role="cell" class="line-clamp-3 overflow-hidden grow">
					{#each item.data.dates as myDate}
						{new Date(myDate).toISOString()}
					{/each}
				</div>
			</div>
		{/each}
	</div>
</div>
