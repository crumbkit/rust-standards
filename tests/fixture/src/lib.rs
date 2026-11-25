//! Arithmetic that the self-test's lane builds, lints, tests and documents.

/// Adds two numbers, and returns `None` when the sum overflows a `u32`.
///
/// ```
/// assert_eq!(rust_lane_fixture::checked_add(2, 3), Some(5));
/// assert_eq!(rust_lane_fixture::checked_add(u32::MAX, 1), None);
/// ```
#[must_use]
pub const fn checked_add(left: u32, right: u32) -> Option<u32> {
    left.checked_add(right)
}

/// Doubles a number, and returns `None` when the result overflows a `u32`.
#[cfg(feature = "double")]
#[must_use]
pub const fn double(value: u32) -> Option<u32> {
    value.checked_mul(2)
}

#[cfg(test)]
mod tests {
    use super::checked_add;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(checked_add(2, 3), Some(5));
    }

    #[test]
    fn reports_overflow() {
        assert_eq!(checked_add(u32::MAX, 1), None);
    }

    #[cfg(feature = "double")]
    #[test]
    fn doubles_a_number() {
        assert_eq!(super::double(4), Some(8));
    }
}
