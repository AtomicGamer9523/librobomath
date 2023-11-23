use crate::{Float64, Float32, Radian64};

/**
 * tanh(x) = (exp(x) - exp(-x))/(exp(x) + exp(-x))
 *         = (exp(2*x) - 1)/(exp(2*x) - 1 + 2)
 *         = (1 - exp(-2*x))/(exp(-2*x) - 1 + 2)
*/

use super::expm1;

/// Returns the hyperbolic tangent of `x`.
#[export_name = "__l2math_tanh"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn tanh(mut x: Radian64) -> Float64 {
    let mut uf: Float64 = x;
    let mut ui: u64 = Float64::to_bits(uf);
    let mut t: Float64;

    /* x = |x| */
    let sign: bool = ui >> 63 != 0;
    ui &= !1 / 2;
    uf = Float64::from_bits(ui);
    x = uf;
    let w: u32 = (ui >> 32) as u32;

    if w > 0x3fe193ea {
        /* |x| > log(3)/2 ~= 0.5493 or nan */
        if w > 0x40340000 {
            /* |x| > 20 or nan */
            /* note: this branch avoids raising overflow */
            t = 1.0 - 0.0 / x;
        } else {
            t = expm1(2.0 * x);
            t = 1.0 - 2.0 / (t + 2.0);
        }
    } else if w > 0x3fd058ae {
        /* |x| > log(5/3)/2 ~= 0.2554 */
        t = expm1(2.0 * x);
        t = t / (t + 2.0);
    } else if w >= 0x00100000 {
        /* |x| >= 0x1p-1022, up to 2ulp error in [0.1,0.2554] */
        t = expm1(-2.0 * x);
        t = -t / (t + 2.0);
    } else {
        /* |x| is subnormal */
        /* note: the branch above would not raise underflow in [0x1p-1023,0x1p-1022) */
        force_eval!(x as Float32);
        t = x;
    }

    if sign {
        -t
    } else {
        t
    }
}
