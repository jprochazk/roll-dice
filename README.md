# roll-dice

Evaluate standard dice roll notation.

```js
import { roll } from "roll-dice";

roll("5d6") // rolls a 6-sided die, 5 times
roll("5d6 + 10") // rolls a 6-sided die, 5 times, and adds 10 to the result.
roll("5d6 / 2") // rolls a 6-sided die, 5 times, and divides the result by 2.
roll("(10*10)d(10*10)") // rolls a (10*10)-sided die, (10*10) times.
```

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
