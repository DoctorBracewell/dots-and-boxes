<script lang="ts">
	import {
		type LineType,
		lineType as lineTypeEnum,
		player,
		translateNumber,
	} from "../../../enums";
	import horizontal from "../../../imgs/game/hover/horizontal.svg";
	import vertical from "../../../imgs/game/hover/vertical.svg";
	import { game } from "../../../stores";

	export let lineType: LineType;

	// Local Variables
	const isHorizontal = lineType === lineTypeEnum.HORIZONTAL;
	let userTurn = false;

	$: userTurn =
		translateNumber(player, $game.current_player) !== player.COMPUTER;

	// $: console.log($game.current_player);
</script>

<div
	class="hover-line {isHorizontal
		? '-mt-4 h-8 w-full'
		: '-ml-4 h-full w-8'} z-40 opacity-0 selection:z-40 {translateNumber(
		player,
		$game.current_player
	) !== player.COMPUTER && 'cursor-pointer hover:opacity-100'}"
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
