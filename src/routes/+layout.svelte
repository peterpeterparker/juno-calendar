<script lang="ts">
	import { onMount } from 'svelte';
	import { initSatellite } from '@junobuild/core';
	import Footer from '$lib/components/Footer.svelte';
	import '../app.css';
	import { appState } from '$lib/stores/app.store';
	import Navbar from '$lib/components/Navbar.svelte';
	import Alert from '$lib/components/Alert.svelte';
	import { hello } from '../declarations/satellite/satellite.api';
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();

	onMount(async () => {
		await initSatellite({
			workers: {
				auth: true
			}
		});

		appState.set('initialized');

		console.log(await hello('David'));
	});
</script>

<div class="flex flex-col min-h-screen">
	<Navbar />

	<!-- Main Content -->
	<div class="flex-grow container mx-auto px-4 py-8">
		<Alert />

		{@render children?.()}
	</div>

	<Footer />
</div>
