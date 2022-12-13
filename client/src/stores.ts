// Local Imports
import init from "../wasm/pkg/wasm";
import { Game } from "../wasm/pkg/wasm";

// Module Imports
import { writable } from "svelte/store";

// Initialise and export the WASM module
await init();
export const game = writable<Game>(new Game(5, 5));

// globalValues is used for website values that are not part of the game state
interface GlobalValues {
	gameActive: boolean;
}

export const globalValues = writable<GlobalValues>({
	gameActive: false,
});

// Affected Boxes is used to rerender the boxes that have been affected an edge interaction
export const affectedBoxes = writable<number[][]>([]);
