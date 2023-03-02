<script lang="ts">
	import chunk from "lodash/chunk";

	import Box from "./Box.svelte";
	import Dot from "./Dot.svelte";
	import Line from "./Lines/Line.svelte";
	import Score from "./Score.svelte";

	import { PORT, URL } from "../../constants";
	import { lineType, mapEnum, player } from "../../enums";
	import { game, gameState } from "../../stores";
	import { checkUsername, range } from "../../utils";

	interface Body {
		username: string;
		result: {
			user: number;
			computer: number;
		};
		board: {
			width: number;
			height: number;
		};
	}

	// Local Variables
	let width: number,
		height: number,
		average: number,
		horizontalEdges: number,
		verticalEdges: number;

	$: {
		width = $game.width;
		height = $game.height;

		average = 11 / Math.sqrt((width + height) / 2);

		horizontalEdges = (height + 1) * width;
		verticalEdges = (width + 1) * height;
	}

	const handleGameEnd = async () => {
		await new Promise((resolve) => setTimeout(resolve, 300));

		const username = prompt("Please enter your name:");
		const failedChecks = checkUsername(username);

		// Cancelled
		if (username === null) return;

		if (failedChecks.length > 0) {
			alert(
				`${failedChecks.map((c) => c.string).join(".\n")}.\n\nPlease try again!`
			);
			return handleGameEnd();
		}

		const body: Body = {
			username,
			result: {
				user: await $game.count_boxes(mapEnum(player, player.USER)),
				computer: await $game.count_boxes(mapEnum(player, player.COMPUTER)),
			},
			board: {
				width: $game.width,
				height: $game.height,
			},
		};

		await fetch(`${URL}:${PORT}/matches`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(body),
		});
	};

	// $game.print_board();
</script>

<main class="relative flex h-full w-full select-none flex-col bg-transparent">
	{#key $gameState.affectedBoxes}
		<div class="score mx-auto mb-5 md:mb-6">
			<Score />
		</div>
	{/key}

	<div class="relative m-auto flex h-full flex-col">
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
		<div class="horizontal-edges absolute">
			{#each chunk([...range(0, horizontalEdges)], $game.width) as row}
				<div class="flex">
					{#each row as index}
						<Line
							{index}
							{average}
							lineType={lineType.HORIZONTAL}
							on:gameend={handleGameEnd}
						/>
					{/each}
				</div>
			{/each}
		</div>

		<!-- Create vertical edges -->
		<div class="vertical-edges absolute">
			{#each chunk([...range(0, verticalEdges)], $game.height + 1) as row}
				<div class="flex">
					{#each row as index}
						<Line
							{index}
							{average}
							lineType={lineType.VERTICAL}
							on:gameend={handleGameEnd}
						/>
					{/each}
				</div>
			{/each}
		</div>

		<!-- Create dots -->
		<div class="dots absolute">
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
