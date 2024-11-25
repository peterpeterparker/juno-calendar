<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<div class="relative card shadow-md bg-base-100 border-2 border-black mb-4">
	<div class="card-body">
		{@render children()}
	</div>
</div>
