<script lang="ts">
	import { lineType, type LineType } from "../../../enums";

	export let line: LineType;
</script>

{#if line === lineType.HORIZONTAL}
	<div
		class="line w-full h-8 z-40 -mt-4 opacity-0 hover:opacity-100 cursor-pointer"
	>
		<img
			src="assets/imgs/game/line/horizontal/hover.svg"
			alt=""
			class="h-full w-full"
		/>
	</div>
{:else}
	<div
		class="line h-full w-8 z-40 -ml-4 opacity-0 hover:opacity-100 cursor-pointer"
	>
		<img
			src="assets/imgs/game/line/vertical/hover.svg"
			alt=""
			class="h-full w-full"
		/>
	</div>
{/if}

<style>
	.line {
		transition: 0.1s opacity;
	}
</style>
