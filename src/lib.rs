#![doc = include_str!("../README.md")]

/// `f32` version of the [`tweener`](crate) crate.
pub mod f32;

use std::time::Duration;

/// Interpolates a value from `begin` to `end` over `duration` using the easing function `func`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tween {
    /// Starting value of the tween.
    pub begin: f64,
    /// Target value of the tween.
    pub end: f64,
    /// Total time the tween takes to go from `begin` to `end`.
    pub duration: Duration,
    /// Easing function that shapes the progress curve.
    pub func: Easing,
    /// When `true`, the tween plays back in the opposite direction on reaching the end instead of finishing.
    pub yoyo: bool,
    /// Current interpolated value.
    pub value: f64,
    /// Time elapsed so far in the current run.
    pub time: Duration,
    /// Time left over after `time` passes `duration` in the last update, carried into the next cycle or step.
    pub overflow: Duration,
    /// When `true`, the tween is currently playing backwards, from `end` to `begin`.
    pub reversed: bool,
}

impl Tween {
    /// Creates a new tween from `begin` to `end` over `duration` using the easing function `func`.
    pub fn new(begin: f64, end: f64, duration: Duration, func: Easing, yoyo: bool) -> Self {
        Self {
            begin,
            end,
            duration,
            func,
            yoyo,
            value: begin,
            time: Duration::ZERO,
            overflow: Duration::ZERO,
            reversed: false,
        }
    }

    /// Sets the elapsed time directly, clamping to `duration` and updating `value` and `overflow` accordingly.
    pub fn set_time(&mut self, current: Duration) {
        if current >= self.duration {
            self.overflow = current - self.duration;
            self.time = self.duration;
            self.value = self.end;
        } else if current.is_zero() {
            self.overflow = Duration::ZERO;
            self.time = Duration::ZERO;
            self.value = self.begin;
        } else {
            self.overflow = Duration::ZERO;
            self.time = current;
            self.value = self.func.apply(
                self.time.as_secs_f64(),
                self.begin,
                self.change(),
                self.duration.as_secs_f64(),
            );
        }
    }

    /// Returns the total distance between `begin` and `end`.
    pub fn change(&self) -> f64 {
        self.end - self.begin
    }

    /// Returns `true` once the tween has reached its end point in the current playback direction.
    pub fn is_finished(&self) -> bool {
        if self.reversed {
            self.time.is_zero()
        } else {
            self.time >= self.duration
        }
    }

    /// Advances the tween by `dt` in the current direction, flipping direction and carrying over the remainder when `yoyo` is enabled.
    pub fn update(&mut self, dt: Duration) {
        let prev = self.time;
        if self.reversed {
            match prev.checked_sub(dt) {
                Some(t) => self.set_time(t),
                None => {
                    self.set_time(Duration::ZERO);
                    self.overflow = dt - prev;
                }
            }
        } else {
            self.set_time(prev + dt);
        }

        while self.yoyo && !self.duration.is_zero() && self.is_finished() {
            let over = self.overflow;
            self.reversed = !self.reversed;
            if self.reversed {
                if over <= self.duration {
                    self.set_time(self.duration - over);
                } else {
                    self.set_time(Duration::ZERO);
                    self.overflow = over - self.duration;
                }
            } else {
                self.set_time(over);
            }
        }
    }
}

// Eeasing functions

use std::f64::consts::PI;
const BACK_S: f64 = 1.70158;

/// The enum that holds the easing methods allows the [`Tween`] struct to be serialized.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Easing {
    Linear,
    InQuad,
    OutQuad,
    InOutQuad,
    OutInQuad,
    InCubic,
    OutCubic,
    InOutCubic,
    OutInCubic,
    InQuart,
    OutQuart,
    InOutQuart,
    OutInQuart,
    InQuint,
    OutQuint,
    InOutQuint,
    OutInQuint,
    InSine,
    OutSine,
    InOutSine,
    OutInSine,
    InExpo,
    OutExpo,
    InOutExpo,
    OutInExpo,
    InCirc,
    OutCirc,
    InOutCirc,
    OutInCirc,
    InElastic,
    OutElastic,
    InOutElastic,
    OutInElastic,
    InBack,
    OutBack,
    InOutBack,
    OutInBack,
    InBounce,
    OutBounce,
    InOutBounce,
    OutInBounce,
}

