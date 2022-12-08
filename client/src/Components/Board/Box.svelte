<script lang="ts">
	import { game } from "../../stores";

	export let average: number;
	export let x: number;
	export let y: number;

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
		{#if showBackground}
			<span class="m-auto">
				{backgroundColour === "blue" ? "D" : "T"}
			</span>
		{/if}
	</div>
</div>
