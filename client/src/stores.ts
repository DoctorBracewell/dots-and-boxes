// Local Imports
import init from "../wasm/pkg/wasm";
import { Game } from "../wasm/pkg/wasm";
import { difficulty, mapEnum, translateNumber, type Difficulty } from "./enums";
import { RED, BLUE } from "./constants";

// Module Imports
import { writable, get } from "svelte/store";

// Initialise and export the WASM module
await init();
export const game = writable<Game>(new Game(5, 5));

// Affected Boxes is used to rerender the boxes that have been affected an edge interaction
export const affectedBoxes = writable<number[][]>([]);

// Extract settings data from local storage and save into writable store
interface Settings {
	difficulty: Difficulty;
	colours: {
		user: string;
		computer: string;
	};
}

export const settings = writable<Settings>({
	get difficulty() {
		return translateNumber(difficulty, get(game).difficulty);
	},

	set difficulty(diff: Difficulty) {
		get(game).difficulty = mapEnum(difficulty, diff);
	},

	colours: {
		user: RED,
		computer: BLUE,
	},
});

// Initialise the settings
const settingsReference = get(settings);
settingsReference.difficulty = difficulty.MEDIUM;
