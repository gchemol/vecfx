// [[file:../vecfx.note::*imports][imports:1]]
use crate::Float;
// imports:1 ends here

// [[file:../vecfx.note::*core][core:1]]
type Array3<F: Float> = [F; 3];

#[inline]
fn array_add<F: Float>(x: Array3<F>, y: Array3<F>) -> Array3<F> {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2]]
}

#[inline]
fn array_sub<F: Float>(x: Array3<F>, y: Array3<F>) -> Array3<F> {
    [x[0] - y[0], x[1] - y[1], x[2] - y[2]]
}

/// Simple math for array
pub trait ArrayMathExt<F: Float> {
    fn array_add(self, other: Self) -> Self;
    fn array_sub(self, other: Self) -> Self;
    fn array_scale(self, value: F) -> Self;
}

impl<F: Float> ArrayMathExt<F> for [F; 3] {
    /// Adds two arrays. Returns x + y.
    fn array_add(self, other: Self) -> Self {
        array_add(self, other)
    }

    /// Subtracts two arrays. Returns x - y.
    fn array_sub(self, other: Self) -> Self {
        array_sub(self, other)
    }

    /// Scale one array with a value.
    fn array_scale(self, value: F) -> Self {
        [self[0] * value, self[1] * value, self[2] * value]
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
}
// test:1 ends here
