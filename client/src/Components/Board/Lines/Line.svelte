<script lang="ts">
	// Component Imports
	import Horizontal from "./Horizontal.svelte";
	import Vertical from "./Vertical.svelte";
	import Hover from "./Hover.svelte";

	// Local Imports
	import {
		claimed,
		lineType,
		player,
		mapEnum,
		translateClaimed,
		type Claimed,
		type LineType,
	} from "../../../enums";
	import { game, affectedBoxes, settings } from "../../../stores";

	// Module Imports
	import chunk from "lodash/chunk";

	// External Props
	export let average: number;
	export let line: LineType;
	export let index: number;

	// Local Variables
	let lineClaimed: Claimed = claimed.EMPTY;

	const handleClick = () => {
		if ($game.current_player === mapEnum(player, player.COMPUTER)) return;

		// Calculate the boxes affected by the interaction
		const affected_boxes = $game.interact_edge(index, mapEnum(lineType, line));
		const chunked_affected_boxes: number[][] = chunk(affected_boxes, 2);
		const chunked_claimed_boxes = chunked_affected_boxes.filter(
			([y, x]) => translateClaimed($game.get_box(x, y)) !== claimed.EMPTY
		);

		// Save the affected boxes into the store
		$affectedBoxes = chunked_claimed_boxes;

		// Update the edge colour
		const edge = $game.get_edge(index, mapEnum(lineType, line));
		lineClaimed = translateClaimed(edge);

		// Computer turn
		$game.take_turn();
	};

	// Reactively update the colour when settings change
	let colour = "";

	$: {
		const colourMap = {
			[claimed.USER]: $settings.colours.user,
			[claimed.COMPUTER]: $settings.colours.computer,
		};

		colour = colourMap[lineClaimed];
	}
</script>

<div
	class="flex bg-transparent"
	title="line"
	style="
		width: {average}em;
		height: {average}em;
	"
	on:click={handleClick}
	on:keypress={handleClick}
>
	{#if lineClaimed === claimed.EMPTY}
		<Hover {line} />
	{:else if line === lineType.HORIZONTAL}
		<div class="line w-full h-4 z-40 -mt-2">
			<Horizontal stroke={colour} />
		</div>
	{:else if line === lineType.VERTICAL}
		<div class="line h-full w-8 z-40 -ml-4">
			<Vertical stroke={colour} />
		</div>
	{/if}
</div>
