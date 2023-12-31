/* origin: FreeBSD /usr/src/lib/msun/src/s_sin.c */
/**
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
*/

use crate::{Float64, Radian64};

use super::{get_high_word, k_cos, k_sin, rem_pio2};

/// Simultaneously computes the sine and cosine of the argument x.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn sincos(x: Radian64) -> (Float64, Float64) {
    let ix = get_high_word(x) & 0x7fffffff;

    /* |x| ~< pi/4 */
    if ix <= 0x3fe921fb {
        /* if |x| < 2**-27 * sqrt(2) */
        if ix < 0x3e46a09e {
            /* raise inexact if x!=0 and underflow if subnormal */
            let x1p120 = Float64::from_bits(0x4770000000000000); // 0x1p120 == 2^120
            if ix < 0x00100000 {
                force_eval!(x / x1p120);
            } else {
                force_eval!(x + x1p120);
            }
            return (x, 1.0);
        }
        return (k_sin(x, 0.0, 0), k_cos(x, 0.0));
    }

    /* sincos(Inf or NaN) is NaN */
    if ix >= 0x7ff00000 {
        let rv = x - x;
        return (rv, rv);
    }

    /* argument reduction needed */
    let (n, y0, y1) = rem_pio2(x);
    let s = k_sin(y0, y1, 1);
    let c = k_cos(y0, y1);
    match n & 3 {
        0 => (s, c),
        1 => (c, -s),
        2 => (-s, -c),
        3 => (-c, s),
        #[cfg(debug_assertions)]
        _ => unreachable!(),
        #[cfg(not(debug_assertions))]
        _ => (0.0, 1.0),
    }
}

/// FFI bindings for sincos
#[inline]
#[doc(hidden)]
#[export_name = "__l2math_sincos"]
pub extern "C" fn __l2math_sincos(x: Float64) -> super::Tuple_Float64_Float64 {
    super::Tuple_Float64_Float64::from(sincos(x))
}

// These tests are based on those from sincosf.rs
#[cfg(test)]
mod tests {
    use super::sincos;
    use super::Float64;

    const TOLERANCE: Float64 = 1e-6;

    #[test]
    fn with_pi() {
        let (s, c) = sincos(core::f64::consts::PI);
        assert!(
            (s - 0.0).abs() < TOLERANCE,
            "|{} - {}| = {} >= {}",
            s,
            0.0,
            (s - 0.0).abs(),
            TOLERANCE
        );
        assert!(
            (c + 1.0).abs() < TOLERANCE,
            "|{} + {}| = {} >= {}",
            c,
            1.0,
            (s + 1.0).abs(),
            TOLERANCE
        );
    }

    #[test]
    fn rotational_symmetry() {
        use core::f64::consts::PI;
        const N: usize = 24;
        for n in 0..N {
            let theta = 2. * PI * (n as Float64) / (N as Float64);
            let (s, c) = sincos(theta);
            let (s_plus, c_plus) = sincos(theta + 2. * PI);
            let (s_minus, c_minus) = sincos(theta - 2. * PI);

            assert!(
                (s - s_plus).abs() < TOLERANCE,
                "|{} - {}| = {} >= {}",
                s,
                s_plus,
                (s - s_plus).abs(),
                TOLERANCE
            );
            assert!(
                (s - s_minus).abs() < TOLERANCE,
                "|{} - {}| = {} >= {}",
                s,
                s_minus,
                (s - s_minus).abs(),
                TOLERANCE
            );
            assert!(
                (c - c_plus).abs() < TOLERANCE,
                "|{} - {}| = {} >= {}",
                c,
                c_plus,
                (c - c_plus).abs(),
                TOLERANCE
            );
            assert!(
                (c - c_minus).abs() < TOLERANCE,
                "|{} - {}| = {} >= {}",
                c,
                c_minus,
                (c - c_minus).abs(),
                TOLERANCE
            );
        }
    }
}
