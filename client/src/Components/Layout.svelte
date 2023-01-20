<script lang="ts">
	// Component Imports
	import Game from "./Board/Game.svelte";
	import Navigation from "./Navigation/Sidebar.svelte";
	import Modal from "./Modals/Modal.svelte";

	// Module Imports
	import { Modals, openModal } from "svelte-modals";
	import type { NavigationEvent } from "../enums";

	// Modal Handling
	const handleNavigationClick = (event: CustomEvent<NavigationEvent>) => {
		openModal(Modal, {
			event: event.detail,
		});
	};
</script>

<div class="w-full h-full flex relative bg-paper red-margin">
	<div class="absolute right-[2.4em] top-[2.4em]">
		<Navigation on:navigationClick={handleNavigationClick} />
	</div>

	<Modals />

	<div class="m-auto flex flex-col relative h-full justify-center">
		<div class="w-full flex">
			<h1 class="text-3xl m-auto title title-large w-fit">Dots & Boxes</h1>
		</div>
		<div class="w-full flex mt-3 bg">
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
			@apply after:md:absolute after:md:w-[0.1em] after:md:h-full after:md:bg-red-500 after:md:left-24;
		}
	}
</style>
