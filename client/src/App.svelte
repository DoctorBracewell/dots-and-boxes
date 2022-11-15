<script lang="ts">
	import Game from "./Components/Game.svelte";

	import init from "../wasm/pkg/wasm";
</script>

<div class="w-full flex-1 flex text-neutral-100 font-body text-block">
	{#await init() then}
		<Game />
	{/await}
</div>
