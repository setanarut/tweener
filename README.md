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

![Easing fonksiyonları](https://github-production-user-asset-6210df.s3.amazonaws.com/36481442/708401534-c3f6140c-862f-49c2-8084-f529099b0409.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20261117%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20261117T000939Z&X-Amz-Expires=300&X-Amz-Signature=e0335fe8c1f15577f1bda2d7df31a00268e05cfde53f150dc78542969c3d8f7b&X-Amz-SignedHeaders=host&response-content-type=image%2Fpng)

See [easings.net](https://easings.net/) for online visualization.
