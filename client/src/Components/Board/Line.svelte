<script lang="ts">
	import { lineType, mapEnum, player } from "../../enums";
	import type { LineType } from "../../enums";
	import { createEventDispatcher } from "svelte";

	import { game } from "../../stores";

	export let average: number;
	export let line: LineType;
	export let index: number;

	let claimed = false;
	let lineColour = "hover";

	const dispatch = createEventDispatcher();

	$: if (claimed) {
		lineColour = $game.get_edge(index, mapEnum(lineType, line))
			? "red"
			: "blue" ?? "hover";
	}

	const handleClick = () => {
		if (claimed) return;

		$game.claim_edge(index, mapEnum(lineType, line));
		claimed = true;
		dispatch("edgeclick");
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
