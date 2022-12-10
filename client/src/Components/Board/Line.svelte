<script lang="ts">
	import { lineType, mapEnum } from "../../enums";
	import type { LineType } from "../../enums";
	import { game, updatedBoxes } from "../../stores";

	import chunk from "lodash/chunk";

	export let average: number;
	export let line: LineType;
	export let index: number;

	let claimed = false;
	let lineColour = "hover";

	$: if (claimed) {
		lineColour = $game.get_edge(index, mapEnum(lineType, line))
			? "red"
			: "blue" ?? "hover";
	}

	const handleClick = () => {
		const affected_boxes = $game.interact_edge(index, mapEnum(lineType, line));

		const chunked_affected_boxes: number[][] = chunk(affected_boxes, 2).filter(
			([y, x]) => $game.get_box(x, y) !== undefined
		);

		for (const box of chunked_affected_boxes) {
			$updatedBoxes = [...$updatedBoxes, box];
		}

		claimed = true;
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
			class="line w-full h-4{lineColour === 'hover'
				? ' opacity-0 hover:opacity-100 cursor-pointer '
				: ' '}z-40 -mt-2"
		>
			<img
				src="assets/imgs/game/line/horizontal/{lineColour}.svg"
				alt=""
				class="h-full"
			/>
		</div>
	{:else}
		<div
			class="line h-full w-4{lineColour === 'hover'
				? ' opacity-0 hover:opacity-100 cursor-pointer '
				: ' '}z-40 -ml-2"
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
