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
	<Navigation on:navigationClick={handleNavigationClick} />

	<Modals />

	<div class="m-auto flex flex-col relative h-full justify-center">
		<div class="w-full flex">
			<h1 class="text-4xl md:text-5xl m-auto title title-large w-fit">
				Dots & Boxes
			</h1>
		</div>
		<div class="w-full flex mt-4 md:mt-6 bg">
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
				var(--white) 2.5rem,
				var(--grey) 2.5rem,
				var(--grey) 2.55rem
			);
		}

		.red-margin {
			@apply after:md:absolute after:md:w-[0.1em] after:md:h-full after:md:bg-red-500 after:md:left-32;
		}
	}
</style>
