<script lang="ts">
	import { eventKey } from '$lib/derived/page.derived';
	import { type Doc, getDoc, setDoc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';
	import { appState } from '$lib/stores/app.store';
	import { formatDate } from '$lib/utils/date.utils';
	import type { AnswerData, AnswersData } from '$lib/types/answers';
	import { nanoid } from 'nanoid';
	import { alertStore } from '$lib/stores/alert.store';

	let eventDoc: Doc<EventData> | undefined;

	let firstname = '';
	let answers: AnswerData[] | undefined;

	const sortAnswers = ({ date: dateA }: AnswerData, { date: dateB }: AnswerData): number =>
		dateA - dateB;

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

		answers = eventDoc?.data.dates
			.map((date) => ({
				date: new Date(date).getTime(),
				checked: false
			}))
			.sort(sortAnswers);
	};

	const onToggle = ({ date: selectedDate, checked }: AnswerData) => {
		answers = [
			...(answers ?? []).filter(({ date }) => date !== selectedDate),
			{
				date: selectedDate,
				checked: !checked
			}
		].sort(sortAnswers);
	};

	$: $eventKey, $appState, (async () => await loadEvent())();

	let progress = false;

	const handleSubmit = async ($event: SubmitEvent) => {
		$event.preventDefault();

		if (firstname === '') {
			alertStore.set({
				type: 'error',
				message: 'Please provide your firstname.'
			});
			return;
		}

		if (answers === undefined || answers.length === 0) {
			alertStore.set({
				type: 'error',
				message: 'No answers provided. That is unexpected.'
			});
			return;
		}

		progress = true;

		try {
			const key = nanoid();

			await setDoc<AnswersData>({
				collection: 'answers',
				doc: {
					key,
					data: {
						firstname,
						answers
					}
				}
			});

			alertStore.set({
				type: 'success',
				message: 'Your answers have been registered!'
			});

			// TODO: what do we do?
		} catch (err) {
			alertStore.set({
				type: 'error',
				message: 'Unexpected error while answering the event.'
			});
			console.error(err);
		}

		progress = false;
	};
</script>

{#if eventDoc !== undefined}
	<h1 class="text-2xl font-semibold mb-4">{eventDoc.data.title ?? 'Unknown'}</h1>
	<p class="mb-4">Select the dates you are available:</p>

	<!-- Availability Form -->
	<form class="space-y-4" on:submit={async ($event) => await handleSubmit($event)}>
		<div class="card shadow-md bg-base-100">
			<div class="card-body">
				<div class="form-control">
					<label class="label" for="event-title">
						<span class="label-text font-medium">Your Firstname</span>
					</label>
					<input
						type="text"
						id="event-title"
						class={`input input-bordered w-full ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
						bind:value={firstname}
						placeholder="Enter your firstname"
						required
						disabled={progress}
					/>
				</div>

				<div class="form-control">
					<label class="label" for="event-proposed-dates">
						<span class="label-text font-medium">Proposed Dates</span>
					</label>
					<div id="event-proposed-dates">
						{#each answers ?? [] as answer (answer.date)}
							<label class="label cursor-pointer">
								<span class="label-text">{formatDate(new Date(answer.date))}</span>
								<input
									type="checkbox"
									class={`checkbox ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
									on:change={() => onToggle(answer)}
									disabled={progress}
								/>
							</label>
						{/each}
					</div>
				</div>
			</div>
		</div>

		<button
			type="submit"
			class={`btn btn-accent w-full ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
			disabled={progress}>Submit Availability</button
		>
	</form>
{:else}
	<p>No corresponding event. 🤨</p>
{/if}
