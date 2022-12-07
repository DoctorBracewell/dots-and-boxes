<script lang="ts">
	import { game } from "../../stores";

	export let average: number;
	export let [x, y]: [number, number] = [0, 0];

	let claimed, showBackground, backgroundColour;

	$: {
		claimed = $game.get_box(x, y);

		showBackground = claimed !== undefined;
		backgroundColour = claimed === 0 ? "blue" : "red";
	}
</script>

<div class="relative">
	<!-- {#if x === 0 && y === 4}
		{@debug claimed}
		{$game.print_edges()}
	{/if} -->

	<div
		class="box flex"
		style="
			width: {average}em;
			height: {average}em;
			{showBackground &&
			`background-image: url('assets/imgs/game/background/${backgroundColour}.svg')`}
		"
	>
		<span class="m-auto">
			<!-- {who ? "Me" : "You"} -->
			{x},{y}
		</span>
	</div>
</div>
