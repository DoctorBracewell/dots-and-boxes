<script lang="ts">
	// Local Imports
	import { game } from "../../../stores";
	import {
		lineType as lineTypeEnum,
		translateNumber,
		player,
		type LineType,
	} from "../../../enums";

	// Asset Imports
	import vertical from "../../../imgs/game/hover/vertical.svg";
	import horizontal from "../../../imgs/game/hover/horizontal.svg";

	export let lineType: LineType;

	// Local Variables
	const isHorizontal = lineType === lineTypeEnum.HORIZONTAL;
	let userTurn = false;

	$: userTurn = translateNumber(player, $game.current_player) === player.USER;

	$: console.log(userTurn);
</script>

<div
	class="hover-line {isHorizontal
		? 'w-full h-8 -mt-4'
		: 'h-full w-8 -ml-4'} z-40 selection:z-40 opacity-0 {userTurn &&
		'hover:opacity-100 cursor-pointer'}"
>
	<img
		src={isHorizontal ? horizontal : vertical}
		alt=""
		class="h-full w-full"
	/>
</div>

<style>
	.hover-line {
		transition: 0.1s opacity;
	}
</style>
