<script lang="ts">
	import { difficulty } from "../../enums";
	import { game, settings } from "../../stores";

	const handleClick = (event: Event) => {
		$settings.difficulty = difficulty[(event.target as HTMLInputElement).value];
	};
</script>

<div class="flex flex-col justify-between gap-5 text-xl">
	<div class="difficulty flex">
		<span class="flex-shrink">Difficulty:</span>
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

	<div class="colours flex">
		<span class="flex-shrink">Colours:</span>
		<div class=" inline-flex w-full flex-grow justify-center gap-3">
			<input
				class="rounded-md bg-transparent"
				type="color"
				bind:value={$settings.colours.user}
			/>
			<span>│</span>
			<input
				class="rounded-md bg-transparent"
				type="color"
				bind:value={$settings.colours.computer}
			/>
		</div>
	</div>
</div>
