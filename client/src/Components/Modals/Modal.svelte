<script lang="ts">
	// Component Imports
	import Settings from "./Settings.svelte";
	import Leaderboard from "./Leaderboard.svelte";

	import { navigationEvent, type NavigationEvent } from "../../enums";

	import { closeModal } from "svelte-modals";
	import { fade } from "svelte/transition";

	// External Props
	export let isOpen;
	export let event: NavigationEvent;
</script>

{#if isOpen}
	<div
		role="dialog"
		class="w-full h-full fixed flex justify-center items-center pointer-events-none z-50 bg-[#333333BB]"
		transition:fade={{ duration: 150 }}
	>
		<div
			class="relative min-w-[40%] rounded-md p-6 flex flex-col pointer-events-auto bg-white"
		>
			<button
				on:click={closeModal}
				class="flex absolute top-4 right-4 text-gray-400 hover:text-gray-800 transition-colors text-3xl"
				>✗</button
			>
			<h2 class="m-auto text-4xl font-bold">{event}</h2>

			<div class="flex-grow w-full mt-5">
				{#if event === navigationEvent.SETTINGS}
					<Settings />
				{:else if event === navigationEvent.LEADERBOARD}
					<Leaderboard />
				{/if}
			</div>

			<div class="w-full flex-shrink flex mt-5 text-2xl">
				<button
					class="sliding-underline sliding-underline-small m-auto"
					on:click={closeModal}>OK</button
				>
			</div>
		</div>
	</div>
{/if}
