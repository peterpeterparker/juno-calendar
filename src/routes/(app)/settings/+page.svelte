<script lang="ts">
	import { userStore } from '$lib/stores/user.store';
	import { alertStore } from '$lib/stores/alert.store';
	import { type Doc, getDoc, setDoc } from '@junobuild/core-peer';
	import type { SettingData } from '$lib/types/settings';
	import { onMount } from 'svelte';

	let email = $state('');
	let progress = $state(true);

	let settings: Doc<SettingData> | undefined;

	onMount(async () => {
		if ($userStore === undefined || $userStore === null) {
			alertStore.set({
				type: 'error',
				message: 'No user. Did you sign-in?'
			});
			return;
		}

		try {
			settings = await getDoc({
				collection: 'settings',
				key: $userStore.key
			});

			email = settings?.data.email ?? '';
		} catch (err: unknown) {
			alertStore.set({
				type: 'error',
				message: 'Cannot retrieve your settings.'
			});
			console.error(err);
		} finally {
			progress = false;
		}
	});

	const handleSubmit = async ($event: SubmitEvent) => {
		$event.preventDefault();

		if ($userStore === undefined || $userStore === null) {
			alertStore.set({
				type: 'error',
				message: 'No user. Did you sign-in?'
			});
			return;
		}

		progress = true;

		try {
			settings = await setDoc<SettingData>({
				collection: 'settings',
				doc: {
					...(settings !== undefined
						? settings
						: {
								key: $userStore.key
							}),
					data: {
						email
					}
				}
			});

			alertStore.set({
				type: 'success',
				message: 'Settings successfully saved!'
			});
		} catch (err) {
			alertStore.set({
				type: 'error',
				message: 'Unexpected error while saving the settings.'
			});
			console.error(err);
		}

		progress = false;
	};
</script>

<h1 class="text-2xl font-semibold mb-6 text-left mt-2">Settings</h1>

<form
	onsubmit={async ($event) => await handleSubmit($event)}
	class="space-y-6 bg-base-100 p-6 border-2 border-black mt-10"
>
	<div class="form-control">
		<label class="label" for="event-title">
			<span class="label-text font-medium">Email</span>
		</label>
		<input
			type="email"
			id="event-title"
			class={`input input-bordered w-full ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
			bind:value={email}
			placeholder="Enter your email address"
			required
			disabled={progress}
			data-tid="input-email"
		/>
	</div>

	<button
		type="submit"
		class={`btn btn-primary btn-block ${progress ? 'opacity-50 cursor-not-allowed' : ''}`}
		disabled={progress}
		data-tid="btn-save-settings"
	>
		{progress ? 'Saving...' : 'Save'}
	</button>
</form>
