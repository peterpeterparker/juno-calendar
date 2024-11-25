<script lang="ts">
	import { formatDate } from '$lib/utils/date.utils';
	import { alertStore } from '$lib/stores/alert.store';
	import { nanoid } from 'nanoid';
	import { type Doc, setDoc } from '@junobuild/core-peer';
	import type { AnswerData, AnswersData } from '$lib/types/answers';
	import type { EventData } from '$lib/types/events';
	import { onMount } from 'svelte';
	import Card from '$lib/components/Card.svelte';

	interface Props {
		eventDoc: Doc<EventData>;
		onSuccess: () => void;
	}

	let { eventDoc, onSuccess }: Props = $props();

	let firstname = $state('');
	let answers: AnswerData[] | undefined = $state();

	onMount(() => {
		answers = eventDoc?.data.dates
			.map((date) => ({
				date: new Date(date).getTime(),
				checked: false
			}))
			.sort(sortAnswers);
	});

	const sortAnswers = ({ date: dateA }: AnswerData, { date: dateB }: AnswerData): number =>
		dateA - dateB;

	const onToggle = ({ date: selectedDate, checked }: AnswerData) => {
		answers = [
			...(answers ?? []).filter(({ date }) => date !== selectedDate),
			{
				date: selectedDate,
				checked: !checked
			}
		].sort(sortAnswers);
	};

	let progress = $state(false);

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

		if (eventDoc === undefined) {
			alertStore.set({
				type: 'error',
				message: 'The event has not been initialized. That is unexpected.'
			});
			return;
		}

		progress = true;

		try {
			const answerKey = nanoid();

			await setDoc<AnswersData>({
				collection: 'answers',
				doc: {
					key: answerKey,
					description: eventDoc?.key,
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

			onSuccess();
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

<form onsubmit={async ($event) => await handleSubmit($event)}>
	<Card>
		<p>Select the dates you are available:</p>

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
							onchange={() => onToggle(answer)}
							disabled={progress}
						/>
					</label>
				{/each}
			</div>
		</div>
	</Card>

	<button
		type="submit"
		class={`btn btn-accent w-full ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
		disabled={progress}>Submit Availability</button
	>
</form>
