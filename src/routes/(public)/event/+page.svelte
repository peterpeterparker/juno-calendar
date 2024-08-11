<script lang="ts">
	import { eventKey } from '$lib/derived/page.derived';
	import { type Doc, getDoc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { appState } from '$lib/stores/app.store';
	import { formatDate } from '$lib/utils/date.utils';

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

	const handleSubmit = async ($event: SubmitEvent) => {
		$event.preventDefault();

		console.log(potentialDates);
	};
</script>

{#if eventDoc !== undefined}
	<h1 class="text-2xl font-semibold mb-4">{eventDoc.data.title ?? 'Unknown'}</h1>
	<p class="mb-4">Select the dates you are available:</p>

	<!-- Availability Form -->
	<form class="space-y-4" on:submit={async ($event) => await handleSubmit($event)}>
		<div class="card shadow-md bg-base-100">
			<div class="card-body">
				{#each potentialDates ?? [] as date, i}
					<div class="form-control">
						<label class="label cursor-pointer">
							<span class="label-text">{formatDate(date.date)}</span>
							<input type="checkbox" class="checkbox" on:change={() => onToggle(date)} />
						</label>
					</div>
				{/each}
			</div>
		</div>

		<button type="submit" class="btn btn-accent w-full">Submit Availability</button>
	</form>
{:else}
	<p>No corresponding event. 🤨</p>
{/if}
