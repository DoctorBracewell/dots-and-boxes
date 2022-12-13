<script lang="ts">
	// Component Imports
	import Box from "./Box.svelte";
	import Line from "./Line.svelte";
	import Dot from "./Dot.svelte";
	import Score from "./Score.svelte";

	// Local Imports
	import { difficulty, lineType, mapEnum } from "../../enums";
	import { game, affectedBoxes } from "../../stores";
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

	$game.difficulty = mapEnum(difficulty, difficulty.MEDIUM);

	// $game.print_board();
</script>

<main class="w-full h-full bg-transparent relative flex flex-col select-none">
	{#key $affectedBoxes}
		<div class="score mx-auto mb-5">
			<Score />
		</div>
	{/key}

	<div class="relative w-full h-full flex flex-col">
		<!-- {#key updateBoxes} -->
		<div class="m-auto">
			{#each [...range(0, height)] as y}
				<div class="flex">
					{#each [...range(0, width)] as x}
						<Box {x} {y} {average} />
					{/each}
				</div>
			{/each}
		</div>
		<!-- {/key} -->

		<!-- Create horizontal edges -->
		<div class="absolute horizontal-edges">
			{#each chunk([...range(0, horizontalEdges)], 5) as row}
				<div class="flex">
					{#each row as index}
						<Line {index} {average} line={lineType.HORIZONTAL} />
					{/each}
				</div>
			{/each}
		</div>

		<!-- Create vertical edges -->
		<div class="absolute vertical-edges">
			{#each chunk([...range(0, verticalEdges)], 6) as row}
				<div class="flex">
					{#each row as index}
						<Line {index} {average} line={lineType.VERTICAL} />
					{/each}
				</div>
			{/each}
		</div>

		<!-- Create dots -->
		<div class="absolute dots">
			{#each chunk([...range(0, (width + 1) * (height + 1))], 6) as row}
				<div class="flex">
					{#each row as _}
						<Dot {average} />
					{/each}
				</div>
			{/each}
		</div>
	</div>
</main>
