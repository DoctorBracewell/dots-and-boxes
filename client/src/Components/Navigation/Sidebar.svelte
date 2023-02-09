<script lang="ts">
	// Local Imports
	import { navigationEvent } from "../../enums";

	// Component Imports
	import Button from "./Button.svelte";
</script>

<nav class="absolute right-[2.95em] top-[2.95em]">
	<div class="flex flex-col text-right text-xl">
		<span class="mb-[0.9rem]">
			<Button label={navigationEvent.SETTINGS} on:navigationClick />
		</span>
		<span>
			<Button label={navigationEvent.LEADERBOARD} on:navigationClick />
		</span>
	</div>
</nav>
