<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { authSubscribe } from '@junobuild/core-peer';
	import { userStore } from '$lib/stores/user.store';
	import { userSignedIn } from '$lib/derived/user.derived';
	import Login from '$lib/components/Login.svelte';
	import { fade } from 'svelte/transition';

	let unsubscribe: (() => void) | undefined = undefined;

	onMount(() => (unsubscribe = authSubscribe((user) => userStore.set(user))));
	onDestroy(() => unsubscribe?.());

	const automaticSignOut = () => console.log('Automatically signed out because session expired');
</script>

<svelte:window on:junoSignOutAuthTimer={automaticSignOut} />

{#if $userSignedIn}
	<div in:fade>
		<slot />
	</div>
{:else}
	<Login />
{/if}
