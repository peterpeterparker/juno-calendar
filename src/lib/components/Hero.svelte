<script lang="ts">
	interface Props {
		href: string;
		ariaLabel: string;
		children?: import('svelte').Snippet;
	}

	let { href, ariaLabel, children }: Props = $props();
</script>

<a {href} rel="noreferrer noopener" target="_blank" aria-label={ariaLabel}>
	{@render children?.()}
</a>
