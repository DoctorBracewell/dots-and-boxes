<script lang="ts">
	import { draw } from "svelte/transition";

	export let stroke: string;
</script>

<svg
	class="h-full w-full"
	xmlns="http://www.w3.org/2000/svg"
	version="1.1"
	viewBox="9 9 96.84 3.38"
>
	<!-- svg-source:excalidraw -->
	<g stroke-linecap="round"
		><g
			transform="translate(9.775308895856142 10.47896108776331) rotate(0 47.64395874267936 0.21186067909002304)"
			><path
				d="M1.59 0.9 C27.11 -0.51, 51.83 0.13, 94.29 -0.48 M0.22 0.06 C27.95 1.45, 58.25 0.65, 95.06 0.31"
				{stroke}
				in:draw={{ delay: 0, duration: 500 }}
				stroke-width="2"
				fill="none"
			/></g
		></g
	></svg
>
