<script lang="ts">
	import { onMount } from "svelte";
	import { URL, PORT } from "../../constants";
	import { settings } from "../../stores";

	interface Match {
		username: string;
		result: {
			user: number;
			computer: number;
		};
		board: {
			width: number;
			height: number;
		};
		game_time: string;
		id: string;
	}

	let matches: Match[] = [];

	const formatDate = (date: string) => {
		const dateObj = new Date(date);

		return `${dateObj.getHours()}:${dateObj.getMinutes()} ${dateObj.getDate()}/${dateObj.getMonth()}/${dateObj.getFullYear()}`;
	};

	onMount(async () => {
		const response = await fetch(`${URL}:${PORT}/matches`);

		matches = (await response.json())[0].result as Match[];
	});
</script>

<div
	class="border-2 border-black rounded-md w-full flex flex-col p-10 gap-3 max-h-[300px] overflow-y-scroll"
>
	{#each matches.sort((a, b) => b.result.user - a.result.user) as { username, result, board, game_time }, index}
		<div class="flex w-full m-auto justify-center align-middle gap-5 h-fit">
			<span>
				<span style="opacity: {result.user > result.computer ? '1' : '0'}"
					>🏆</span
				>
				{username}
			</span>│
			<div class="inline-flex justify-center gap-1">
				<span style="color: {$settings.colours.user}">{result.user}</span>
				<span>-</span>
				<span style="color: {$settings.colours.computer}"
					>{result.computer}</span
				>
			</div>
			│
			<span>{board.width} × {board.height}</span>
			│
			<span>{formatDate(game_time)}</span>
		</div>

		{#if index !== matches.length - 1}
			<div class="w-full h-[2px] bg-black rounded-lg mx-auto" />{/if}
	{/each}
</div>