impl Easing {
    pub(crate) fn apply(self, t: f64, b: f64, c: f64, d: f64) -> f64 {
        use Easing::*;
        match self {
            Linear => linear(t, b, c, d),
            InQuad => in_quad(t, b, c, d),
            OutQuad => out_quad(t, b, c, d),
            InOutQuad => in_out_quad(t, b, c, d),
            OutInQuad => out_in_quad(t, b, c, d),
            InCubic => in_cubic(t, b, c, d),
            OutCubic => out_cubic(t, b, c, d),
            InOutCubic => in_out_cubic(t, b, c, d),
            OutInCubic => out_in_cubic(t, b, c, d),
            InQuart => in_quart(t, b, c, d),
            OutQuart => out_quart(t, b, c, d),
            InOutQuart => in_out_quart(t, b, c, d),
            OutInQuart => out_in_quart(t, b, c, d),
            InQuint => in_quint(t, b, c, d),
            OutQuint => out_quint(t, b, c, d),
            InOutQuint => in_out_quint(t, b, c, d),
            OutInQuint => out_in_quint(t, b, c, d),
            InSine => in_sine(t, b, c, d),
            OutSine => out_sine(t, b, c, d),
            InOutSine => in_out_sine(t, b, c, d),
            OutInSine => out_in_sine(t, b, c, d),
            InExpo => in_expo(t, b, c, d),
            OutExpo => out_expo(t, b, c, d),
            InOutExpo => in_out_expo(t, b, c, d),
            OutInExpo => out_in_expo(t, b, c, d),
            InCirc => in_circ(t, b, c, d),
            OutCirc => out_circ(t, b, c, d),
            InOutCirc => in_out_circ(t, b, c, d),
            OutInCirc => out_in_circ(t, b, c, d),
            InElastic => in_elastic(t, b, c, d),
            OutElastic => out_elastic(t, b, c, d),
            InOutElastic => in_out_elastic(t, b, c, d),
            OutInElastic => out_in_elastic(t, b, c, d),
            InBack => in_back(t, b, c, d),
            OutBack => out_back(t, b, c, d),
            InOutBack => in_out_back(t, b, c, d),
            OutInBack => out_in_back(t, b, c, d),
            InBounce => in_bounce(t, b, c, d),
            OutBounce => out_bounce(t, b, c, d),
            InOutBounce => in_out_bounce(t, b, c, d),
            OutInBounce => out_in_bounce(t, b, c, d),
        }
    }
}

fn linear(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * t / d + b
}

fn in_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * (t / d).powi(2) + b
}

fn out_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d;
    -c * t * (t - 2.0) + b
}

fn in_out_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d * 2.0;
    if t < 1.0 {
        return c / 2.0 * t.powi(2) + b;
    }
    -c / 2.0 * ((t - 1.0) * (t - 3.0) - 1.0) + b
}

fn out_in_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return out_quad(t * 2.0, b, c / 2.0, d);
    }
    in_quad((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn in_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * (t / d).powi(3) + b
}

fn out_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * ((t / d - 1.0).powi(3) + 1.0) + b
}

fn in_out_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let mut t = t / d * 2.0;
    if t < 1.0 {
        return c / 2.0 * t * t * t + b;
    }
    t -= 2.0;
    c / 2.0 * (t * t * t + 2.0) + b
}

