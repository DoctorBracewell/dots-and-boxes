<script lang="ts">
	export let label: string;

	import { createEventDispatcher } from "svelte";
	const dispatch = createEventDispatcher();

	const handleClick = () => {
		dispatch("navigationClick", {
			label,
		});
	};
</script>

<button
	on:click={handleClick}
	class="bg-none p-0 m-0 border-none sliding-underline">{label}</button
>
