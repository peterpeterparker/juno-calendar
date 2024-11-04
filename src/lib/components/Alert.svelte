<script lang="ts">
	import { alertStore } from '$lib/stores/alert.store';
	import { fly } from 'svelte/transition';
</script>

{#if $alertStore !== undefined && $alertStore !== null}
	<div
		role="alert"
		class={`alert alert-${$alertStore.type} absolute w-full max-w-lg left-1/2 transform -translate-x-1/2 shadow-lg z-10`}
		transition:fly={{ y: -20, duration: 300 }}
	>
		<div class="flex justify-between items-center">
			{#if $alertStore.type === 'error'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6 shrink-0 stroke-current mr-2"
					fill="none"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
					/>
				</svg>
			{:else if $alertStore.type === 'success'}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6 shrink-0 stroke-current"
					fill="none"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
					/>
				</svg>
			{/if}
			<span>{$alertStore.message}</span>
			<button onclick={alertStore.reset} class="ml-4 text-xl font-bold cursor-pointer"
				>&times;</button
			>
		</div>
	</div>
{/if}
