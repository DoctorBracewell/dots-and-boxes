<script lang="ts">
	// Local Imports
	import type { NavigationEvent } from "../../enums";

	// Module Imports
	import { createEventDispatcher } from "svelte";

	// External Props
	export let label: NavigationEvent;

	// Dispatch a navigation event on click
	const dispatch = createEventDispatcher();
	const handleClick = () => dispatch("navigationClick", label);
</script>

<button
	on:click={handleClick}
	class="bg-none p-0 m-0 border-none sliding-underline">{label}</button
>

<style>
	@tailwind components;

	@layer components {
		/* https://nerdcowboy.com/blog/sliding-underlined-links/ */
		.sliding-underline {
			@apply relative z-10;
			text-decoration: none;
		}

		.sliding-underline::before {
			content: "";
			position: absolute;
			bottom: -10%;
			right: -5%;
			height: 0.5em;
			width: 110%;
			background-image: url("assets/imgs/underline/small.svg");
			background-size: 110% 0.5em;
			background-position-y: 110%;
			background-position-x: -0.4em;
			background-repeat: no-repeat;
			z-index: -2;
		}

		.sliding-underline::after {
			@apply bg-white;
			content: "";
			position: absolute;
			bottom: -10%;
			right: -5%;
			height: 0.5em;
			width: 110%;
			z-index: -1;
			transition: width 0.2s ease-in-out;
		}

		.sliding-underline:hover::after {
			width: 0%;
		}
	}
</style>
