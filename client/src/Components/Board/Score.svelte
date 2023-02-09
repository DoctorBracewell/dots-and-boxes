<script lang="ts">
	// Local Imports
	import { mapEnum, player } from "../../enums";
	import { settings, game } from "../../stores";
</script>

<div class="flex">
	<span
		class="font-bold text-lg md:text-2xl"
		style="color: {$settings.colours.user}"
		>{$game.count_boxes(mapEnum(player, player.USER))}</span
	>
	<span class="mx-3">-</span>
	<span
		class="font-bold text-lg md:text-2xl"
		style="color: {$settings.colours.computer}"
		>{$game.count_boxes(mapEnum(player, player.COMPUTER))}</span
	>
</div>
