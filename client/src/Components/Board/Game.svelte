<script lang="ts">
	import Box from "./Box.svelte";
	import Line from "./Line.svelte";

	import { difficulty, mapEnum } from "../../enums";
	import { game } from "../../stores";
	import { range } from "../../utils";

	import chunk from "lodash/chunk";

	$: average = 9 / Math.sqrt(($game.width + $game.height) / 2);

	$game.set_difficulty(mapEnum(difficulty, difficulty.MEDIUM));

	// $game.claim_edge(3, 1);
	$game.print_edges();
</script>

<main class="w-full h-full bg-transparent relative flex">
	<div class="m-auto">
		{#each [...range(0, $game.height)] as i}
			<div class="flex">
				{#each [...range(0, $game.width)] as j}
					<Box x={i} y={j} {average} />
				{/each}
			</div>
		{/each}
	</div>

	<div class="absolute edges">
		{#each chunk([...range(0, $game.width * ($game.height + 1))], 5) as row}
			<div class="flex">
				{#each row as edge}
					<Line {average} />
				{/each}
			</div>
		{/each}
	</div>
</main>
