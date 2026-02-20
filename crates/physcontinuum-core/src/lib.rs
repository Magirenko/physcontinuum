//! A foundational library for physics continuum simulations.
//!
//! This crate provides core utilities and types for building
//! high-performance physics simulations.

/// Adds two unsigned integers and returns the sum.
///
/// # Examples
///
/// ```
/// let result = physcontinuum_core::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
