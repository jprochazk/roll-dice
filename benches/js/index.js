import { roll as RollDice } from "roll-dice";
import DiceRollEval from "dice-roll-eval";
import b from "benny";

const rng = (min, max) => min + (max - min) / 2;
const seed = BigInt(1234);
const limit = BigInt(2 ** 32 - 1);
const options = { limit, rng, strict: true };

b.suite(
  "dice roll (1d20)",
  b.add("roll-dice", function () {
    RollDice("1d20", seed, limit);
  }),
  b.add("dice-roll-eval", function () {
    DiceRollEval("1d20", options);
  }),
  b.cycle(),
  b.complete()
);

b.suite(
  "dice roll (big expr)",
  b.add("roll-dice", function () {
    RollDice(
      "10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/10)))))))))))))))))))",
      seed,
      limit
    );
  }),
  b.add("dice-roll-eval", function () {
    DiceRollEval(
      "10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/10)))))))))))))))))))",
      options
    );
  }),
  b.cycle(),
  b.complete()
);

