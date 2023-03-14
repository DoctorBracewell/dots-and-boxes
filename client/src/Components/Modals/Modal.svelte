<script lang="ts">
	import { closeModal } from "svelte-modals";
	import { fade } from "svelte/transition";

	import Leaderboard from "./Leaderboard.svelte";
	import Settings from "./Settings.svelte";

	import { type NavigationEvent, navigationEvent } from "../../enums";

	// External Props
	export let isOpen;
	export let event: NavigationEvent;
</script>

{#if isOpen}
	<div
		role="dialog"
		class="pointer-events-none fixed z-50 flex h-full w-full items-center justify-center bg-[#333333BB]"
		transition:fade={{ duration: 150 }}
	>
		<div
			class="pointer-events-auto relative flex min-h-fit min-w-[40%] flex-col rounded-md bg-white p-6"
		>
			<button
				on:click={closeModal}
				class="absolute right-4 top-4 flex text-3xl text-gray-400 transition-colors hover:text-gray-800"
				>✗</button
			>
			<h2 class="m-auto text-4xl font-bold">{event}</h2>

			<div class="mt-5 w-full flex-grow">
				{#if event === navigationEvent.SETTINGS}
					<Settings />
				{:else if event === navigationEvent.LEADERBOARD}
					<Leaderboard />
				{/if}
			</div>

			<div class="mt-5 flex w-full flex-shrink text-2xl">
				<button
					class="sliding-underline sliding-underline-small m-auto"
					on:click={closeModal}>OK</button
				>
			</div>
		</div>
	</div>
{/if}
