use std::time::Duration;

use tweener::*;

fn main() {
    // Tweener from 0 to 1 in 3 seconds.
    let mut tw = Tween::new(0.0, 1.0, Duration::from_secs(3), Easing::Linear, false);

    // Advance by 1.5 seconds.
    tw.update(Duration::from_millis(1500));

    // Get tween value at 1.5 seconds.
    println!("{}", tw.value); // 0.5

    // Is the animation finished?
    println!("{}", tw.is_finished()); // false
}
