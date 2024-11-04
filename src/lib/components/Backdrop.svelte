<script lang="ts">
	interface Props {
		spinner?: boolean;
	}

	let { spinner = false }: Props = $props();
</script>

<div class:backdrop-blur-sm={spinner} class:backdrop-blur-xl={!spinner}>
	{#if spinner}
		<div
			class="w-12 h-12 rounded-full animate-spin border-[3px] border-solid border-lavender-blue-600 border-t-transparent"
		></div>
	{/if}
</div>
