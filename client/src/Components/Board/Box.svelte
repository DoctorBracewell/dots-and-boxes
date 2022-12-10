<script lang="ts">
	import { game, updatedBoxes } from "../../stores";
	import Background from "../SVG/Background.svelte";

	export let average: number;
	export let x: number;
	export let y: number;

	let claimed, showBackground, backgroundColour;

	$: {
		if ($updatedBoxes.some(([dy, dx]) => x === dx && y === dy)) {
			claimed = $game.get_box(x, y);
		}

		showBackground = claimed !== undefined;
		backgroundColour = claimed === 0 ? "blue" : "red";
	}

	// $: console.log($updatedBoxes);
</script>

{#key claimed}
	<div class="relative">
		<div
			class="box flex relative"
			style="
			width: {average}em;
			height: {average}em;
		"
		>
			{#if showBackground}
				<Background colour={backgroundColour} />

				<span class="m-auto z-10">
					{claimed === 0 ? "You" : "Me"}
				</span>
			{/if}
		</div>
	</div>
{/key}
