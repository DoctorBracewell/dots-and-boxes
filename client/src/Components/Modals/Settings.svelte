<script lang="ts">
	import { writable } from "svelte/store";

	import { Game } from "../../../wasm/pkg/wasm";
	import { difficulty } from "../../enums";
	import boxURL from "../../imgs/box.svg";
	import { game, settings } from "../../stores";

	let width = $game.width;
	let height = $game.height;

	let newWidth;
	let newHeight;

	const handleClick = (event: Event) => {
		$settings.difficulty =
			difficulty[(event.target as HTMLInputElement).value];
	};

	$: {
		newWidth = width > 0 ? width : 1;
		newHeight = height > 0 ? height : 1;

		$game = new Game(newWidth, newHeight);
	}
</script>

<div class="flex flex-col justify-between gap-10 text-xl">
	<div class="colours flex">
		<span class="my-auto flex-shrink">colours:</span>
		<div class=" inline-flex w-full flex-grow justify-center gap-3">
			<div class="relative flex h-20 w-20">
				<img class="absolute h-full w-full" src={boxURL} alt="" />

				<div
					class="m-auto h-[91%] w-[91%]"
					style:backgroundColor={$settings.colours.user}
				>
					<input
						class="h-full w-full rounded-md border-none bg-transparent opacity-0"
						type="color"
						bind:value={$settings.colours.user}
					/>
				</div>
			</div>
			<span class="my-auto">│</span>
			<div class="relative flex h-20 w-20">
				<img
					class="absolute h-full w-full rotate-90"
					src={boxURL}
					alt=""
				/>

				<div
					class="m-auto h-[91%] w-[91%]"
					style:backgroundColor={$settings.colours.computer}
				>
					<input
						class="h-full w-full rounded-md border-none bg-transparent opacity-0"
						type="color"
						bind:value={$settings.colours.computer}
					/>
				</div>
			</div>
		</div>
	</div>

	<div class="colours flex">
		<span class="my-auto flex-shrink">boardㅤsize:</span>
		<div class=" inline-flex w-full flex-grow justify-center gap-3">
			<div class="relative flex h-20 w-20">
				<img class="absolute h-full w-full" src={boxURL} alt="" />

				<input
					class="relative z-10 m-auto h-full w-full rounded-md border-none bg-transparent text-center outline-none"
					type="text"
					bind:value={width}
				/>
			</div>
			<span class="my-auto">│</span>
			<div class="relative flex h-20 w-20">
				<img class="absolute h-full w-full" src={boxURL} alt="" />

				<input
					class="relative z-10 m-auto h-full w-full rounded-md border-none bg-transparent text-center outline-none"
					type="text"
					bind:value={height}
				/>
			</div>
		</div>
	</div>

	<div class="difficulty flex">
		<span class="my-auto flex-shrink">difficulty:</span>
		<div class="inline-flex w-full flex-grow justify-center gap-3">
			{#each Object.entries(difficulty) as [key, value], index}
				<div class="inline-block">
					<input
						on:click={handleClick}
						type="radio"
						name="difficulty"
						checked={$settings.difficulty === value}
						value={key}
						id={key}
						class="peer hidden"
					/>
					<label
						for={key}
						class="sliding-underline sliding-underline-small cursor-pointer transition-colors peer-checked:after:w-0"
						>{value}</label
					>
				</div>

				{#if index !== Object.keys(difficulty).length - 1}
					<span>│</span>
				{/if}
			{/each}
		</div>
	</div>
</div>
