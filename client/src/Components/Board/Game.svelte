<script lang="ts">
	import { difficulty, mapEnum } from "../../enums";
	import { game } from "../../stores";

	$game.set_difficulty(mapEnum(difficulty, difficulty.MEDIUM));

	$game.log_difficulty();

	const board = Array.from(Array($game.height), () =>
		new Array($game.width).fill(0)
	);
</script>

<main class="w-full h-full bg-transparent">
	{#each board as row, i}
		<div class="flex">
			{#each row as cell, j}
				<div class="w-8 h-8 bg-red m-2" />
			{/each}
		</div>
	{/each}
</main>
