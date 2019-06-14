
//! A crate to test if floats are almost equal.
//!
//! There are already a lot of crates to do this. Many of them are fine but I
//! feel that most of them make it too easy for someone who doesn't know they're
//! doing to make a poor decision.
//!
//! This crate is somewhat opinionated on what the right thing for code
//!
//! There are a lot of choices you can make for how to correctly compare
//! floating point numbers, and no single choice is correct for all use cases.
//!
//! This crate strives to make choices that are good for most cases.
//!
//! 1. Arbitrary floats should usually be compared with relative tolerance.
//! 2. Zero should be compared with absolute tolerance.
//! 3. Uses a better default for tolerance than `std::{f32,f64}::EPSILON`.
//! 4. Handles infinities properly.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use core as std;

// pub mod f32;
// pub mod f64;
pub(crate) mod imp;

#[inline]
pub fn equal<T: AlmostEqual>(lhs: T, rhs: T) -> bool {
    lhs.almost_equals(rhs)
}

#[inline]
pub fn zero<T: AlmostEqual>(a: T) -> bool {
    a.almost_zero()
}

#[inline]
pub fn zero_with_tolerance<T: AlmostEqual>(v: T, tolerance: T) -> bool{
    v.almost_zero_with_tolerance(tolerance)
}

#[inline]
pub fn equal_with_tolerance<T: AlmostEqual>(lhs: T, rhs: T, tolerance: T) -> bool {
    lhs.almost_equal_with_tolerance(rhs, tolerance)
}


/// A trait for comparing floating point numbers. Not broadly intended to be
/// used by most code (instead, use the functions at the crate root), however it
/// could be useful for generic code too.
pub trait AlmostEqual {
    /// The default tolerance value for this type. Typically equivalent to
    /// `T::EPSILON.sqrt()`, as we assume that around half of the precision bits
    /// of any arbitrary computation have been rounded away.
    const DEFAULT_TOLERANCE: Self;

    /// The machine epsilon for this type. This generally should not be used as
    /// a tolerance value (it's frequently too strict), however it can be useful
    /// when computing tolerances.
    const MACHINE_EPSILON: Self;

    /// Returns true if `self` is
    #[inline]
    fn almost_zero(self) -> bool where Self: Sized {
        self.almost_zero_with_tolerance(Self::DEFAULT_TOLERANCE)
    }

    #[inline]
    fn almost_equals(self, rhs: Self) -> bool where Self: Sized {
        self.almost_equal_with_tolerance(rhs, Self::DEFAULT_TOLERANCE)
    }

    /// Compare `self` with `rhs` using the tolerance `tol` as a relative
    /// tolerance.
    fn almost_equal_with_tolerance(self, rhs: Self, tol: Self) -> bool;

    /// Compare `self` with `rhs` using the tolerance `tol` as an absolute
    /// tolerance.
    ///
    /// Note: This function's fairly long name is because this is not usually
    /// what you actually want to do.
    fn almost_zero_with_tolerance(self, tol: Self) -> bool;

}

/// Constants and operations on f64.
pub mod f64 {
    use super::*;

    /// The default tolerance used for `f64`. Equivalent to `f64::EPSILON.sqrt()`
    /// (or `0.000000014901161193847656_f64`), as we assume that around half of the
    /// precision bits of any arbitrary computation have been rounded away.
    pub const TOLERANCE: f64 = 0.000000014901161193847656_f64;

    impl AlmostEqual for f64 {

        const MACHINE_EPSILON: Self = std::f64::EPSILON;

        const DEFAULT_TOLERANCE: Self = TOLERANCE;

        fn almost_equal_with_tolerance(self, rhs: Self, tol: Self) -> bool {
            debug_assert!(tol < 1.0, "Tolerance should not be greater than 1.0");
            debug_assert!(tol >= Self::MACHINE_EPSILON, "Tolerance should not be smaller than the machine epsilon");
            crate::imp::f64::eq_with_tol_impl(self, rhs, tol)
        }

        fn almost_zero_with_tolerance(self, tol: Self) -> bool {
            debug_assert!(tol > 0.0);
            crate::imp::f64::abs(self) < tol
        }
    }
}

/// Constants and operations on f64.
pub mod f32 {
    use super::*;

    /// The default tolerance used for `f32`. Equivalent to `f32::EPSILON.sqrt()`
    /// (or `0.00034526698_f32`), as we assume that around half of the precision
    /// bits of any arbitrary computation have been rounded away.
    pub const TOLERANCE: f32 = 0.00034526698_f32;

    impl AlmostEqual for f32 {

        const MACHINE_EPSILON: Self = std::f32::EPSILON;

        const DEFAULT_TOLERANCE: Self = TOLERANCE;

        fn almost_equal_with_tolerance(self, rhs: Self, tol: Self) -> bool {
            debug_assert!(tol < 1.0, "Tolerance should not be greater than 1.0");
            debug_assert!(tol >= Self::MACHINE_EPSILON, "Tolerance should not be smaller than the machine epsilon");
            crate::imp::f32::eq_with_tol_impl(self, rhs, tol)
        }

        fn almost_zero_with_tolerance(self, tol: Self) -> bool {
            debug_assert!(tol > 0.0);
            crate::imp::f32::abs(self) < tol
        }
    }
}
