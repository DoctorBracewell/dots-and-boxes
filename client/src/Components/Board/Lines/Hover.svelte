<script lang="ts">
	// Local Imports
	import { game } from "../../../stores";
	import { lineType, player, mapEnum, type LineType } from "../../../enums";

	// Asset Imports
	import vertical from "../../../imgs/game/hover/vertical.svg";
	import horizontal from "../../../imgs/game/hover/horizontal.svg";

	export let line: LineType;

	// Local Variables
	$: active = $game.current_player === mapEnum(player, player.USER);
	const isHorizontal = line === lineType.HORIZONTAL;
</script>

<div
	class="line {isHorizontal
		? 'w-full h-8 -mt-4'
		: 'h-full w-8 -ml-4'} z-40 selection:z-40 opacity-0 {active &&
		'hover:opacity-100 cursor-pointer'}"
>
	<img
		src={isHorizontal ? horizontal : vertical}
		alt=""
		class="h-full w-full"
	/>
</div>

<style>
	.line {
		transition: 0.1s opacity;
	}
</style>
