<script lang="ts">
	import random from "lodash/random";

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
			class="absolute w-[1.6em] h-[1.6em] top-[-0.8em] left-[-0.8em] z-50"
			style="transform: rotate({random(360)}deg);"
			src="assets/imgs/game/dot.svg"
			alt=""
		/>
	</div>
</div>
