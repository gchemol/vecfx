//! Backend for vector operations

// imports

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*imports][imports:1]]
use std::f64;
// imports:1 ends here

// mods

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*mods][mods:1]]
#[cfg(feature = "nalgebra")]
mod projection;

mod stats;
mod vector;
// mods:1 ends here

// pub

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*pub][pub:1]]
pub use crate::stats::*;

#[cfg(feature = "nalgebra")]
pub use projection::*;

pub use crate::vector::*;
// pub:1 ends here

// for Iterator<Item=f64>
// Adopted from:
// [[doi:][https://www.reddit.com/r/rust/comments/3fg0xr/how_do_i_find_the_max_value_in_a_vecf64/ctoaxna?utm_source=share&utm_medium=web2x]]


// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*for%20Iterator<Item=f64>][for Iterator<Item=f64>:1]]
pub trait FloatIterExt {
    fn float_min(&mut self) -> f64;
    fn float_max(&mut self) -> f64;
}

impl<T> FloatIterExt for T
where
    T: Iterator<Item = f64>,
{
    fn float_max(&mut self) -> f64 {
        self.fold(f64::NAN, f64::max)
    }

    fn float_min(&mut self) -> f64 {
        self.fold(f64::NAN, f64::min)
    }
}

#[test]
fn test_float_iter_min_max() {
    let x = vec![1.0f64, 2.0, 0.0, -9.0, 0.0 / 0.0];

    assert_eq!(x.iter().cloned().float_max(), 2.0);
    assert_eq!(x.iter().cloned().float_min(), -9.0);
}
// for Iterator<Item=f64>:1 ends here
