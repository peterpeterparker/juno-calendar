<script lang="ts">
	import { signIn } from '@junobuild/core-peer';
	import { APP_NAME } from '$lib/constants/app.constants';
	import Button from '$lib/components/ui/Button.svelte';

	const login = async () => {
		await signIn();
	};
</script>

<div class="flex flex-col items-center justify-center min-h-[60vh] text-center bg-white border-black border-2 max-w-[700px] mx-auto">
	<h1 class="text-3xl font-bold mb-6">Welcome to {APP_NAME}</h1>
	<p class="text-lg mb-4">To simple way to schedule anything with your busy friends.</p>
	<p class="text-gray-600 mb-8 max-w-[600px]">
		By signing in, you can create events and share them with your friends or colleagues, to maybe find a date to finally got that coffee you have been trying to schedule for months.
	</p>
	<Button on:click={login}>Sign In</Button>
</div>
