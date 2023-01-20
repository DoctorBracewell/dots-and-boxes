<script lang="ts">
	import { difficulty } from "../../enums";
	import { settings, game } from "../../stores";

	const handleClick = (event: Event) => {
		$settings.difficulty = difficulty[(event.target as HTMLInputElement).value];
	};
</script>

<div class="flex flex-col justify-between gap-5">
	<div class="difficulty flex">
		<span class="flex-shrink">Difficulty:</span>
		<div class="inline-flex flex-grow gap-3 w-full justify-center">
			{#each Object.entries(difficulty) as [key, value], index}
				<div class="inline-block">
					<input
						on:click={handleClick}
						type="radio"
						name="difficulty"
						checked={$settings.difficulty === value}
						value={key}
						id={key}
						class="hidden peer"
					/>
					<label
						for={key}
						class="transition-colors cursor-pointer sliding-underline sliding-underline-small peer-checked:after:w-0"
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
		<div class="inline-flex flex-grow gap-3 w-full justify-center">
			<input
				class="bg-transparent rounded-md"
				type="color"
				bind:value={$settings.colours.user}
			/>
			<span>│</span>
			<input
				class="bg-transparent rounded-md"
				type="color"
				bind:value={$settings.colours.computer}
			/>
		</div>
	</div>
</div>
