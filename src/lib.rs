#![doc = include_str!("../README.md")]

#[cfg(test)]
mod test;

use std::f64::consts::PI;
use std::time::Duration;

use num_traits::{Float, NumCast};

/// Converts an `f64` constant into the generic float type.
#[inline]
fn lit<T: Float>(v: f64) -> T {
    <T as NumCast>::from(v).unwrap()
}

/// Converts a [`Duration`] into seconds as the generic float type.
#[inline]
fn secs<T: Float>(d: Duration) -> T {
    lit(d.as_secs_f64())
}

/// Interpolates a value from `begin` to `end` over `duration` using the easing function `func`.
///
/// Generic over the float type (`f32` or `f64`); defaults to `f64`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tween<T = f64> {
    /// Current interpolated value.
    pub value: T,
    /// Starting value of the tween.
    pub begin: T,
    /// Target value of the tween.
    pub end: T,
    /// Total time the tween takes to go from `begin` to `end`.
    pub duration: Duration,
    /// Easing function that shapes the progress curve.
    pub func: Easing,
    /// When `true`, the tween plays back in the opposite direction on reaching the end instead of finishing.
    pub yoyo: bool,
    /// Time elapsed so far in the current run.
    pub time: Duration,
    /// Time left over after `time` passes `duration` in the last update, carried into the next cycle or step.
    pub overflow: Duration,
    /// When `true`, the tween is currently playing backwards, from `end` to `begin`.
    pub reversed: bool,
}

impl<T: Float> Tween<T> {
    /// Creates a new tween from `begin` to `end` over `duration` using the easing function `func`.
    pub fn new(begin: T, end: T, duration: Duration, func: Easing, yoyo: bool) -> Self {
        Self {
            value: begin,
            begin,
            end,
            duration,
            func,
            yoyo,
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
                secs(self.time),
                self.begin,
                self.change(),
                secs(self.duration),
            );
        }
    }

    /// Returns the total distance between `begin` and `end`.
    pub fn change(&self) -> T {
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

// Easing functions

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
    pub(crate) fn apply<T: Float>(self, t: T, b: T, c: T, d: T) -> T {
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

type EaseFn<T> = fn(T, T, T, T) -> T;

/// Shared shape of every `OutIn*` easing: the first half plays `out`, the second half plays `inn`.
fn out_in<T: Float>(t: T, b: T, c: T, d: T, out: EaseFn<T>, inn: EaseFn<T>) -> T {
    let two = lit::<T>(2.0);
    if t < d / two {
        return out(t * two, b, c / two, d);
    }
    inn((t * two) - d, b + c / two, c / two, d)
}

fn linear<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c * t / d + b
}

fn in_quad<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c * (t / d).powi(2) + b
}

fn out_quad<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let two = lit::<T>(2.0);
    let t = t / d;
    -c * t * (t - two) + b
}

fn in_out_quad<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let (one, two) = (T::one(), lit::<T>(2.0));
    let t = t / d * two;
    if t < one {
        return c / two * t.powi(2) + b;
    }
    -c / two * ((t - one) * (t - lit::<T>(3.0)) - one) + b
}

fn out_in_quad<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_quad, in_quad)
}

fn in_cubic<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c * (t / d).powi(3) + b
}

fn out_cubic<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c * ((t / d - T::one()).powi(3) + T::one()) + b
}

fn in_out_cubic<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let two = lit::<T>(2.0);
    let mut t = t / d * two;
    if t < T::one() {
        return c / two * t * t * t + b;
    }
    t = t - two;
    c / two * (t * t * t + two) + b
}

fn out_in_cubic<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_cubic, in_cubic)
}

fn in_quart<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c * (t / d).powi(4) + b
}

fn out_quart<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let one = T::one();
    -c * ((t / d - one).powi(4) - one) + b
}

fn in_out_quart<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let two = lit::<T>(2.0);
    let t = t / d * two;
    if t < T::one() {
        return c / two * t.powi(4) + b;
    }
    -c / two * ((t - two).powi(4) - two) + b
}

fn out_in_quart<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_quart, in_quart)
}

fn in_quint<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c * (t / d).powi(5) + b
}

fn out_quint<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c * ((t / d - T::one()).powi(5) + T::one()) + b
}

fn in_out_quint<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let two = lit::<T>(2.0);
    let t = t / d * two;
    if t < T::one() {
        return c / two * t.powi(5) + b;
    }
    c / two * ((t - two).powi(5) + two) + b
}

fn out_in_quint<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_quint, in_quint)
}

fn in_sine<T: Float>(t: T, b: T, c: T, d: T) -> T {
    -c * (t / d * lit::<T>(PI / 2.0)).cos() + c + b
}

fn out_sine<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c * (t / d * lit::<T>(PI / 2.0)).sin() + b
}

fn in_out_sine<T: Float>(t: T, b: T, c: T, d: T) -> T {
    -c / lit::<T>(2.0) * ((lit::<T>(PI) * t / d).cos() - T::one()) + b
}

fn out_in_sine<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_sine, in_sine)
}

fn in_expo<T: Float>(t: T, b: T, c: T, d: T) -> T {
    if t == T::zero() {
        return b;
    }
    c * lit::<T>(2.0).powf(lit::<T>(10.0) * (t / d - T::one())) + b - c * lit::<T>(0.001)
}

