<script lang="ts">
	// Local Imports
	import { mapEnum, player } from "../../enums";
	import { game } from "../../stores";
</script>

<div class="flex">
	<span class="font-bold text-lg text-blue"
		>{$game.count_boxes(mapEnum(player, player.USER))}</span
	>
	<span class="mx-2">-</span>
	<span class="font-bold text-lg text-red"
		>{$game.count_boxes(mapEnum(player, player.COMPUTER))}</span
	>
</div>
