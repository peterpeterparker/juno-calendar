<script lang="ts">
	interface Props {
		color?: 'primary' | 'secondary' | 'tertiary' | 'quaternary';
		disabled?: boolean;
		type?: 'submit' | 'button' | undefined;
		fullWidth?: boolean;
		children?: import('svelte').Snippet;
		onclick: () => Promise<void>;
	}

	let {
		color = 'primary',
		disabled = false,
		type = undefined,
		fullWidth = false,
		children,
		onclick
	}: Props = $props();

	let primary: boolean = $derived(color === 'primary');
	let secondary: boolean = $derived(color === 'secondary');
	let tertiary: boolean = $derived(color === 'tertiary');
	let quaternary: boolean = $derived(color === 'quaternary');
</script>

<button
	class:opacity-20={disabled}
	{onclick}
	{type}
	{disabled}
	class="flex items-center gap-2 border-black border-2 transition-all rounded-none h-12 px-5 hover:shadow-[2px_2px_0px_rgba(0,0,0,1)]"
	class:w-full={fullWidth}
	class:justify-center={fullWidth}
	class:bg-cyan-200={primary}
	class:hover:bg-cyan-300={primary}
	class:active:bg-cyan-400={primary}
	class:bg-violet-100={secondary}
	class:hover:bg-violet-200={secondary}
	class:active:bg-violet-300={secondary}
	class:bg-lime-200={tertiary}
	class:hover:bg-lime-300={tertiary}
	class:active:bg-lime-400={tertiary}
	class:bg-white={quaternary}
	class:hover:bg-gray-100={quaternary}
	class:active:bg-gray-200={quaternary}>{@render children?.()}</button
>
