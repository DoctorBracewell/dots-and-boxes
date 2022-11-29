<script lang="ts">
	export let average: number;
</script>

<div
	class="flex"
	style="
		width: {average}em;
		height: {average}em;
	"
>
	<div class="w-full h-4 opacity-0 hover:opacity-100 cursor-pointer z-40 -mt-2">
		<img src="assets/imgs/horizontal-line-hover.svg" alt="" class="h-full" />
	</div>
</div>
