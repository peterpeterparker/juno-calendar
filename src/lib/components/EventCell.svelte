<script lang="ts">
	import type { Doc } from '@junobuild/core-peer';
	import type { EventData } from '$lib/types/events';

	export let eventDoc: Doc<EventData>;

	let displayDate: string;
	$: displayDate = new Date(eventDoc.data.dates[0]).toISOString();

	let shareUrl: string;
	$: shareUrl = `http://localhost:5173/?event=${eventDoc.key}`;
</script>

<div class="flex items-center justify-center gap-2">
	<output>{displayDate}</output>

	<a
		aria-label="Share event link"
		rel="noopener noreferrer"
		href={shareUrl}
		target="_blank"
		class="hover:text-lavender-blue-500 active:text-lavender-blue-400"
	>
		<svg width="16" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 29 29" fill="currentColor">
			<g>
				<rect fill="none" class="opacity-25" width="29" height="29" />
				<path
					d="M8.36,26.92c-2,0-3.88-.78-5.29-2.19C.15,21.81.15,17.06,3.06,14.14L12.57,4.64c.39-.39,1.02-.39,1.41,0s.39,1.02,0,1.41L4.48,15.56c-2.14,2.14-2.14,5.62,0,7.76,1.04,1.04,2.41,1.61,3.88,1.61s2.84-.57,3.88-1.61l12.79-12.79c1.47-1.47,1.47-3.87,0-5.34-1.47-1.47-3.87-1.47-5.34,0l-12.45,12.45c-.73.73-.73,1.91,0,2.64.73.73,1.91.73,2.64,0l9.17-9.17c.39-.39,1.02-.39,1.41,0s.39,1.02,0,1.41l-9.17,9.17c-1.51,1.51-3.96,1.51-5.47,0-1.51-1.51-1.51-3.96,0-5.47L18.26,3.77c2.25-2.25,5.92-2.25,8.17,0s2.25,5.92,0,8.17l-12.79,12.79c-1.41,1.41-3.29,2.19-5.29,2.19Z"
				/>
			</g>
		</svg>
	</a>
</div>
