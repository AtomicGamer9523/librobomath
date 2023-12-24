use crate::{Float32, Radian32};

use super::ln1pf;

/* atanh(x) = log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2 ~= x + x^3/3 + o(x^5) */
/// Inverse hyperbolic tangent
///
/// Calculates the inverse hyperbolic tangent of `x`.
/// Is defined as `log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2`.
#[export_name = "__l2math_atanhf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn atanhf(mut x: Float32) -> Radian32 {
    let mut u = x.to_bits();
    let sign = (u >> 31) != 0;

    /* |x| */
    u &= 0x7fffffff;
    x = Float32::from_bits(u);

    if u < 0x3f800000 - (1 << 23) {
        if u < 0x3f800000 - (32 << 23) {
            /* handle underflow */
            if u < (1 << 23) {
                force_eval!((x * x) as Float32);
            }
        } else {
            /* |x| < 0.5, up to 1.7ulp error */
            x = 0.5 * ln1pf(2.0 * x + 2.0 * x * x / (1.0 - x));
        }
    } else {
        /* avoid overflow */
        x = 0.5 * ln1pf(2.0 * (x / (1.0 - x)));
    }

    if sign {
        -x
    } else {
        x
    }
}
