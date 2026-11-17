[![Crates.io](https://img.shields.io/crates/v/tweener.svg)](https://crates.io/crates/tweener)
[![Docs](https://docs.rs/tweener/badge.svg)](https://docs.rs/tweener/latest/)
[![License](https://img.shields.io/crates/l/tweener.svg)](https://crates.io/crates/tweener)
[![Downloads](https://img.shields.io/crates/d/tweener.svg)](https://crates.io/crates/tweener)

`tweener` is a one-dimensional tweening library.

## Features

- 41 easing functions
- Optional `serde` feature enables serialization and deserialization of the `Tween` struct (useful for games). Enable it in `Cargo.toml` with `features = ["serde"]`.
- Yoyo animation feature
- Animation durations use the `std::time::Duration` type (easy-to-read code).
- Generic over `f32` and `f64` (via `num-traits`), defaults to `f64`

## Quick start

```rust
use std::time::Duration;
use tweener::{Easing, Tween};

fn main() {

    // Tween from 0 to 1 in 3 seconds.
    let mut tw = Tween::new(0.0, 1.0, Duration::from_secs(3), Easing::Linear, false);

    // Advance by 1.5 seconds.
    tw.update(Duration::from_millis(1500));

    // Get tween value at 1.5 seconds.
    println!("{}", tw.value); // 0.5

    // Is the animation finished?
    println!("{}", tw.is_finished()); // false
}

```

## Easing functions

| family      | in        | out        | inOut        | outIn        |
| ----------- | --------- | ---------- | ------------ | ------------ |
| **Linear**  | Linear    | Linear     | Linear       | Linear       |
| **Quad**    | InQuad    | OutQuad    | InOutQuad    | OutInQuad    |
| **Cubic**   | InCubic   | OutCubic   | InOutCubic   | OutInCubic   |
| **Quart**   | InQuart   | OutQuart   | InOutQuart   | OutInQuart   |
| **Quint**   | InQuint   | OutQuint   | InOutQuint   | OutInQuint   |
| **Expo**    | InExpo    | OutExpo    | InOutExpo    | OutInExpo    |
| **Sine**    | InSine    | OutSine    | InOutSine    | OutInSine    |
| **Circ**    | InCirc    | OutCirc    | InOutCirc    | OutInCirc    |
| **Back**    | InBack    | OutBack    | InOutBack    | OutInBack    |
| **Bounce**  | InBounce  | OutBounce  | InOutBounce  | OutInBounce  |
| **Elastic** | InElastic | OutElastic | InOutElastic | OutInElastic |

See [easings.net](https://easings.net/) for online visualization.
