<script lang="ts">
	import { Modals, openModal } from "svelte-modals";

	import Game from "./Board/Game.svelte";
	import Modal from "./Modals/Modal.svelte";
	import Navigation from "./Navigation/Sidebar.svelte";

	import type { NavigationEvent } from "../enums";

	// Modal Handling
	const handleNavigationClick = (event: CustomEvent<NavigationEvent>) => {
		openModal(Modal, {
			event: event.detail,
		});
	};
</script>

<div
	class="bg-paper relative flex h-full w-full after:md:absolute after:md:left-32 after:md:h-full after:md:w-[0.1em] after:md:bg-red-500"
>
	<Navigation on:navigationClick={handleNavigationClick} />

	<Modals />

	<div class="relative m-auto flex h-full flex-col justify-center">
		<div class="flex w-full">
			<h1 class="title title-large m-auto w-fit text-4xl md:text-5xl">
				Dots & Boxes
			</h1>
		</div>
		<div class="bg mt-4 flex w-full md:mt-6">
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
	}
</style>
