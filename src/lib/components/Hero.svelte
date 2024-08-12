<script lang="ts">
	export let href: string;
	export let ariaLabel: string;
</script>

<a {href} rel="noreferrer noopener" target="_blank" aria-label={ariaLabel}>
	<slot />
</a>
