[![Docs](https://docs.rs/tweener/badge.svg)](https://docs.rs/tweener/latest/)

`tweener` is a one-dimensional tweening library.

## Features

- 41 easing functions
- Optional `serde` feature enables serialization and deserialization of the `Tween` struct (useful for games). Enable it in `Cargo.toml` with `features = ["serde"]`.
- Yoyo animation feature
- Animation durations use the `std::time::Duration` type (easy-to-read code).
- The `f32` submodule can be used for the f32 `Tween` type. `use tweener::f32::*;`

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
