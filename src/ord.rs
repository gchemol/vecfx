// [[file:../vecfx.note::be82ddff][be82ddff]]
use std::borrow::Borrow;

pub use ordered_float::{Float, OrderedFloat};

/// Convert a float number to OrderedFloat type
pub trait AsOrderedFloatExt<F>
where
    F: Float,
{
    fn as_ordered_float(self) -> OrderedFloat<F>;
}

impl<F> AsOrderedFloatExt<F> for F
where
    F: Float,
{
    /// Convert a float number to OrderedFloat type
    fn as_ordered_float(self) -> OrderedFloat<F> {
        OrderedFloat(self)
    }
}

/// Provides method for sorting a vec of floats
pub trait SortByExt {
    fn sort_by_float(&mut self);
}

impl<F: Float> SortByExt for [F] {
    /// Sort by float numbers
    fn sort_by_float(&mut self) {
        self.sort_by_key(|x| OrderedFloat(*x));
    }
}
// be82ddff ends here

// [[file:../vecfx.note::edd3e54f][edd3e54f]]
#[test]
fn test_float_ord() {
    let mut values = vec![1.0, -1.0, 2.0];
    let mut values_ordered: Vec<_> = values.iter().map(|x| x.as_ordered_float()).collect();
    values_ordered.sort();
    assert_eq!(values_ordered[0], -1.0);
    assert_eq!(values_ordered[2], 2.0);

    values.sort_by_float();
    assert_eq!(values[0], values_ordered[0].into());
    assert_eq!(values[1], values_ordered[1].into());
}
// edd3e54f ends here
