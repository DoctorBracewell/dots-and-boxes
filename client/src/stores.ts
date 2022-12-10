import { writable } from "svelte/store";

import init from "../wasm/pkg/wasm";
import { Game } from "../wasm/pkg/wasm";

await init();

export const game = writable<Game>(new Game(5, 5));

interface GlobalValues {
	gameActive: boolean;
}

export const globalValues = writable<GlobalValues>({
	gameActive: false,
});

export const updatedBoxes = writable<number[][]>([]);
