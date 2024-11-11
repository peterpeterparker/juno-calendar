<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { authSubscribe } from '@junobuild/core-peer';
	import { userStore } from '$lib/stores/user.store';
	import { userSignedIn } from '$lib/derived/user.derived';
	import Login from '$lib/components/Login.svelte';
	import { fade } from 'svelte/transition';
	import { alertStore } from '$lib/stores/alert.store';
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();

	let unsubscribe: (() => void) | undefined = undefined;

	onMount(() => (unsubscribe = authSubscribe((user) => userStore.set(user))));
	onDestroy(() => unsubscribe?.());

	const automaticSignOut = () => {
		alertStore.set({
			type: 'error',
			message: 'Your session is expired. Please login again!'
		});
	};
</script>

<svelte:window onjunoSignOutAuthTimer={automaticSignOut} />

{#if $userSignedIn}
	<div in:fade>
		{@render children?.()}
	</div>
{:else}
	<Login />
{/if}
