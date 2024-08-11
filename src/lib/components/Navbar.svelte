<script lang="ts">
	import Logout from '$lib/components/Logout.svelte';
	import { page } from '$app/stores';
	import { userSignedIn } from '$lib/derived/user.derived';
	import { fade } from 'svelte/transition';

	let currentPage: string;
	$: currentPage = $page.url.pathname;
</script>

<nav class="bg-base-100 shadow">
	<div class="container mx-auto px-4">
		<div class="flex justify-between items-center py-4">
			<a href="/" class="text-xl font-bold">Doodle App</a>
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
					</div>
				{/if}

				<Logout />
			</div>
		</div>
	</div>
</nav>
