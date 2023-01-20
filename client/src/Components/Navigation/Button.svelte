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
	class="bg-none p-0 m-0 border-none sliding-underline sliding-underline-medium"
	>{label}</button
>
