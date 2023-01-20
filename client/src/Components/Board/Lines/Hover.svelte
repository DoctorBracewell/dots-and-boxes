<script lang="ts">
	import { game } from "../../../stores";
	import { lineType, player, mapEnum, type LineType } from "../../../enums";

	export let line: LineType;

	let active = true;
	$: active = $game.current_player === mapEnum(player, player.USER);
</script>

<div
	class="line {line === lineType.HORIZONTAL
		? 'w-full h-8 -mt-4'
		: 'h-full w-8 -ml-4'} z-40 selection:z-40 opacity-0 {active &&
		'hover:opacity-100 cursor-pointer'}"
>
	<img
		src="assets/imgs/game/line/{line}/hover.svg"
		alt=""
		class="h-full w-full"
	/>
</div>

<style>
	.line {
		transition: 0.1s opacity;
	}
</style>
