export const difficulty = {
	EASY: "easy",
	MEDIUM: "medium",
	HARD: "hard",
} as const;

export type Difficulty = typeof difficulty[keyof typeof difficulty];

export const mapEnum = (enumObject: any, value: any): number =>
	Object.values(enumObject).indexOf(value);
