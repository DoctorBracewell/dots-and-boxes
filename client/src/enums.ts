// Enum Objects
export const difficulty = {
	EASY: "easy",
	MEDIUM: "medium",
	HARD: "hard",
} as const;

export type Difficulty = typeof difficulty[keyof typeof difficulty];

export const lineType = {
	HORIZONTAL: "horizontal",
	VERTICAL: "vertical",
} as const;

export type LineType = typeof lineType[keyof typeof lineType];

export const player = {
	USER: "user",
	COMPUTER: "computer",
} as const;

export type Player = typeof player[keyof typeof player];

export const claimed = {
	USER: "user",
	COMPUTER: "computer",
	EMPTY: "hover",
} as const;

export type Claimed = typeof claimed[keyof typeof claimed];

export const navigationEvent = {
	SETTINGS: "settings",
	LEADERBOARD: "leaderboard",
} as const;

export type NavigationEvent =
	typeof navigationEvent[keyof typeof navigationEvent];

// Helper Functions
export const translateClaimed = (option: number | undefined) => {
	if (option === undefined) {
		return claimed.EMPTY;
	}

	return Object.values(claimed)[option];
};

export const translateNumber = <T>(enumObject: T, index: any): T[keyof T] =>
	Object.values(enumObject)[index];

export const mapEnum = (enumObject: any, value: any): number =>
	Object.values(enumObject).indexOf(value);
