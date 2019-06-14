# [almost](https://crates.io/crates/almost): A crate for comparing floating point numbers.

[![Docs](https://docs.rs/almost/badge.svg)](https://docs.rs/almost)

This crate strives to make choices that are good for most cases, and not
providing you a complex API that encourages misuse.

1. Uses relative comparison by default.
2. Provides a separate function for comparison with zero (the only time when
   absolute comparison is a good default choice).
3. Uses a better default for tolerance than `std::{f32,f64}::EPSILON`.
4. Handles infinities / subnormals properly.
5. `no_std` compatible always

# License
Public domain, as explained [here](https://creativecommons.org/publicdomain/zero/1.0/legalcode)
