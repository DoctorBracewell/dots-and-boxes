<script lang="ts">
	import { game } from "../../stores";
	import Dot from "./Dot.svelte";

	export let average: number;

	export let [x, y]: [number, number] = [0, 0];
</script>

<div class="relative">
	<Dot />
	<div
		class="box flex"
		style="
			width: {average}em;
			height: {average}em;
			<!-- background-image: url('assets/imgs/background-{Math.random() > 0.5
			? 'red'
			: 'blue'}.svg') -->
		"
	>
		<span class="m-auto" />
	</div>
</div>
