//! Backend for vector operations

// [[file:../vecfx.note::*imports][imports:1]]
use std::f64;
// imports:1 ends here

// [[file:../vecfx.note::*mods][mods:1]]
#[cfg(feature = "nalgebra")]
mod projection;
#[cfg(feature = "nalgebra")]
#[cfg(feature = "adhoc")]
mod rotation;

mod array;
mod iterator;
mod ord;
mod stats;
mod vector;
// mods:1 ends here

// [[file:../vecfx.note::*pub][pub:1]]
pub use crate::stats::*;

#[cfg(feature = "nalgebra")]
pub use projection::*;

pub use crate::vector::*;

#[cfg(feature = "nalgebra")]
pub use nalgebra;

pub use crate::iterator::FloatIterExt;

#[cfg(feature = "nalgebra")]
pub use crate::iterator::na::*;

#[cfg(feature = "nalgebra")]
// for using assert_relative_eq macro for Vector or Matrix
pub use approx;

pub use crate::ord::*;

pub use array::*;
// pub:1 ends here
