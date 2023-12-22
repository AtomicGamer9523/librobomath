use crate::Float64;

/// Absolute value (magnitude)
/// 
/// Calculates the absolute value (magnitude) of the argument `x`,
/// by direct manipulation of the bit representation of `x`.
#[export_name = "__l2math_fabs"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn fabs(x: Float64) -> Float64 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `Float64.abs` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::fabsf64(x) }
        }
    }
    Float64::from_bits(x.to_bits() & (u64::MAX / 2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::*;

    #[test]
    fn sanity_check() {
        assert_eq!(fabs(-1.0), 1.0);
        assert_eq!(fabs(2.8), 2.8);
    }

    /// The spec: https://en.cppreference.com/w/cpp/numeric/math/fabs
    #[test]
    fn spec_tests() {
        assert!(fabs(NAN).is_nan());
        for f in [0.0, -0.0].iter().copied() {
            assert_eq!(fabs(f), 0.0);
        }
        for f in [INFINITY, NEG_INFINITY].iter().copied() {
            assert_eq!(fabs(f), INFINITY);
        }
    }
}