fn out_expo<T: Float>(t: T, b: T, c: T, d: T) -> T {
    if t == d {
        return b + c;
    }
    c * lit::<T>(1.001) * (-(lit::<T>(2.0).powf(lit::<T>(-10.0) * t / d)) + T::one()) + b
}

fn in_out_expo<T: Float>(t: T, b: T, c: T, d: T) -> T {
    if t == T::zero() {
        return b;
    }
    if t == d {
        return b + c;
    }
    let (one, two) = (T::one(), lit::<T>(2.0));
    let t = t / d * two;
    if t < one {
        return c / two * two.powf(lit::<T>(10.0) * (t - one)) + b - c * lit::<T>(0.0005);
    }
    c / two * lit::<T>(1.0005) * (-(two.powf(lit::<T>(-10.0) * (t - one))) + two) + b
}

fn out_in_expo<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_expo, in_expo)
}

fn in_circ<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let one = T::one();
    -c * ((one - (t / d).powi(2)).sqrt() - one) + b
}

fn out_circ<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c * (T::one() - (t / d - T::one()).powi(2)).sqrt() + b
}

fn in_out_circ<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let (one, two) = (T::one(), lit::<T>(2.0));
    let mut t = t / d * two;
    if t < one {
        return -c / two * ((one - t * t).sqrt() - one) + b;
    }
    t = t - two;
    c / two * ((one - t * t).sqrt() + one) + b
}

fn out_in_circ<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_circ, in_circ)
}

fn in_elastic<T: Float>(t: T, b: T, c: T, d: T) -> T {
    if t == T::zero() {
        return b;
    }
    let mut t = t / d;
    if t == T::one() {
        return b + c;
    }
    let (p, a, s) = calculate_pas(c, d);
    t = t - T::one();
    -(a * lit::<T>(2.0).powf(lit::<T>(10.0) * t) * ((t * d - s) * lit::<T>(2.0 * PI) / p).sin()) + b
}

fn out_elastic<T: Float>(t: T, b: T, c: T, d: T) -> T {
    if t == T::zero() {
        return b;
    }
    let t = t / d;
    if t == T::one() {
        return b + c;
    }
    let (p, a, s) = calculate_pas(c, d);
    a * lit::<T>(2.0).powf(lit::<T>(-10.0) * t) * ((t * d - s) * lit::<T>(2.0 * PI) / p).sin()
        + c
        + b
}

fn in_out_elastic<T: Float>(t: T, b: T, c: T, d: T) -> T {
    if t == T::zero() {
        return b;
    }
    let (two, half) = (lit::<T>(2.0), lit::<T>(0.5));
    let mut t = t / d * two;
    if t == two {
        return b + c;
    }
    let (p, a, s) = calculate_pas(c, d);
    let tau = lit::<T>(2.0 * PI);
    t = t - T::one();
    if t < T::zero() {
        return -half * (a * two.powf(lit::<T>(10.0) * t) * ((t * d - s) * tau / p).sin()) + b;
    }
    a * two.powf(lit::<T>(-10.0) * t) * ((t * d - s) * tau / p).sin() * half + c + b
}

fn out_in_elastic<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_elastic, in_elastic)
}

fn in_back<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let s = lit::<T>(BACK_S);
    let t = t / d;
    c * t * t * ((s + T::one()) * t - s) + b
}

fn out_back<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let (one, s) = (T::one(), lit::<T>(BACK_S));
    let t = t / d - one;
    c * (t * t * ((s + one) * t + s) + one) + b
}

fn in_out_back<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let (one, two) = (T::one(), lit::<T>(2.0));
    let s = lit::<T>(BACK_S * 1.525);
    let mut t = t / d * two;
    if t < one {
        return c / two * (t * t * ((s + one) * t - s)) + b;
    }
    t = t - two;
    c / two * (t * t * ((s + one) * t + s) + two) + b
}

fn out_in_back<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_back, in_back)
}

fn out_bounce<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let k = lit::<T>(7.5625);
    let mut t = t / d;
    if t < lit::<T>(1.0 / 2.75) {
        return c * (k * t * t) + b;
    }
    if t < lit::<T>(2.0 / 2.75) {
        t = t - lit::<T>(1.5 / 2.75);
        return c * (k * t * t + lit::<T>(0.75)) + b;
    } else if t < lit::<T>(2.5 / 2.75) {
        t = t - lit::<T>(2.25 / 2.75);
        return c * (k * t * t + lit::<T>(0.9375)) + b;
    }
    t = t - lit::<T>(2.625 / 2.75);
    c * (k * t * t + lit::<T>(0.984375)) + b
}

fn in_bounce<T: Float>(t: T, b: T, c: T, d: T) -> T {
    c - out_bounce(d - t, T::zero(), c, d) + b
}

fn in_out_bounce<T: Float>(t: T, b: T, c: T, d: T) -> T {
    let (two, half) = (lit::<T>(2.0), lit::<T>(0.5));
    if t < d / two {
        return in_bounce(t * two, T::zero(), c, d) * half + b;
    }
    out_bounce(t * two - d, T::zero(), c, d) * half + c * half + b
}

fn out_in_bounce<T: Float>(t: T, b: T, c: T, d: T) -> T {
    out_in(t, b, c, d, out_bounce, in_bounce)
}

fn calculate_pas<T: Float>(c: T, d: T) -> (T, T, T) {
    let p = d * lit::<T>(0.3);
    (p, c, p / lit::<T>(4.0))
}
