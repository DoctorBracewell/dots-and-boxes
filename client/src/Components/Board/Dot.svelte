<script lang="ts">
	import random from "lodash/random";

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
			class="absolute top-[-1em] left-[-1em] z-[45] h-[2em] w-[2em] select-none"
			style="transform: rotate({random(360)}deg);"
			src={dotURL}
			alt=""
		/>
	</div>
</div>
