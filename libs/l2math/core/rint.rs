use crate::Float64;

/// Round to nearest integer, rounding halfway cases away from zero.
#[export_name = "__l2math_rint"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn rint(x: Float64) -> Float64 {
    let one_over_e = 1.0 / Float64::EPSILON;
    let as_u64: u64 = x.to_bits();
    let exponent: u64 = as_u64 >> 52 & 0x7ff;
    let is_positive = (as_u64 >> 63) == 0;
    if exponent >= 0x3ff + 52 {
        return x;
    }
    let ans = if is_positive {
        #[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
        let x = force_eval!(x);
        let xplusoneovere = x + one_over_e;
        #[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
        let xplusoneovere = force_eval!(xplusoneovere);
        xplusoneovere - one_over_e
    } else {
        #[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
        let x = force_eval!(x);
        let xminusoneovere = x - one_over_e;
        #[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
        let xminusoneovere = force_eval!(xminusoneovere);
        xminusoneovere + one_over_e
    };

    if ans != 0.0 {
        return ans;
    };
    if is_positive {
        0.0
    } else {
        -0.0
    }
}

// PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
#[cfg(not(target_arch = "powerpc64"))]
#[cfg(test)]
mod tests {
    use super::rint;

    #[test]
    fn negative_zero() {
        assert_eq!(rint(-0.0_f64).to_bits(), (-0.0_f64).to_bits());
    }

    #[test]
    fn sanity_check() {
        assert_eq!(rint(-1.0), -1.0);
        assert_eq!(rint(2.8), 3.0);
        assert_eq!(rint(-0.5), -0.0);
        assert_eq!(rint(0.5), 0.0);
        assert_eq!(rint(-1.5), -2.0);
        assert_eq!(rint(1.5), 2.0);
    }
}
