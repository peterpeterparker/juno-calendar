<script lang="ts">
	import { userStore } from '$lib/stores/user.store';
	import { nanoid } from 'nanoid';
	import { alertStore } from '$lib/stores/alert.store';
	import { setDoc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';

	let title = '';
	let dates = [''];

	const addDateField = () => {
		dates = [...dates, ''];
	};

	const removeDateField = (index: number) => {
		dates = dates.filter((_, i) => i !== index);
	};

	let progress = false;

	const handleSubmit = async ($event: SubmitEvent) => {
		$event.preventDefault();

		if ($userStore === undefined || $userStore === null) {
			alertStore.set({
				type: 'error',
				message: 'No user. Did you sign-in?'
			});
			return;
		}

		const submittedDates = dates
			.filter((date) => date !== '')
			.map((date) => new Date(date).getTime());

		if (submittedDates.length === 0) {
			alertStore.set({
				type: 'error',
				message: 'You must at least specify one date to create the event.'
			});
		}

		progress = true;

		try {
			const key = nanoid();

			await setDoc<EventData>({
				collection: 'events',
				doc: {
					key,
					data: {
						title,
						dates: submittedDates
					}
				}
			});

			alertStore.set({
				type: 'success',
				message: 'Event successfully created!'
			});

			title = '';
			dates = [''];
		} catch (err) {
			alertStore.set({
				type: 'error',
				message: 'Unexpected error while creating the event.'
			});
			console.error(err);
		}

		progress = false;
	};
</script>

<h1 class="text-2xl font-semibold mb-6 text-left mt-2">Create New Event</h1>

<form
	on:submit={async ($event) => await handleSubmit($event)}
	class="space-y-6 bg-base-100 p-6 border-2 border-black mt-10"
>
	<div class="form-control">
		<label class="label" for="event-title">
			<span class="label-text font-medium">Event Title</span>
		</label>
		<input
			type="text"
			id="event-title"
			class={`input input-bordered w-full ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
			bind:value={title}
			placeholder="Enter event title"
			required
			disabled={progress}
		/>
	</div>

	<div class="form-control">
		<label class="label" for="event-dates">
			<span class="label-text font-medium">Select Dates</span>
		</label>
		<div id="event-dates">
			{#each dates as date, index}
				<div class="flex items-center space-x-2 mb-2">
					<input
						type="date"
						class={`input input-bordered w-full ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
						bind:value={dates[index]}
						placeholder="Choose date"
						required
						disabled={progress}
					/>
					{#if dates.length > 1}
						<button
							type="button"
							class={`btn btn-outline btn-error btn-sm ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
							on:click={() => removeDateField(index)}
							disabled={progress}
						>
							Remove
						</button>
					{/if}
				</div>
			{/each}
		</div>
		<button
			type="button"
			class={`btn btn-secondary btn-sm mt-2 ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
			on:click={addDateField}
			disabled={progress}
		>
			Add Date
		</button>
	</div>

	<button
		type="submit"
		class={`btn btn-primary btn-block ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
		disabled={progress}
	>
		{progress ? 'Saving...' : 'Create Event'}
	</button>
</form>
