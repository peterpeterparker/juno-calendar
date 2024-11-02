<script lang="ts">
	import Logout from '$lib/components/Logout.svelte';
	import { page } from '$app/stores';
	import { userSignedIn } from '$lib/derived/user.derived';
	import { fade } from 'svelte/transition';
	import { APP_NAME } from '$lib/constants/app.constants';

	let currentPage: string;
	$: currentPage = $page.url.pathname;
</script>

<nav class="bg-accent border-black border-b-2">
	<div class="container mx-auto px-4">
		<div class="flex justify-between items-center py-4">
			<a href="/" class="text-xl font-bold">{APP_NAME}</a>
			<div class="flex space-x-8">
				{#if $userSignedIn}
					<div in:fade>
						<a href="/" class={currentPage === '/' ? 'btn btn-primary' : 'btn btn-outline'}
							>Dashboard</a
						>
						<a
							href="/create-event"
							class={`hidden md:inline-flex ${currentPage === '/create-event' ? 'btn btn-primary' : 'btn btn-outline'}`}
							>Create Event</a
						>
						<a
							data-tid="btn-settings"
							href="/settings"
							class={`hidden md:inline-flex ${currentPage === '/settings' ? 'btn btn-primary' : 'btn btn-outline'}`}
							>Settings</a
						>
					</div>
				{/if}

				<Logout />
			</div>
		</div>
	</div>
</nav>
