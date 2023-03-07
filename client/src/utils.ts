/**
 * Creates an iterator that counts up between two numbers
 */
export function* range(
	start: number,
	end: number,
	step = 1
): Generator<number> {
	for (let i = start; i < end; i += step) {
		yield i;
	}
}

/**
 * Validate a username against an array of predicates
 */
export function checkUsername(username: string) {
	const checks = [
		{ check: username.length >= 1, string: "Username too short" },
		{ check: username.length <= 20, string: "Username too long" },
		{
			check: /^[a-zA-Z0-9_]+$/.test(username),
			string: "Username uses invalid characters",
		},
	];

	return checks.filter((check) => !check.check);
}
