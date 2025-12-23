<script lang="ts">
	import Button from "./Button.svelte";

	import { navigationEvent } from "../../enums";
</script>

<nav class="absolute right-[2.95em] top-[2.95em]">
	<div class="flex flex-col text-right text-xl">
		<span class="mb-[0.9rem]">
			<Button label={navigationEvent.SETTINGS} on:navigationClick />
		</span>
		<span>
			<Button label={navigationEvent.LEADERBOARD} on:navigationClick />
		</span>
		<span>
			<a href="https://github.com/DoctorBracewell/dots-and-boxes">source<a/>
		</span>
	</div>
</nav>
