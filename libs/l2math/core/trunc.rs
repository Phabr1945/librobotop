use crate::Float64;

/// Returns the integer part of self.
/// This means that non-integer numbers are always truncated towards zero.
#[export_name = "__l2math_trunc"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn trunc(x: Float64) -> Float64 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `Float64.trunc` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::truncf64(x) }
        }
    }
    let x1p120 = Float64::from_bits(0x4770000000000000); // 0x1p120f === 2 ^ 120

    let mut i: u64 = x.to_bits();
    let mut e: i64 = (i >> 52 & 0x7ff) as i64 - 0x3ff + 12;

    if e >= 52 + 12 {
        return x;
    }
    if e < 12 {
        e = 1;
    }
    let m: u64 = -1i64 as u64 >> e;
    if (i & m) == 0 {
        return x;
    }
    force_eval!(x + x1p120);
    i &= !m;
    Float64::from_bits(i)
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(super::trunc(1.1), 1.0);
    }
}
