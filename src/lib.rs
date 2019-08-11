//! A crate to test if floats are almost equal.
//!
//! ```
//! # let (x, y, z) = (0.5, 1.0, 2.0);
//! // Compare two variables.
//! if almost::equal(x, y) {
//!    println!("They're almost equal!");
//! }
//! // Or, if you need need to compare with a constant zero:
//! if almost::zero(z) {
//!    println!("It's almost zero!");
//! }
//! ```
//!
//! # Why another crate?
//!
//! There are a lot of crates for doing this already.
//!
//! The author crate has fairly strong opinions on how this should be done, and
//! thinks most of the similar crates in the wild make dubious choices, make it
//! easy for the user to misuse, or follow poor numerical robustness practices.
//!
//! Specific differences / benefits compared to other crates:
//!
//! 1. Better choice of default tolerances for unknown inputs. Often the value
//!    of the `EPSILON` is used as a value for tolerance, or its use is
//!    encouraged by the API).
//!
//!    This is the wrong choice more often than it is right. The machine epsilon
//!    is a quite strict bound for comparison, and after just a few arithmetic
//!    operations you will no longer be within it.
//!
//!    This library chooses a default tolerance value that is much more
//!    forgiving while still tight enough for it to be unlikely to cause false
//!    positives (specifically, it assumes roughly half of the bits have been
//!    lost to rounding, e.g. the *square root* of the machine epsilon).
//!
//! 2. Relative comparison by default. Most of the crates in the wild seem to
//!    use a hybrid between relative and absolute comparison. This is bad for
//!    arbitrary numbers which may have any scale, and gives up a number of
//!    desirable properties of the floating point number system.
//!
//! 3. Absolute comparison with zero. The only downside to using relative
//!    comparison by default is that it is essentially never useful to use
//!    relative comparison where one of the values is known in advance to be
//!    zero.
//!
//!    As a result, this library provides `almost::zero(v)` as well, which uses
//!    absolute comparison.
//!
//! 4. Properly handling both overflow and underflow.
//!
//!    Because this library uses relative comparison, denormal numbers behave
//!    properly, as well as comparisons where one of the values has overflowed
//!    to infinity. The second might sound impossible, but we can just rescale
//!    both values, and compare with the same tolerance.
//!
//! 5. Simple API. We don't expose other ways of comparing numbers, most of
//!    which are either dubious choices for non-niche use cases.
//!
//! That said, there's no one size fits all here. Numerical robustness is full
//! of tradeoffs, and while I believe the ones made by this library are good for
//! most cases, they do not and cannot satisfy every possible case.
#![no_std]

pub(crate) mod imp;

/// Returns `true` if `lhs` and `rhs` are almost equal.
///
/// ```
/// assert!(almost::equal(0.1 + 0.2, 0.3));
/// ```
///
/// Do not use this to compare a value with a constant zero. Instead, for this
/// you should use [`almost::zero`](zero).
///
/// This uses a relative comparison with a threshold chosen to be good for
/// values produced by arbitrary computation, while still tight enough for it to
/// be unlikely to cause false positives in practice This is a good default, but
/// you have a tighter bound you need to use,
/// [`almost::equal_with`](equal_with) is also available.
///
/// Note that this returns false in the case that both values are NaN.
#[inline]
pub fn equal<T: AlmostEqual>(lhs: T, rhs: T) -> bool {
    lhs.almost_equals(rhs)
}

/// Returns `true` if `a` is almost zero.
///
/// ```
/// # use core as std;
/// assert!(almost::zero(std::f32::EPSILON));
/// ```
///
/// This is the correct function to use when comparing to see if a value is
/// almost zero.
///
/// Testing if a value is zero is a unique case where a relative comparison
/// becomes almost useless, and an absolute comparison becomes the obviously
/// correct choice.
///
/// As such, this performs comparison with a *absolute* threshold. The threshold
/// used assumes half of the bits have been lost to rounding, which is a good
/// default for user code that does not keep track of this, while still tight
/// enough for it to be unlikely to cause false positives in practice. However,
/// if you need a tighter bound, the function
/// [`almost::zero_with`](zero_with) can be used.
#[inline]
pub fn zero<T: AlmostEqual>(a: T) -> bool {
    a.almost_zero()
}

/// Returns `true` if `a` is almost zero, using the specified absolute
/// tolerance.
///
/// ```
/// # use core as std;
/// assert!(!almost::zero_with(std::f32::EPSILON, std::f32::EPSILON));
/// ```
///
/// This is a version of [`almost::zero`](zero) which does not define a
/// tolerance value for you.
///
/// The correct choice of tolerance value is tricky and differs depending on:
///
/// - The type of numerical operations you're doing.
/// - The scale of the values used.
/// - The number of operations performed.
///
/// However, for comparison with zero it is possible to give broad guidelines
/// (note that these do *not* apply to
/// [`almost::equal_with`](equal_with), which for which the
/// correct decision is more challenging).
///
/// # Panics
/// This function panics in debug mode if `tolerance` is not greater than zero,
/// as the results are unlikely to be sensible.
///
/// In release builds it should never panic.
#[inline]
pub fn zero_with<T: AlmostEqual>(v: T, tolerance: T::Float) -> bool{
    v.almost_zero_with(tolerance)
}

