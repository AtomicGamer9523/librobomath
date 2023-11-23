use crate::Float64;

/// Remainder of floating point division 
/// 
/// A.K.A. `modulus` or `modulo` in other languages.
/// 
/// Computes the floating-point remainder of the division operation `x/y`.
#[export_name = "__l2math_fmod"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn fmod(x: Float64, y: Float64) -> Float64 {
    let mut uxi = x.to_bits();
    let mut uyi = y.to_bits();
    let mut ex = (uxi >> 52 & 0x7ff) as i64;
    let mut ey = (uyi >> 52 & 0x7ff) as i64;
    let sx = uxi >> 63;
    let mut i;

    if uyi << 1 == 0 || y.is_nan() || ex == 0x7ff {
        return (x * y) / (x * y);
    }
    if uxi << 1 <= uyi << 1 {
        if uxi << 1 == uyi << 1 {
            return 0.0 * x;
        }
        return x;
    }

    /* normalize x and y */
    if ex == 0 {
        i = uxi << 12;
        while i >> 63 == 0 {
            ex -= 1;
            i <<= 1;
        }
        uxi <<= -ex + 1;
    } else {
        uxi &= u64::MAX >> 12;
        uxi |= 1 << 52;
    }
    if ey == 0 {
        i = uyi << 12;
        while i >> 63 == 0 {
            ey -= 1;
            i <<= 1;
        }
        uyi <<= -ey + 1;
    } else {
        uyi &= u64::MAX >> 12;
        uyi |= 1 << 52;
    }

    /* x mod y */
    while ex > ey {
        i = uxi.wrapping_sub(uyi);
        if i >> 63 == 0 {
            if i == 0 {
                return 0.0 * x;
            }
            uxi = i;
        }
        uxi <<= 1;
        ex -= 1;
    }
    i = uxi.wrapping_sub(uyi);
    if i >> 63 == 0 {
        if i == 0 {
            return 0.0 * x;
        }
        uxi = i;
    }
    while uxi >> 52 == 0 {
        uxi <<= 1;
        ex -= 1;
    }

    /* scale result */
    if ex > 0 {
        uxi -= 1 << 52;
        uxi |= (ex as u64) << 52;
    } else {
        uxi >>= -ex + 1;
    }
    uxi |= sx << 63;

    Float64::from_bits(uxi)
}
