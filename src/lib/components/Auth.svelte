<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { authSubscribe } from '@junobuild/core-peer';
	import { userStore } from '$lib/stores/user.store';
	import { userSignedIn } from '$lib/derived/user.derived';
	import Login from '$lib/components/Login.svelte';
	import { fade } from 'svelte/transition';
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();

	let unsubscribe: (() => void) | undefined = undefined;

	onMount(() => (unsubscribe = authSubscribe((user) => userStore.set(user))));
	onDestroy(() => unsubscribe?.());

	const automaticSignOut = () => console.log('Automatically signed out because session expired');
</script>

<svelte:window onjunoSignOutAuthTimer={automaticSignOut} />

{#if $userSignedIn}
	<div in:fade>
		{@render children?.()}
	</div>
{:else}
	<Login />
{/if}
