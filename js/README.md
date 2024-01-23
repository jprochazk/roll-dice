# roll-dice

Evaluate standard dice roll notation.

```
$ npm i roll-dice@npm:jprochazk/roll-dice
```

```js
import { roll } from "roll-dice";

roll("5d6") // roll a 6-sided die 5 times.
roll("5d6 + 10") // roll a 6-sided die 5 times, and add 10 to the result.
```

## Syntax

|         | description                                     | example |
|---------|-------------------------------------------------|---------|
| `a + b` | add `a` and `b`                                 | `1 + 1` |
| `a - b` | subtract `b` from `a`                           | `1 - 1` |
| `a * b` | multiply `a` by `b`                             | `1 * 1` |
| `a / b` | divide `a` by `b`                               | `1 / 1` |
| `- b`   | negates `b`                                     | `-1`    |
| `d a`   | rolls an `a`-sided die                          | `d5`    |
| `a d b` | rolls a `b`-sided die, `a` times                | `2d10`  |
| `( a )` | grouping expression, used to specify precedence | `(1+1)` |

Every `a` or `b` in the above table may contain another expression, and expressions may be arbitrarily nested. For example `(10+5)*2+(5d10)` is a valid expression.

## Structure

- [`lib.rs`](./src/lib.rs) - entry point for the WASM module
- [`parse.rs`](./src/parse.rs) - a [recursive descent parser](https://en.wikipedia.org/wiki/Recursive_descent_parser) which outputs a list of instructions in [postfix notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation).
- [`eval.rs`](./src/eval.rs)

The library is written in [Rust](https://www.rust-lang.org/), and uses [wasm-pack](https://github.com/rustwasm/wasm-pack) for packing it as a WASM module and publishing it on NPM. Various commands (available in the [justfile](./justfile)) are run using [`just`](https://github.com/casey/just).

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
