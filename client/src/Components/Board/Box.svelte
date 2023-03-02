<script lang="ts">
	import Background from "./Background.svelte";

	import { type Claimed, claimed, translateClaimed } from "../../enums";
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
			class="box relative flex"
			style="
			width: {average}rem;
			height: {average}rem;
		"
		>
			{#if claimedBy !== claimed.EMPTY}
				<Background {claimedBy} />

				<span class="z-10 m-auto text-xl">
					<!-- {labelMap[claimedBy]} -->
				</span>
			{/if}
			<span class="z-10 m-auto text-xl">
				{y * 5 + x + 1}
			</span>
		</div>
	</div>
{/key}
