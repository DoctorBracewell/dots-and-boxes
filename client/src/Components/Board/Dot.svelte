<script lang="ts">
	// Module Imports
	import random from "lodash/random";

	// Asset Imports
	import dotURL from "../../imgs/game/dot.svg";

	// External Props
	export let average: number;
	export let [x, y]: [number, number] = [0, 0];
</script>

<div class="relative">
	<div
		class="flex"
		style="
			width: {average}em;
			height: {average}em;
		"
	>
		<img
			class="absolute w-[2em] h-[2em] top-[-1em] left-[-1em] z-[45]"
			style="transform: rotate({random(360)}deg);"
			src={dotURL}
			alt=""
		/>
	</div>
</div>
