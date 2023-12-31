use crate::Float64;

use super::sqrt;

const SPLIT: Float64 = 134217728. + 1.; // 0x1p27 + 1 === (2 ^ 27) + 1

fn sq(x: Float64) -> (Float64, Float64) {
    let xc = x * SPLIT;
    let xh = x - xc + xc;
    let xl = x - xh;
    let hi = x * x;
    let lo = xh * xh - hi + 2. * xh * xl + xl * xl;
    (hi, lo)
}

/// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y.
#[export_name = "__l2math_hypot"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn hypot(mut x: Float64, mut y: Float64) -> Float64 {
    let x1p700 = Float64::from_bits(0x6bb0000000000000); // 0x1p700 === 2 ^ 700
    let x1p_700 = Float64::from_bits(0x1430000000000000); // 0x1p-700 === 2 ^ -700

    let mut uxi = x.to_bits();
    let mut uyi = y.to_bits();
    let uti;
    let mut z: Float64;

    /* arrange |x| >= |y| */
    uxi &= -1i64 as u64 >> 1;
    uyi &= -1i64 as u64 >> 1;
    if uxi < uyi {
        uti = uxi;
        uxi = uyi;
        uyi = uti;
    }

    /* special cases */
    let ex: i64 = (uxi >> 52) as i64;
    let ey: i64 = (uyi >> 52) as i64;
    x = Float64::from_bits(uxi);
    y = Float64::from_bits(uyi);
    /* note: hypot(inf,nan) == inf */
    if ey == 0x7ff {
        return y;
    }
    if ex == 0x7ff || uyi == 0 {
        return x;
    }
    /* note: hypot(x,y) ~= x + y*y/x/2 with inexact for small y/x */
    /* 64 difference is enough for ld80 double_t */
    if ex - ey > 64 {
        return x + y;
    }

    /* precise sqrt argument in nearest rounding mode without overflow */
    /* xh*xh must not overflow and xl*xl must not underflow in sq */
    z = 1.;
    if ex > 0x3ff + 510 {
        z = x1p700;
        x *= x1p_700;
        y *= x1p_700;
    } else if ey < 0x3ff - 450 {
        z = x1p_700;
        x *= x1p700;
        y *= x1p700;
    }
    let (hx, lx) = sq(x);
    let (hy, ly) = sq(y);
    z * sqrt(ly + lx + hy + hx)
}
