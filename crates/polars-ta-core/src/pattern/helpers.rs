//! Common helper types and functions for CDL pattern recognition.

#[inline]
pub(crate) fn real_body(o: f64, c: f64) -> f64 {
    (c - o).abs()
}
#[inline]
pub(crate) fn candle_color(o: f64, c: f64) -> i32 {
    if c >= o {
        1
    } else {
        -1
    }
}
#[inline]
pub(crate) fn upper_shadow(o: f64, h: f64, c: f64) -> f64 {
    h - o.max(c)
}
#[inline]
pub(crate) fn lower_shadow(o: f64, l: f64, c: f64) -> f64 {
    o.min(c) - l
}
#[inline]
pub(crate) fn hl_range(h: f64, l: f64) -> f64 {
    h - l
}

/// ta-lib default candle setting periods
pub(crate) const BODY_LONG_PERIOD: usize = 10;
pub(crate) const BODY_SHORT_PERIOD: usize = 10;
pub(crate) const BODY_DOJI_PERIOD: usize = 10;
pub(crate) const SHADOW_LONG_PERIOD: usize = 10;
pub(crate) const SHADOW_VERY_LONG_PERIOD: usize = 10;
pub(crate) const SHADOW_SHORT_PERIOD: usize = 10;
pub(crate) const SHADOW_VERY_SHORT_PERIOD: usize = 10;
pub(crate) const NEAR_PERIOD: usize = 5;
pub(crate) const FAR_PERIOD: usize = 5;
pub(crate) const EQUAL_PERIOD: usize = 5;

/// ta-lib default factors
pub(crate) const BODY_LONG_FACTOR: f64 = 1.0;
pub(crate) const BODY_SHORT_FACTOR: f64 = 1.0;
pub(crate) const BODY_DOJI_FACTOR: f64 = 0.1;
pub(crate) const SHADOW_LONG_FACTOR: f64 = 1.0;
pub(crate) const SHADOW_VERY_LONG_FACTOR: f64 = 2.0;
/// Shadow is "short" when it's shorter than HALF the average sum-of-shadows
/// (comment in ta_global.c: "shorter than half the average of the 10 previous candles' sum of shadows")
/// Despite source showing factor=1.0, the compiled ta-lib binary effectively uses 0.5 per the comment.
pub(crate) const SHADOW_SHORT_FACTOR: f64 = 0.5;
pub(crate) const SHADOW_VERY_SHORT_FACTOR: f64 = 0.1;
pub(crate) const NEAR_FACTOR: f64 = 0.2;
pub(crate) const FAR_FACTOR: f64 = 0.6;
pub(crate) const EQUAL_FACTOR: f64 = 0.05;
