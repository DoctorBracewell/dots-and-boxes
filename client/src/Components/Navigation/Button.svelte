<script lang="ts">
	// Local Imports
	// Module Imports
	import { createEventDispatcher } from "svelte";

	import type { NavigationEvent } from "../../enums";

	// External Props
	export let label: NavigationEvent;

	// Dispatch a navigation event on click
	const dispatch = createEventDispatcher();
	const handleClick = () => dispatch("navigationClick", label);
</script>

<button
	on:click={handleClick}
	class="sliding-underline sliding-underline-medium m-0 border-none bg-none p-0"
	>{label}</button
>