/// Returns `true` if `lhs` and `rhs` are almost equal using the provided
/// relative tolerance.
///
/// ```
/// const MY_TOLERANCE: f32 = almost::F32_TOLERANCE / 2.0;
/// assert!(almost::equal_with(0.1 + 0.2, 0.3f32, MY_TOLERANCE));
/// ```
///
/// Do not use this to compare a value with a constant zero. Instead, for this
/// you should use [`almost::zero_with`](zero_with).
///
/// # Panics
/// This function panics in debug mode if `tolerance` is less than `T::EPSILON`
/// or greater than 1.0, as the results are unlikely to be sensible.
///
/// In release builds it should never panic.
#[inline]
pub fn equal_with<T: AlmostEqual>(lhs: T, rhs: T, tolerance: T::Float) -> bool {
    lhs.almost_equals_with(rhs, tolerance)
}

/// A trait for comparing floating point numbers. Not broadly intended to be
/// used by most code (instead, use the functions at the crate root), however it
/// could be useful for generic code too.
pub trait AlmostEqual {
    /// The floating point type. For f32 and f64 this is Self, but for custom
    /// aggregate types it could be different.
    type Float;

    /// The default tolerance value for this type. Typically equivalent to
    /// `T::EPSILON.sqrt()`, as we assume that around half of the precision bits
    /// of any arbitrary computation have been rounded away.
    const DEFAULT_TOLERANCE: Self::Float;

    /// The machine epsilon for this type. This generally should not be used as
    /// a tolerance value (it's frequently too strict), however it can be useful
    /// when computing tolerances.
    const MACHINE_EPSILON: Self::Float;

    /// Equivalent to [`almost::zero`](zero).
    /// ```
    /// # let v = 0.000001f32;
    /// # use almost::AlmostEqual;
    /// assert!(v.almost_zero());
    /// ```
    #[inline]
    fn almost_zero(self) -> bool where Self: Sized {
        self.almost_zero_with(Self::DEFAULT_TOLERANCE)
    }

    /// Equivalent to [`almost::equal`](equal).
    /// ```
    /// # let (a, b) = (0.5, 0.5);
    /// # use almost::AlmostEqual;
    /// assert!(a.almost_equals(b));
    /// ```
    #[inline]
    fn almost_equals(self, rhs: Self) -> bool where Self: Sized {
        self.almost_equals_with(rhs, Self::DEFAULT_TOLERANCE)
    }

    /// Equivalent to [`almost::equal_with`](equal_with).
    /// ```
    /// # let (a, b) = (0.5f32, 0.5);
    /// # use almost::AlmostEqual;
    /// const MY_TOLERANCE: f32 = almost::F32_TOLERANCE / 2.0;
    /// assert!(a.almost_equals_with(b, MY_TOLERANCE));
    /// ```
    fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool;

    /// Equivalent to [`almost::zero_with`](zero_with).
    /// ```
    /// # use almost::AlmostEqual;
    /// assert!(0.01.almost_zero_with(0.05));
    /// ```
    fn almost_zero_with(self, tol: Self::Float) -> bool;
}

/// The default tolerance used for `f64`. Equivalent to `f64::EPSILON.sqrt()`
/// (or `0.000000014901161193847656_f64`), as we assume that around half of the
/// precision bits of any arbitrary value have been rounded away.
pub const F64_TOLERANCE: f64 = 0.000000014901161193847656_f64;
/// The default tolerance used for `f32`. Equivalent to `f32::EPSILON.sqrt()`
/// (or `0.00034526698_f32`), as we assume that around half of the precision
/// bits of any arbitrary value have been rounded away.
pub const F32_TOLERANCE: f32 = 0.00034526698_f32;

impl AlmostEqual for f64 {
    type Float = f64;

    const MACHINE_EPSILON: Self::Float = core::f64::EPSILON;

    const DEFAULT_TOLERANCE: Self::Float = F64_TOLERANCE;

    fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool {
        debug_assert!(tol < 1.0, "Tolerance should not be greater than 1.0");
        debug_assert!(tol >= Self::MACHINE_EPSILON, "Tolerance should not be smaller than the machine epsilon");
        crate::imp::f64::eq_with_tol_impl(self, rhs, tol)
    }

    fn almost_zero_with(self, tol: Self::Float) -> bool {
        debug_assert!(tol > 0.0);
        crate::imp::f64::abs(self) < tol
    }
}


impl AlmostEqual for f32 {
    type Float = f32;

    const MACHINE_EPSILON: Self::Float = core::f32::EPSILON;

    const DEFAULT_TOLERANCE: Self::Float = F32_TOLERANCE;

    fn almost_equals_with(self, rhs: Self, tol: Self::Float) -> bool {
        debug_assert!(tol < 1.0, "Tolerance should not be greater than 1.0");
        debug_assert!(tol >= Self::MACHINE_EPSILON, "Tolerance should not be smaller than the machine epsilon");
        crate::imp::f32::eq_with_tol_impl(self, rhs, tol)
    }

    fn almost_zero_with(self, tol: Self::Float) -> bool {
        debug_assert!(tol > 0.0);
        crate::imp::f32::abs(self) < tol
    }
}

