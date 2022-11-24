<script lang="ts">
	import Button from "./Button.svelte";
</script>

<nav class="flex flex-col text-right">
	<span class="mb-[0.65em]">
		<Button label="settings" on:navigationClick />
	</span>
	<span>
		<Button label="leaderboard" on:navigationClick />
	</span>
</nav>
