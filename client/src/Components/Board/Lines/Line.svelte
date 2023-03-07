<script lang="ts">
	import chunk from "lodash/chunk";
	import { createEventDispatcher } from "svelte";

	import Horizontal from "./Horizontal.svelte";
	import Hover from "./Hover.svelte";
	import Vertical from "./Vertical.svelte";

	import {
		type Claimed,
		type LineType,
		claimed,
		lineType as lineTypeEnum,
		mapEnum,
		player,
		translateClaimed,
		translateNumber,
	} from "../../../enums";
	import { game, gameState, settings } from "../../../stores";

	// External Props
	export let average: number;
	export let lineType: LineType;
	export let index: number;

	// Local Variables
	const dispatch = createEventDispatcher();

	let lineClaimed: Claimed;
	let affected = false;

	// Rerender this line whenever the store updates
	const updateClaimed = () =>
		(lineClaimed = translateClaimed(
			$game.get_edge(index, mapEnum(lineTypeEnum, lineType))
		));

	// Receive affected boxes information from WASM logic and trigger a UI update
	const updateAffectedBoxes = (affected_boxes: Uint32Array) => {
		// Calculate the boxes affected by the interaction
		const chunked_affected_boxes: number[][] = chunk(affected_boxes, 2);
		const chunked_claimed_boxes = chunked_affected_boxes.filter(
			([y, x]) => translateClaimed($game.get_box(x, y)) !== claimed.EMPTY
		);

		// Save the affected boxes into the store
		$gameState.affectedBoxes = chunked_claimed_boxes;
		$gameState.currentPlayer = translateNumber(
			player,
			$game.current_player
		);

		// Update the UI for this line
		updateClaimed();
	};

	const handleClick = async () => {
		if ($game.current_player === mapEnum(player, player.COMPUTER)) return;
		if (lineClaimed !== claimed.EMPTY) return;

		/* User Turn */
		updateAffectedBoxes(
			$game.handle_edge_interact(index, mapEnum(lineTypeEnum, lineType))
		);

		/* Check for game over */
		if ($game.board_full()) {
			return dispatch("gameend");
		}

		/* Computer Turn
		 * Ideally the computer turn logic would be in the WASM logic
		 * but WASM cannot block the thread to "sleep" for a second
		 * so the turn-taking and second-turn logic is here instead
		 */
		while ($game.current_player === mapEnum(player, player.COMPUTER)) {
			// Sleep for a second before the computer makes a move
			await new Promise((resolve) => setTimeout(resolve, 500));

			// Let the computer take a move and retrieve information to update the UI
			const {
				0: indexToUpdate,
				1: line,
				affected_boxes,
			} = $game.computer_turn();

			const lineTypeToUpdate = translateNumber(lineTypeEnum, line);

			// Update affected boxes
			updateAffectedBoxes(affected_boxes);

			// Update the line
			$gameState.affectedLines = [
				[
					lineTypeToUpdate === lineTypeEnum.HORIZONTAL
						? indexToUpdate
						: indexToUpdate - $game.width * ($game.height + 1),
					translateNumber(lineTypeEnum, lineTypeToUpdate),
				],
			];

			if ($game.board_full()) {
				return dispatch("gameend");
			}
		}
	};

	// Update the line if the store updates (triggered by a computer turn)
	$: {
		affected = $gameState.affectedLines.some(
			(line) => index === line[0] && lineType === line[1]
		);

		// Update the UI for this line
		updateClaimed();
	}

	// Reactively update the colour when settings change
	let colour = "";

	$: {
		const colourMap = {
			[claimed.USER]: $settings.colours.user,
			[claimed.COMPUTER]: $settings.colours.computer,
		};

		colour = colourMap[lineClaimed];
	}
</script>

{#key affected}
	<div
		class="line flex bg-transparent"
		title="line"
		style="
		width: {average}em;
		height: {average}em;
	"
		on:click={handleClick}
		on:keypress={handleClick}
	>
		{#if lineClaimed === claimed.EMPTY}
			<Hover {lineType} />
		{:else if lineType === lineTypeEnum.HORIZONTAL}
			<div class="z-40 -mt-2 h-4 w-full">
				<Horizontal stroke={colour} />
			</div>
		{:else if lineType === lineTypeEnum.VERTICAL}
			<div class="z-40 -ml-4 h-full w-8">
				<Vertical stroke={colour} />
			</div>
		{/if}
	</div>
{/key}
