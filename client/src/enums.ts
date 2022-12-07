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

export const mapEnum = (enumObject: any, value: any): number =>
	Object.values(enumObject).indexOf(value);
