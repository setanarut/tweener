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
