<script lang="ts">
	import { signOut } from '@junobuild/core-peer';
	import { userSignedIn } from '$lib/derived/user.derived';
	import { fade } from 'svelte/transition';
</script>

{#if $userSignedIn}
	<button class="btn btn-secondary" in:fade on:click={signOut}>Logout</button>
{/if}