fn out_in_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return out_cubic(t * 2.0, b, c / 2.0, d);
    }
    in_cubic((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn in_quart(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * (t / d).powi(4) + b
}

fn out_quart(t: f64, b: f64, c: f64, d: f64) -> f64 {
    -c * ((t / d - 1.0).powi(4) - 1.0) + b
}

fn in_out_quart(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d * 2.0;
    if t < 1.0 {
        return c / 2.0 * t.powi(4) + b;
    }
    -c / 2.0 * ((t - 2.0).powi(4) - 2.0) + b
}

fn out_in_quart(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return out_quart(t * 2.0, b, c / 2.0, d);
    }
    in_quart((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn in_quint(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * (t / d).powi(5) + b
}

fn out_quint(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * ((t / d - 1.0).powi(5) + 1.0) + b
}

fn in_out_quint(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d * 2.0;
    if t < 1.0 {
        return c / 2.0 * t.powi(5) + b;
    }
    c / 2.0 * ((t - 2.0).powi(5) + 2.0) + b
}

fn out_in_quint(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return out_quint(t * 2.0, b, c / 2.0, d);
    }
    in_quint((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn in_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    -c * (t / d * (PI / 2.0)).cos() + c + b
}

fn out_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * (t / d * (PI / 2.0)).sin() + b
}

fn in_out_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    -c / 2.0 * ((PI * t / d).cos() - 1.0) + b
}

fn out_in_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return out_sine(t * 2.0, b, c / 2.0, d);
    }
    in_sine((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn in_expo(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t == 0.0 {
        return b;
    }
    c * 2.0_f64.powf(10.0 * (t / d - 1.0)) + b - c * 0.001
}

fn out_expo(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t == d {
        return b + c;
    }
    c * 1.001 * (-(2.0_f64.powf(-10.0 * t / d)) + 1.0) + b
}

fn in_out_expo(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t == 0.0 {
        return b;
    }
    if t == d {
        return b + c;
    }
    let t = t / d * 2.0;
    if t < 1.0 {
        return c / 2.0 * 2.0_f64.powf(10.0 * (t - 1.0)) + b - c * 0.0005;
    }
    c / 2.0 * 1.0005 * (-(2.0_f64.powf(-10.0 * (t - 1.0))) + 2.0) + b
}

fn out_in_expo(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return out_expo(t * 2.0, b, c / 2.0, d);
    }
    in_expo((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn in_circ(t: f64, b: f64, c: f64, d: f64) -> f64 {
    -c * ((1.0 - (t / d).powi(2)).sqrt() - 1.0) + b
}

fn out_circ(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * (1.0 - (t / d - 1.0).powi(2)).sqrt() + b
}

fn in_out_circ(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let mut t = t / d * 2.0;
    if t < 1.0 {
        return -c / 2.0 * ((1.0 - t * t).sqrt() - 1.0) + b;
    }
    t -= 2.0;
    c / 2.0 * ((1.0 - t * t).sqrt() + 1.0) + b
}

fn out_in_circ(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return out_circ(t * 2.0, b, c / 2.0, d);
    }
    in_circ((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn in_elastic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t == 0.0 {
        return b;
    }
    let mut t = t / d;
    if t == 1.0 {
        return b + c;
    }
    let (p, a, s) = calculate_pas(c, d);
    t -= 1.0;
    -(a * 2.0_f64.powf(10.0 * t) * ((t * d - s) * (2.0 * PI) / p).sin()) + b
}

fn out_elastic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t == 0.0 {
        return b;
    }
    let t = t / d;
    if t == 1.0 {
        return b + c;
    }
    let (p, a, s) = calculate_pas(c, d);
    a * 2.0_f64.powf(-10.0 * t) * ((t * d - s) * (2.0 * PI) / p).sin() + c + b
}

fn in_out_elastic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t == 0.0 {
        return b;
    }
    let mut t = t / d * 2.0;
    if t == 2.0 {
        return b + c;
    }
    let (p, a, s) = calculate_pas(c, d);
    t -= 1.0;
    if t < 0.0 {
        return -0.5 * (a * 2.0_f64.powf(10.0 * t) * ((t * d - s) * (2.0 * PI) / p).sin()) + b;
    }
    a * 2.0_f64.powf(-10.0 * t) * ((t * d - s) * (2.0 * PI) / p).sin() * 0.5 + c + b
}

fn out_in_elastic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return out_elastic(t * 2.0, b, c / 2.0, d);
    }
    in_elastic((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn in_back(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d;
    c * t * t * ((BACK_S + 1.0) * t - BACK_S) + b
}

fn out_back(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d - 1.0;
    c * (t * t * ((BACK_S + 1.0) * t + BACK_S) + 1.0) + b
}

fn in_out_back(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let s = BACK_S * 1.525;
    let mut t = t / d * 2.0;
    if t < 1.0 {
        return c / 2.0 * (t * t * ((s + 1.0) * t - s)) + b;
    }
    t -= 2.0;
    c / 2.0 * (t * t * ((s + 1.0) * t + s) + 2.0) + b
}

fn out_in_back(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < (d / 2.0) {
        return out_back(t * 2.0, b, c / 2.0, d);
    }
    in_back((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn out_bounce(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let mut t = t / d;
    if t < 1.0 / 2.75 {
        return c * (7.5625 * t * t) + b;
    }
    if t < 2.0 / 2.75 {
        t -= 1.5 / 2.75;
        return c * (7.5625 * t * t + 0.75) + b;
    } else if t < 2.5 / 2.75 {
        t -= 2.25 / 2.75;
        return c * (7.5625 * t * t + 0.9375) + b;
    }
    t -= 2.625 / 2.75;
    c * (7.5625 * t * t + 0.984375) + b
}

fn in_bounce(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c - out_bounce(d - t, 0.0, c, d) + b
}

fn in_out_bounce(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return in_bounce(t * 2.0, 0.0, c, d) * 0.5 + b;
    }
    out_bounce(t * 2.0 - d, 0.0, c, d) * 0.5 + c * 0.5 + b
}

fn out_in_bounce(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        return out_bounce(t * 2.0, b, c / 2.0, d);
    }
    in_bounce((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
}

fn calculate_pas(c: f64, d: f64) -> (f64, f64, f64) {
    let p = d * 0.3;
    (p, c, p / 4.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn new_and_change() {
        let tween = Tween::new(2.0, 8.0, Duration::from_secs(10), Easing::Linear, false);
        assert_eq!(tween.value, 2.0);
        assert_eq!(tween.change(), 6.0);
        assert!(!tween.is_finished());
    }

    #[test]
    fn set_time_updates_value_and_overflow() {
        let mut tween = Tween::new(0.0, 10.0, Duration::from_secs(10), Easing::Linear, false);

        tween.set_time(Duration::from_secs(5));
        assert_eq!(tween.value, 5.0);

        tween.set_time(Duration::from_secs(12));
        assert_eq!(tween.value, 10.0);
        assert_eq!(tween.overflow, Duration::from_secs(2));
        assert!(tween.is_finished());
    }

    #[test]
    fn reversed_update_counts_down() {
        let mut tween = Tween::new(0.0, 10.0, Duration::from_secs(10), Easing::Linear, false);
        tween.reversed = true;
        tween.set_time(Duration::from_secs(5));

        tween.update(Duration::from_secs(2));
        assert_eq!(tween.time, Duration::from_secs(3));
        assert_eq!(tween.value, 3.0);

        tween.update(Duration::from_secs(4));
        assert_eq!(tween.time, Duration::ZERO);
        assert_eq!(tween.overflow, Duration::from_secs(1));
        assert!(tween.is_finished());
    }

    #[test]
    fn yoyo_reverses_at_the_end() {
        let mut tween = Tween::new(0.0, 10.0, Duration::from_secs(10), Easing::Linear, true);

        tween.update(Duration::from_secs(12));
        assert!(tween.reversed);
        assert_eq!(tween.time, Duration::from_secs(8));
        assert_eq!(tween.value, 8.0);
    }
    #[test]
    fn set_time_exact_end_has_no_overflow() {
        let mut tween = Tween::new(0.0, 10.0, Duration::from_secs(10), Easing::Linear, false);
        tween.set_time(Duration::from_secs(10));
        assert_eq!(tween.value, 10.0);
        assert_eq!(tween.overflow, Duration::ZERO);
        assert!(tween.is_finished());
    }

    #[test]
    fn reversed_finish_sets_begin_value() {
        let mut tween = Tween::new(0.0, 10.0, Duration::from_secs(10), Easing::Linear, false);
        tween.reversed = true;
        tween.set_time(Duration::from_secs(5));
        tween.update(Duration::from_secs(6)); // 5 sn'den 6 sn düşer, 1 sn taşar
        assert_eq!(tween.time, Duration::ZERO);
        assert_eq!(tween.value, 0.0);
        assert_eq!(tween.overflow, Duration::from_secs(1));
        assert!(tween.is_finished());
    }

    #[test]
    fn yoyo_exact_end_flips_without_overflow() {
        let mut tween = Tween::new(0.0, 10.0, Duration::from_secs(10), Easing::Linear, true);
        tween.update(Duration::from_secs(10));
        assert!(tween.reversed);
        assert_eq!(tween.time, Duration::from_secs(10));
        assert_eq!(tween.value, 10.0);
        assert_eq!(tween.overflow, Duration::ZERO);
        assert!(!tween.is_finished());
    }

    #[test]
    fn yoyo_multiple_flips_in_one_update() {
        let mut tween = Tween::new(0.0, 10.0, Duration::from_secs(10), Easing::Linear, true);
        tween.update(Duration::from_secs(25)); // ileri, geri, tekrar ileri
        assert!(!tween.reversed);
        assert_eq!(tween.time, Duration::from_secs(5));
        assert_eq!(tween.value, 5.0);
        assert_eq!(tween.overflow, Duration::ZERO);
    }

    #[test]
    fn zero_duration_yoyo_does_not_hang() {
        let mut tween = Tween::new(1.0, 5.0, Duration::ZERO, Easing::Linear, true);
        tween.update(Duration::from_secs(1));
        assert_eq!(tween.value, 5.0);
        assert!(tween.is_finished());
    }
}
