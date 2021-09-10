// [[file:../vecfx.note::*imports][imports:1]]
use crate::Float;
// imports:1 ends here

// [[file:../vecfx.note::*core][core:1]]
#[inline]
fn array_sub_generic<F, const N: usize>(x: [F; N], y: [F; N]) -> [F; N]
where
    F: Float + std::fmt::Debug,
{
    use std::convert::TryInto;
    let d: Vec<_> = x.iter().zip(y.iter()).map(|(a, b)| *a - *b).collect();
    d.try_into().unwrap()
}

#[inline]
fn array_add_generic<F, const N: usize>(x: [F; N], y: [F; N]) -> [F; N]
where
    F: Float + std::fmt::Debug,
{
    use std::convert::TryInto;
    let d: Vec<_> = x.iter().zip(y.iter()).map(|(a, b)| *a + *b).collect();
    d.try_into().unwrap()
}

/// Simple math for array
pub trait ArrayMathExt<F: Float> {
    fn array_add(self, other: Self) -> Self;
    fn array_sub(self, other: Self) -> Self;
    fn array_scale(self, value: F) -> Self;
}

impl<F, const N: usize> ArrayMathExt<F> for [F; N]
where
    F: Float + std::fmt::Debug,
{
    /// Adds two arrays. Returns x + y.
    fn array_add(self, other: Self) -> Self {
        array_add_generic(self, other)
    }

    /// Subtracts two arrays. Returns x - y.
    fn array_sub(self, other: Self) -> Self {
        array_sub_generic(self, other)
    }

    /// Scale one array with a value.
    fn array_scale(self, value: F) -> Self {
        self.map(|x| x * value)
    }
}
// core:1 ends here

// [[file:../vecfx.note::*test][test:1]]
#[test]
fn test_array_sub_add() {
    use crate::approx::*;

    let p1 = [1.0, 2.0, 3.0];
    let p2 = [1.1, 2.2, 3.3];
    let p3 = p2.array_sub(p1);
    let p4 = p2.array_add(p1);
    assert_relative_eq!(p3[0], 0.1, epsilon = 1e-4);
    assert_relative_eq!(p3[1], 0.2, epsilon = 1e-4);
    assert_relative_eq!(p3[2], 0.3, epsilon = 1e-4);
    assert_relative_eq!(p4[0], 2.1, epsilon = 1e-4);
    assert_relative_eq!(p4[1], 4.2, epsilon = 1e-4);
    assert_relative_eq!(p4[2], 6.3, epsilon = 1e-4);
    let p5 = p1.array_scale(2.0);
    assert_eq!(p5[0], 2.0);
    assert_eq!(p5[1], 4.0);
    assert_eq!(p5[2], 6.0);
}
// test:1 ends here
