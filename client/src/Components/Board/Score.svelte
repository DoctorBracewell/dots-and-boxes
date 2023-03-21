<script lang="ts">
	import { mapEnum, player } from "../../enums";
	import { game, settings } from "../../stores";
</script>

<div class="flex">
	<span
		class="text-lg font-bold md:text-2xl"
		style:color={$settings.colours.user}
		>{$game.count_boxes(mapEnum(player, player.USER))}</span
	>
	<span class="mx-3">-</span>
	<span
		class="text-lg font-bold md:text-2xl"
		style:color={$settings.colours.computer}
		>{$game.count_boxes(mapEnum(player, player.COMPUTER))}</span
	>
</div>
