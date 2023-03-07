<script lang="ts">
	import { draw } from "svelte/transition";

	export let stroke: string;
</script>

<svg
	class="h-full w-full"
	xmlns="http://www.w3.org/2000/svg"
	version="1.1"
	viewBox="9 9 2.98 72.08"
>
	<!-- svg-source:excalidraw -->
	<g stroke-linecap="round"
		><g
			transform="translate(10.634629119187593 11.012141920626163) rotate(0 -0.14224320622906816 34.02681378884024)"
			><path
				d="M0.02 -1.01 C-0.52 28.78, -0.68 53.42, -0.47 69.07 M0.02 0.77 C0.32 26.53, 0.81 52.93, -0.63 68.54"
				{stroke}
				in:draw={{ delay: 0, duration: 500 }}
				stroke-width="2"
				fill="none"
			/></g
		></g
	><mask /></svg
>
