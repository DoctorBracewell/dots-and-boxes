<script lang="ts">
	import Game from "./Board/Game.svelte";
	import Navigation from "./Navigation/Sidebar.svelte";

	const handleNavigationClick = (event: CustomEvent<{ label: string }>) => {
		console.log(event.detail.label);
	};
</script>

<div class="w-full h-full flex relative bg-paper">
	<div class="absolute right-[2.4em] top-[2.4em]">
		<Navigation on:navigationClick={handleNavigationClick} />
	</div>
	<div class="relative m-auto">
		<Game />
	</div>
</div>
