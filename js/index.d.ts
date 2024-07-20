declare module '@jprochazk/roll-dice' {
	/**
	 * Evaluates an expression in standard dice notation form
	 *
	 * Options:
	 * * limit - Max number of dice rolls in a single evaluation. Default is 10.
	 * * rng - The random number generator used in dice rolls. Default implementation uses Math.random
	 * * strict - Enable strict mode. Default is `true`.
	 *
	 * If strict mode is enabled, this function throws an error when it fails to parse or evaluate the expression.
	 * If strict mode is disabled, it returns `null` in that case.
	 *
	 * */
	export function roll<Strict extends boolean = true>(input: string, options: {
		limit: number | bigint;
		seed: number | bigint;
		strict?: Strict;
	}): Strict extends true ? bigint : bigint | null;

	export {};
}

//# sourceMappingURL=index.d.ts.map