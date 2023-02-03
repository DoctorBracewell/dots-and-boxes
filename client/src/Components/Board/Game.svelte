<script lang="ts">
	// Component Imports
	import Box from "./Box.svelte";
	import Line from "./Lines/Line.svelte";
	import Dot from "./Dot.svelte";
	import Score from "./Score.svelte";

	// Local Imports
	import { lineType } from "../../enums";
	import { game, gameState } from "../../stores";
	import { range } from "../../utils";

	// Module Imports
	import chunk from "lodash/chunk";

	// Local Variables
	let width: number,
		height: number,
		average: number,
		horizontalEdges: number,
		verticalEdges: number;

	$: {
		width = $game.width;
		height = $game.height;

		average = 9 / Math.sqrt((width + height) / 2);

		horizontalEdges = (height + 1) * width;
		verticalEdges = (width + 1) * height;
	}

	// $game.print_board();
</script>

<main class="w-full h-full bg-transparent relative flex flex-col select-none">
	{#key $gameState.affectedBoxes}
		<div class="score mx-auto mb-5">
			<Score />
		</div>
	{/key}

	<div class="relative w-full h-full flex flex-col">
		<div class="m-auto">
			{#each [...range(0, height)] as y}
				<div class="flex">
					{#each [...range(0, width)] as x}
						<Box {x} {y} {average} />
					{/each}
				</div>
			{/each}
		</div>

		<!-- Create horizontal edges -->
		<div class="absolute horizontal-edges">
			{#each chunk([...range(0, horizontalEdges)], $game.width) as row}
				<div class="flex">
					{#each row as index}
						<Line {index} {average} line={lineType.HORIZONTAL} />
					{/each}
				</div>
			{/each}
		</div>

		<!-- Create vertical edges -->
		<div class="absolute vertical-edges">
			{#each chunk([...range(0, verticalEdges)], $game.height + 1) as row}
				<div class="flex">
					{#each row as index}
						<Line {index} {average} line={lineType.VERTICAL} />
					{/each}
				</div>
			{/each}
		</div>

		<!-- Create dots -->
		<div class="absolute dots">
			{#each chunk([...range(0, (width + 1) * (height + 1))], $game.width + 1) as row}
				<div class="flex">
					{#each row as _}
						<Dot {average} />
					{/each}
				</div>
			{/each}
		</div>
	</div>
</main>
