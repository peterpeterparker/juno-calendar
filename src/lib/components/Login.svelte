<script lang="ts">
	import { signIn } from '@junobuild/core-peer';

	const login = async () => {
		await signIn();
	};
</script>

<div class="flex flex-col items-center justify-center min-h-[60vh] text-center">
	<h1 class="text-3xl font-bold mb-6">Welcome to Doodle App</h1>
	<p class="text-lg mb-4">To view and manage your events, please sign in.</p>
	<p class="text-gray-600 mb-8">
		By signing in, you can create new events, share them with your friends, and keep track of their
		availability.
	</p>
	<button on:click={login} class="btn btn-primary" data-tid="login-button">Sign In</button>
</div>
