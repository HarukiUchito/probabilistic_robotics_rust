use std::f64;
use std::ops::*;

pub const PI: f64 = f64::consts::PI;

#[macro_export]
macro_rules! deg2rad {
    ($deg:expr) => ($deg * PI / 180.0);
}

#[macro_export]
macro_rules! rad2deg {
    ($rad:expr) => ($rad * 180.0 / PI);
}

#[inline]
pub fn square<T>(v: T) -> T
where T: Mul<Output = T> + Copy
{
    v * v
}

#[inline]
pub fn cube<T>(v: T) -> T
where T: Mul<Output = T> + Copy
{
    v * v * v
}

/// assert if not norm(vectorA - vectorB) < f64C
#[macro_export]
macro_rules! assert_near_vec {
    ($a:expr, $b:expr, $c:expr) => {
        assert!(($a - $b).norm_l2() < $c, "assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`, threshold: `{:?})", $a, $b, $c);
    }
}