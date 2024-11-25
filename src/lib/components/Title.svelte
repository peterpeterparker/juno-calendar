<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<h1 class="w-fit text-2xl font-semibold bg-white p-2 border-[3px] border-solid border-black">
	{@render children()}
</h1>
