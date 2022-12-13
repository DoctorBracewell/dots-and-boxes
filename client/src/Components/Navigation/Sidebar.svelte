<script lang="ts">
	// Local Imports
	import { navigationEvent } from "../../enums";

	// Component Imports
	import Button from "./Button.svelte";
</script>

<nav class="flex flex-col text-right">
	<span class="mb-[0.55rem]">
		<Button label={navigationEvent.SETTINGS} on:navigationClick />
	</span>
	<span>
		<Button label={navigationEvent.LEADERBOARD} on:navigationClick />
	</span>
</nav>
