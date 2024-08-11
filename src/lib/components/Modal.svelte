<script lang="ts">
	import Backdrop from '$lib/components/Backdrop.svelte';
	import Button from '$lib/components/Button.svelte';
	import { userSignedIn } from '$lib/derived/user.derived';
	import { setDoc } from '@junobuild/core-peer';
	import { nanoid } from 'nanoid';
	import type { EventData } from '$lib/types/events';
	import { userStore } from '$lib/stores/user.store';

	let showModal = false;

	let inputText = '';

	let selectedDate1Input = '';
	let selectedDate1: Date | undefined;
	const onChange = () =>
		(selectedDate1 =
			selectedDate1Input !== undefined && selectedDate1Input !== ''
				? new Date(selectedDate1Input)
				: undefined);

	let selectedDate2Input = '';
	let selectedDate2: Date | undefined;
	const onChange2 = () =>
		(selectedDate2 =
			selectedDate2Input !== undefined && selectedDate2Input !== ''
				? new Date(selectedDate2Input)
				: undefined);

	let progress = false;

	let valid = false;
	$: valid = inputText !== '' && selectedDate1 !== undefined && $userSignedIn;

	const reload = () => {
		const event = new CustomEvent('exampleReload');
		window.dispatchEvent(event);
	};

	const add = async () => {
		// Demo purpose therefore edge case not properly handled
		if ($userStore === undefined || $userStore === null) {
			return;
		}

		progress = true;

		if (selectedDate1 === undefined || selectedDate2 === undefined) {
			// TODO: handle errors
			return;
		}

		try {
			const key = nanoid();

			await setDoc<EventData>({
				collection: 'events',
				doc: {
					key,
					data: {
						dates: [selectedDate1.getTime(), selectedDate2.getTime()]
					}
				}
			});

			showModal = false;

			reload();
		} catch (err) {
			console.error(err);
		}

		progress = false;
	};
</script>

<Button on:click={() => (showModal = true)}>
	Schedule an event
	<svg
		xmlns="http://www.w3.org/2000/svg"
		height="20"
		viewBox="0 -960 960 960"
		width="20"
		fill="currentColor"
	>
		<path d="M417-417H166v-126h251v-251h126v251h251v126H543v251H417v-251Z" />
	</svg>
</Button>

{#if showModal}
	<div class="fixed inset-0 z-50 p-16 md:px-24 md:py-44 animate-fade" role="dialog">
		<div class="relative w-full max-w-xl">
			<div>
				<label for="event-name">Your event name:</label>
				<input
					id="event-name"
					class="form-control block w-full mt-2 px-3 py-1.5 text-base font-normal m-0 resize-none border-black border-[3px] rounded-sm bg-white shadow-[5px_5px_0px_rgba(0,0,0,1)] focus:outline-none"
					class:opacity-50={progress}
					bind:value={inputText}
					disabled={progress}
				/>
			</div>

			<div class="flex flex-col mt-4">
				<p>Selected dates:</p>
				<input
					bind:value={selectedDate1Input}
					id="selected-date"
					name="selected-date"
					type="date"
					on:change={onChange}
					class="form-control block w-full mt-2 px-3 py-1.5 text-base font-normal m-0 resize-none border-black border-[3px] rounded-sm bg-white shadow-[5px_5px_0px_rgba(0,0,0,1)] focus:outline-none"
				/>
				<input
					bind:value={selectedDate2Input}
					id="selected-date"
					name="selected-date"
					type="date"
					on:change={onChange2}
					class="form-control block w-full mt-2 px-3 py-1.5 text-base font-normal m-0 resize-none border-black border-[3px] rounded-sm bg-white shadow-[5px_5px_0px_rgba(0,0,0,1)] focus:outline-none"
				/>
			</div>

			<div role="toolbar" class="flex justify-end items-center">
				{#if progress}
					<div
						class="my-8 animate-spin inline-block w-6 h-6 border-[3px] border-current border-t-transparent text-indigo-600 rounded-full"
						role="status"
						aria-label="loading"
					>
						<span class="sr-only">Loading...</span>
					</div>
				{:else}
					<div class="flex my-4">
						<button
							class="py-1 px-8 hover:text-lavender-blue-600 active:text-lavender-blue-400"
							type="button"
							on:click={() => (showModal = false)}
						>
							Close
						</button>

						<Button on:click={add} disabled={!valid}>Submit</Button>
					</div>
				{/if}
			</div>
		</div>
	</div>
	<Backdrop />
{/if}
