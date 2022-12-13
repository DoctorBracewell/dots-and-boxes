<script lang="ts">
	// Component Imports
	import Game from "./Board/Game.svelte";
	import Navigation from "./Navigation/Sidebar.svelte";

	// ! TODO Modal Handling
	const handleNavigationClick = (event: CustomEvent<{ label: string }>) => {
		console.log(event.detail.label);
	};
</script>

<div class="w-full h-full flex relative bg-paper red-margin">
	<div class="absolute right-[2.4em] top-[2.4em]">
		<Navigation on:navigationClick={handleNavigationClick} />
	</div>
	<div class="m-auto flex flex-col relative h-full justify-center">
		<div class="w-full flex">
			<h1 class="text-3xl m-auto title w-fit">Dots & Boxes</h1>
		</div>
		<div class="w-full flex mt-3">
			<Game />
		</div>
	</div>
</div>

<style>
	@tailwind components;

	@layer components {
		.bg-paper {
			@apply relative;
			background: repeating-linear-gradient(
				var(--white),
				var(--white) 2rem,
				var(--grey) 2rem,
				var(--grey) 2.05rem
			);
		}

		.red-margin {
			@apply after:md:absolute after:md:w-[0.1em] after:md:h-full after:md:bg-red after:md:left-24;
		}

		.title {
			@apply relative z-10 no-underline;
		}

		.title::before {
			content: "";
			position: absolute;
			left: -10%;
			bottom: -50%;
			width: 120%;
			height: 1em;
			background-image: url("assets/imgs/underline/large.svg");
			background-size: 100% 1em;
			background-repeat: no-repeat;
			z-index: -2;
		}
	}
</style>
