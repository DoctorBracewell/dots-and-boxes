<script lang="ts">
	// Component Imports
	import Background from "./Background.svelte";

	// Local Imports
	import { claimed, translateClaimed, type Claimed } from "../../enums";
	import { game, gameState } from "../../stores";

	// External Props
	export let average: number;
	export let x: number;
	export let y: number;

	// Local Variables
	let claimedBy: Claimed = claimed.EMPTY;

	const labelMap = {
		[claimed.USER]: "You",
		[claimed.COMPUTER]: "Me",
	};

	// Rerender box if it has been affected by a edge interaction
	$: if ($gameState.affectedBoxes.some(([dy, dx]) => x === dx && y === dy)) {
		claimedBy = translateClaimed($game.get_box(x, y));
	}
</script>

{#key claimedBy}
	<div class="relative">
		<div
			class="box flex relative"
			style="
			width: {average}rem;
			height: {average}rem;
		"
		>
			{#if claimedBy !== claimed.EMPTY}
				<Background {claimedBy} />

				<span class="m-auto z-10 text-xl">
					{labelMap[claimedBy]}
				</span>
			{/if}
		</div>
	</div>
{/key}
