// Creates an iterator that counts up between two numbers
export function* range(start: number, end: number): Generator<number> {
	for (let i = start; i < end; i++) {
		yield i;
	}
}
