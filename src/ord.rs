// [[file:../vecfx.note::*impl][impl:1]]
use std::borrow::Borrow;

pub use ordered_float::{Float, OrderedFloat};

// Not very useful
trait IntoOrderedExt {
    fn into_ordered(self) -> Vec<OrderedFloat<f64>>;
}

impl<F, T> IntoOrderedExt for T
where
    T: Iterator<Item = F>,
    F: std::borrow::Borrow<f64>,
{
    /// Convert an iterator over floats into a vec of ordered floats
    fn into_ordered(self) -> Vec<OrderedFloat<f64>> {
        self.map(|x| OrderedFloat(*x.borrow())).collect()
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
// impl:1 ends here

// [[file:../vecfx.note::*test][test:1]]
#[test]
fn test_float_ord() {
    let mut values = vec![1.0, -1.0, 2.0];
    let mut values_ordered = values.iter().into_ordered();
    values_ordered.sort();

    assert_eq!(values_ordered[0], -1.0);
    assert_eq!(values_ordered[2], 2.0);

    values.sort_by_float();
    assert_eq!(values[0], values_ordered[0].into());
    assert_eq!(values[1], values_ordered[1].into());
}
// test:1 ends here
