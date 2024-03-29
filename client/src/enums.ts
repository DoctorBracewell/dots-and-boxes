/*
 * Direct mappings of rust enums to typescript objects and types
 */

export const difficulty = {
	EASY: "easy",
	MEDIUM: "medium",
	HARD: "hard",
} as const;

export type Difficulty = (typeof difficulty)[keyof typeof difficulty];

export const lineType = {
	HORIZONTAL: "horizontal",
	VERTICAL: "vertical",
} as const;

export type LineType = (typeof lineType)[keyof typeof lineType];

export const player = {
	USER: "user",
	COMPUTER: "computer",
} as const;

export type Player = (typeof player)[keyof typeof player];

export const claimed = {
	USER: "user",
	COMPUTER: "computer",
	EMPTY: "hover",
} as const;

export type Claimed = (typeof claimed)[keyof typeof claimed];

export const navigationEvent = {
	SETTINGS: "settings",
	LEADERBOARD: "leaderboard",
} as const;

export type NavigationEvent =
	(typeof navigationEvent)[keyof typeof navigationEvent];

/**
 * Translate rust claimed option to typescript enum
 */
export const translateClaimed = (option: number | undefined) => {
	if (option === undefined) {
		return claimed.EMPTY;
	}

	return Object.values(claimed)[option];
};

/**
 * Translate rust enum (sent as number via WASM) to typescript enum value
 */
export const translateNumber = <T>(enumObject: T, index: number): T[keyof T] =>
	Object.values(enumObject)[index];

/**
 * Translate typescript enum value to rust enum (sent as number via WASM)
 */
export const mapEnum = (
	enumObject: {
		[key: string]: string;
	},
	value: string
): number => Object.values(enumObject).indexOf(value);
