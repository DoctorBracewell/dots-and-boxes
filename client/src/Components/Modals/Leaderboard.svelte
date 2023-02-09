<script lang="ts">
	import { onMount } from "svelte";
	import { DEVELOPMENT } from "../../constants";

	const URL = DEVELOPMENT ? "http://127.0.0.1" : "todo";
	const PORT = DEVELOPMENT ? 3000 : 3000;

	interface Match {
		username: string;
		result: {
			player: number;
			opponent: number;
		};
		board: {
			width: number;
			height: number;
		};
		game_time: string;
		id: string;
	}

	let matches: Match[] = [];

	onMount(async () => {
		const response = await fetch(`${URL}:${PORT}/matches`);

		matches = (await response.json()) as Match[];
	});
</script>

<div>
	{#each matches as match}{match[0].username}{/each}
</div>
