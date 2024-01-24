// @ts-check

const { roll: rollDice } = require("./wasm/roll_dice.js");

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
 * @template {boolean} [Strict=true]
 * @param {string} input
 * @param {{
 *   limit: number | bigint,
 *   seed: number | bigint,
 *   strict?: Strict
 * }} options
 * @returns {Strict extends true ? bigint : bigint | null}
 * @throws
 */
function roll(input, options) {
  /** @type {Strict extends true ? bigint : bigint | null} */
  let result;
  try {
    // @ts-expect-error: cant assign to a conditional type
    result = rollDice(input, BigInt(options.seed), BigInt(options.limit));
  } catch (e) {
    if (options.strict) {
      throw e;
    } else {
      // @ts-expect-error: cant assign to a conditional type
      result = null;
    }
  }
  return result;
}

module.exports.roll = roll;

