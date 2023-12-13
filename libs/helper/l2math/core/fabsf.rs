use crate::Float32;

/// Absolute value (magnitude)
/// 
/// Calculates the absolute value (magnitude) of the argument `x`,
/// by direct manipulation of the bit representation of `x`.
#[export_name = "__l2math_fabsf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn fabsf(x: Float32) -> Float32 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `Float32.abs` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::fabsf32(x) }
        }
    }
    Float32::from_bits(x.to_bits() & 0x7fffffff)
}

// PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
#[cfg(not(target_arch = "powerpc64"))]
#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::*;

    #[test]
    fn sanity_check() {
        assert_eq!(fabsf(-1.0), 1.0);
        assert_eq!(fabsf(2.8), 2.8);
    }

    /// The spec: https://en.cppreference.com/w/cpp/numeric/math/fabs
    #[test]
    fn spec_tests() {
        assert!(fabsf(NAN).is_nan());
        for f in [0.0, -0.0].iter().copied() {
            assert_eq!(fabsf(f), 0.0);
        }
        for f in [INFINITY, NEG_INFINITY].iter().copied() {
            assert_eq!(fabsf(f), INFINITY);
        }
    }
}
