<script lang="ts">
	import Auth from '$lib/components/Auth.svelte';
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();
</script>

<Auth>
	{@render children?.()}
</Auth>
