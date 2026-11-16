use std::time::Duration;

use tweener::{Easing, Tween};

fn main() {
    let mut tw = Tween::new(0.0, 90.0, Duration::from_secs(20), Easing::InBounce, false);

    // Your game loop
    for _ in 0..20 {
        let n = tw.value as usize;
        for _ in 0..n {
            print!("x");
        }
        for _ in n..tw.end as usize {
            print!(" ");
        }
        print!("\n");

        // Advance
        tw.update(Duration::from_secs(1));
    }
}

// output:
/*
    x
    x
    xxxx
    xxxxx
    xx
    xxxxxx
    xxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxx
    xxxxxxxx
    xxxxxx
    xxxxxxxxxxxxxxxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
*/
