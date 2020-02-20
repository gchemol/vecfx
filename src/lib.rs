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
mod iterator;
// mods:1 ends here

// pub

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*pub][pub:1]]
pub use crate::stats::*;

#[cfg(feature = "nalgebra")]
pub use projection::*;

pub use crate::vector::*;

#[cfg(feature = "nalgebra")]
pub use nalgebra;

pub use crate::iterator::FloatIterExt;
// pub:1 ends here
