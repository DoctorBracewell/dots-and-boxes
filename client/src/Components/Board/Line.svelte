<script lang="ts">
	// Local Imports
	import {
		claimed,
		lineType,
		mapEnum,
		translateClaimed,
		type Claimed,
		type LineType,
	} from "../../enums";
	import { game, affectedBoxes } from "../../stores";

	// Module Imports
	import chunk from "lodash/chunk";

	// External Props
	export let average: number;
	export let line: LineType;
	export let index: number;

	// Local Variables
	let lineColour: Claimed = claimed.EMPTY;
	let emptyClasses = "opacity-0 hover:opacity-100 cursor-pointer";

	const handleClick = () => {
		// Calculate the boxes affected by the interaction
		const affected_boxes = $game.interact_edge(index, mapEnum(lineType, line));
		const chunked_affected_boxes: number[][] = chunk(affected_boxes, 2);
		const chunked_claimed_boxes = chunked_affected_boxes.filter(
			([y, x]) => translateClaimed($game.get_box(x, y)) !== claimed.EMPTY
		);

		// Save the affected boxes into the store
		for (const box of chunked_claimed_boxes) {
			$affectedBoxes = [...$affectedBoxes, box];
		}

		// Update the edge colour
		const edge = $game.get_edge(index, mapEnum(lineType, line));
		lineColour = translateClaimed(edge);
	};
</script>

<div
	class="flex"
	style="
		width: {average}em;
		height: {average}em;
	"
	on:click={handleClick}
>
	{#if line === lineType.HORIZONTAL}
		<div
			class="line w-full h-6 z-40 -mt-3 
				{lineColour === claimed.EMPTY && emptyClasses}"
		>
			<img
				src="assets/imgs/game/line/horizontal/{lineColour}.svg"
				alt=""
				class="h-full w-full"
			/>
		</div>
	{:else}
		<div
			class="line h-full w-6 z-40 -ml-3 
				{lineColour === claimed.EMPTY && emptyClasses}"
		>
			<img
				src="assets/imgs/game/line/vertical/{lineColour}.svg"
				alt=""
				class="h-full w-full"
			/>
		</div>
	{/if}
</div>

<style>
	.line {
		transition: 0.1s opacity;
	}
</style>
