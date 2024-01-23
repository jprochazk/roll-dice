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
 * In strict mode, this function throws an error when it fails to parse or evaluate the expression.
 * If strict mode is disabled, it returns `null` in that case.
 *
 * @template {boolean} [Strict=true]
 * @param {string} input
 * @param {{ seed: bigint, limit: bigint, strict?: Strict }} options
 * @returns {Strict extends true ? bigint : bigint | null}
 * @throws
 */
function roll(input, { seed, limit, strict }) {
  /** @type {Strict extends true ? bigint : bigint | null} */
  let result;
  try {
    result = /** @type {any} */ (rollDice(input, seed, limit));
  } catch (e) {
    if (strict) {
      throw e;
    } else {
      result = /** @type {any} */ (null);
    }
  }
  return result;
}

module.exports.roll = roll;

