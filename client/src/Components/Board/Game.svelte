<script lang="ts">
	import Box from "./Box.svelte";
	import Line from "./Line.svelte";
	import Dot from "./Dot.svelte";

	import { difficulty, lineType, mapEnum } from "../../enums";
	import { game } from "../../stores";
	import { range } from "../../utils";

	import chunk from "lodash/chunk";

	let width, height, average, horizontalEdges, verticalEdges;
	let updateBoxes = false;

	$: {
		width = $game.width;
		height = $game.height;

		average = 9 / Math.sqrt((width + height) / 2);

		horizontalEdges = (height + 1) * width;
		verticalEdges = (width + 1) * height;
	}

	$game.set_difficulty(mapEnum(difficulty, difficulty.MEDIUM));

	const handleEdgeClick = () => {
		updateBoxes = !updateBoxes;
	};

	// $game.claim_edge(3, 1);
	// $game.print_b();
</script>

<main class="w-full h-full bg-transparent relative flex">
	<!-- Create boxes -->

	{#key updateBoxes}
		<div class="m-auto">
			{#each [...range(0, height)] as y}
				<div class="flex">
					{#each [...range(0, width)] as x}
						<Box {x} {y} {average} />
					{/each}
				</div>
			{/each}
		</div>
	{/key}

	<!-- Create horizontal edges -->
	<div class="absolute horizontal-edges">
		{#each chunk([...range(0, horizontalEdges)], 5) as row}
			<div class="flex">
				{#each row as index}
					<Line
						{index}
						{average}
						line={lineType.HORIZONTAL}
						on:edgeclick={handleEdgeClick}
					/>
				{/each}
			</div>
		{/each}
	</div>

	<!-- Create vertical edges -->
	<div class="absolute vertical-edges">
		{#each chunk([...range(0, verticalEdges)], 6) as row}
			<div class="flex">
				{#each row as index}
					<Line
						{index}
						{average}
						line={lineType.VERTICAL}
						on:edgeclick={handleEdgeClick}
					/>
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
</main>
