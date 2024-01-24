// @ts-check

import { roll as RollDice } from "@jprochazk/roll-dice";
import DiceRollEval from "dice-roll-eval";
import b from "benny";

const avg = (min, max) => min + (max - min) / 2;

b.suite(
  "dice roll (big)",
  b.add("dice-roll-eval", function () {
    DiceRollEval(
      "10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/10)))))))))))))))))))",
      { limit: Number(2 ** 32 - 1), rng: avg, strict: true }
    );
  }),
  b.add("roll-dice", function () {
    RollDice(
      "10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/(10*(10/10)))))))))))))))))))",
      { seed: 1234n, limit: BigInt(2 ** 32 - 1) }
    );
  }),
  b.cycle(),
  b.complete()
);

b.suite(
  "dice roll (1d20)",
  b.add("dice-roll-eval", function () {
    DiceRollEval("1d20", { limit: Number(2 ** 32 - 1), rng: avg, strict: true });
  }),
  b.add("roll-dice", function () {
    RollDice("1d20", { seed: 1234n, limit: BigInt(2 ** 32 - 1), strict: true });
  }),
  b.cycle(),
  b.complete()
);

