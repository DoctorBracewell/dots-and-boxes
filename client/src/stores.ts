import { get, writable } from "svelte/store";

import init from "../wasm/pkg/wasm";
import { Game } from "../wasm/pkg/wasm";
import { BLUE, RED } from "./constants";
import {
	type Difficulty,
	type LineType,
	type Player,
	difficulty,
	mapEnum,
	player,
	translateNumber,
} from "./enums";

interface Settings {
	difficulty: Difficulty;
	colours: {
		user: string;
		computer: string;
	};
}

/*
 * Initialise and export the WASM module as a store
 */
await init();
export const game = writable<Game>(new Game(5, 5));

interface GameState {
	affectedBoxes: number[][];
	affectedLines: [number, LineType][];
	currentPlayer: Player;
}

/**
 * Global game state
 */
export const gameState = writable<GameState>({
	affectedBoxes: [],
	affectedLines: [],
	currentPlayer: player.USER,
});

/**
 * Global game settings
 */
export const settings = writable<Settings>({
	/**
	 * Get difficulty from WASM module
	 */
	get difficulty() {
		return translateNumber(difficulty, get(game).difficulty);
	},

	/**
	 * Save difficulty to WASM module
	 */
	set difficulty(diff: Difficulty) {
		get(game).difficulty = mapEnum(difficulty, diff);
	},

	colours: {
		user: BLUE,
		computer: RED,
	},
});

/*
 * Initialise settings with default difficulty
 */
const settingsReference = get(settings);
settingsReference.difficulty = difficulty.MEDIUM;
