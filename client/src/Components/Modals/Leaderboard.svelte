<script lang="ts">
	import { onMount } from "svelte";

	import { PORT, URL } from "../../constants";
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
	let filteredMatches: Match[] = matches;
	let search = "";

	const formatDate = (date: string) => {
		const dateObj = new Date(date);

		return `${dateObj.getHours()}:${dateObj.getMinutes()} ${dateObj.getDate()}/${dateObj.getMonth()}/${dateObj.getFullYear()}`;
	};

	onMount(async () => {
		const response = await fetch(`${URL}:${PORT}/matches`);

		matches = (await response.json())[0].result as Match[];
	});

	$: filteredMatches =
		search.length === 0
			? matches
			: matches.filter((match) =>
					match.username.toLowerCase().includes(search)
			  );
</script>

<div>
	<input
		class="mb-5 w-full rounded-md border-2 border-black p-3"
		type="text"
		placeholder="Search for a specific player"
		bind:value={search}
	/>
	<div
		class="flex max-h-[300px] w-full flex-col gap-3 overflow-y-scroll rounded-md border-2 border-black p-10"
	>
		{#each filteredMatches.sort((a, b) => b.result.user - a.result.user) as { username, result, board, game_time }, index}
			<div
				class="m-auto flex h-fit w-full justify-center gap-5 align-middle"
			>
				<span>
					<span
						style="opacity: {result.user > result.computer
							? '1'
							: '0'}">🏆</span
					>
					{username}
				</span>│
				<div class="inline-flex justify-center gap-1">
					<span style="color: {$settings.colours.user}"
						>{result.user}</span
					>
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
				<div class="mx-auto h-[2px] w-full rounded-lg bg-black" />{/if}
		{/each}
	</div>
</div>
