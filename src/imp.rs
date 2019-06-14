
// This is gross but it's also a big pain to write this via a trait...

macro_rules! impl_equals {
    ($fp:ident, $bits:ident, $SIGNIFICAND_SIZE:expr) => {
        const SIGNIFICAND_SIZE: $bits = $SIGNIFICAND_SIZE;
        const EXPONENT_SIZE: $bits = (core::mem::size_of::<$fp>() as $bits) * 8 - SIGNIFICAND_SIZE - 1;
        const EXPONENT_MASK: $bits = ((1 << EXPONENT_SIZE) - 1) << SIGNIFICAND_SIZE;
        const EXPONENT_BIAS: $bits = (1 << (EXPONENT_SIZE - 1)) - 1;

        const SIGN_BIT: $bits = 1 << (core::mem::size_of::<$fp>() as $bits * 8 - 1);

        // abs requires std? ugh.
        #[inline]
        pub(crate) fn abs(f: $fp) -> $fp {
            $fp::from_bits(f.to_bits() & !SIGN_BIT)
        }

        #[inline]
        pub(crate) fn eq_with_tol_impl(lhs: $fp, rhs: $fp, tol: $fp) -> bool {
            let left_mag = abs(lhs);
            let right_mag = abs(rhs);
            if !((left_mag < core::$fp::INFINITY) & (right_mag < core::$fp::INFINITY)) {
                handle_not_finite(lhs, rhs, tol)
            } else {
                let scale = if left_mag > right_mag {
                    left_mag
                } else {
                    right_mag
                };
                // If both left_mag and right_mag are subnormal, rescale to
                // MIN_POSITIVE instead, which is what they round against anyway.
                let scale = if scale > core::$fp::MIN_POSITIVE {
                    scale
                } else {
                    core::$fp::MIN_POSITIVE
                };
                let abs_tol = tol * scale;
                abs(lhs - rhs) < abs_tol
            }
        }

        #[cold]
        #[inline(never)]
        fn handle_not_finite(lhs: $fp, rhs: $fp, tol: $fp) -> bool {
            if lhs.is_nan() || rhs.is_nan() {
                false
            } else if lhs.is_infinite() && rhs.is_infinite() {
                lhs == rhs
            } else {
                // One of `rhs` or `lhs` are infinite, and the other is not.
                // They still might be within the requested tolerance, so we
                // rescale both so that we can do that.

                // ensure lhs is the infinite one.
                let (lhs, rhs) = if lhs.is_infinite() { (lhs, rhs) } else { (rhs, lhs) };
                debug_assert!(rhs.is_finite() && lhs.is_infinite(), "logic bug {} {} {:x} {:x}", lhs, rhs, lhs.to_bits(), rhs.to_bits());
                let rbits = rhs.to_bits();
                if (rbits & EXPONENT_MASK) == 0 {
                    // subnormal, so clearly not equal to infinity, and would
                    // otherwise need special casing below.
                    return false;
                }
                // XXX: does rust turn this into a constant like it should?
                let max_float_binade_bits = core::$fp::MAX.to_bits() & EXPONENT_MASK;
                // copysign requires std, so just build directly.
                let new_lhs = $fp::from_bits(max_float_binade_bits | (lhs.to_bits() & SIGN_BIT));

                let rhs_rescale = $fp::from_bits((EXPONENT_BIAS - 1) << SIGNIFICAND_SIZE);
                let new_rhs = rhs * rhs_rescale;

                eq_with_tol_impl(new_lhs, new_rhs, tol)
            }
        }

    };
}

pub(crate) mod f32 {
    impl_equals!(f32, u32, 23);
}


pub(crate) mod f64 {
    impl_equals!(f64, u64, 52);
}