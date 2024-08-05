<script lang="ts">
	import { eventKey } from '$lib/derived/page.derived';
	import { type Doc, getDoc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { appState } from '$lib/stores/app.store';

	let eventDoc: Doc<EventData> | undefined;

	type PotentialDate = { date: Date; checked: boolean };
	let potentialDates: PotentialDate[] | undefined;

	const loadEvent = async () => {
		if ($appState === undefined) {
			eventDoc = undefined;
			return;
		}

		if ($eventKey === undefined) {
			eventDoc = undefined;
			return;
		}

		eventDoc = await getDoc<EventData>({
			collection: 'events',
			key: $eventKey
		});

		potentialDates = eventDoc?.data.dates.map((date) => ({ date: new Date(date), checked: false }));
	};

	const onToggle = ({ date: selectedDate, checked }: PotentialDate) => {
		potentialDates = [
			...(potentialDates ?? []).filter(({ date }) => date.getTime() !== selectedDate.getTime()),
			{
				date: selectedDate,
				checked: !checked
			}
		];
	};

	// TODO:
	// - a collection or entity where to save the user choice
	// - submit to save those choices
	//     - do we want to make this public?
	//     - do we want user to create a provide (sign-in with authentication)
	//     - provide a username or surname for the entry
	// - display the potential attendees per dates for the one that created the event

	$: $eventKey, $appState, (async () => await loadEvent())();
</script>

{#if eventDoc !== undefined}
	<div class="w-full max-w-2xl mt-8 dark:text-white" role="table">
		{#each potentialDates ?? [] as date, i}
			<div class="py-2" role="rowgroup">
				<div
					class="flex items-center gap-2 px-3 border-black dark:border-lavender-blue-500 border-[3px] rounded bg-white dark:bg-black dark:text-white transition-all shadow-[8px_8px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_#7888FF]"
					role="row"
				>
					<span role="cell" aria-rowindex={0} class="p-1 flex align-center min-w-max">
						{i + 1}
					</span>
					<div role="cell" class="line-clamp-3 overflow-hidden grow">
						<output>{date.date.toISOString()}</output>
					</div>
					<input type="checkbox" on:change={() => onToggle(date)} />
				</div>
			</div>
		{/each}
	</div>
{:else}
	<p class="dark:text-white">No corresponding event. 🤨</p>
{/if}
