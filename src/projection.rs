// imports

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*imports][imports:1]]
use crate::{VecFloatExt, Vector3fVec};

use nalgebra as na;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*base][base:1]]
pub trait VectorProject {
    fn cosine_similarity(&self, vb: &Self) -> f64;

    fn scalar_projection(&self, vb: &Self) -> f64;

    fn vector_projection(&self, vb: &Self) -> Self;

    fn vector_rejection(&self, vb: &Self) -> Self;
}
// base:1 ends here

// core

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*core][core:1]]
macro_rules! impl_vector_project {
    ($type_name:ty) => {
        impl VectorProject for $type_name {
            /// The cosine of two non-zero vectors `va` and `vb`
            fn cosine_similarity(&self, vb: &Self) -> f64 {
                self.normalize().dot(&vb.normalize())
            }

            /// the scalar projection of a vector a onto a vector b
            fn scalar_projection(&self, vb: &Self) -> f64 {
                self.cosine_similarity(vb) * &self.norm()
            }

            /// The vector component of `va` parallel to `vb`
            fn vector_projection(&self, vb: &Self) -> Self {
                self.scalar_projection(vb) * vb.normalize()
            }

            /// The vector component of `va` perpendicular to `vb`
            fn vector_rejection(&self, vb: &Self) -> Self {
                self - &self.vector_projection(vb)
            }
        }
    };
}

impl_vector_project!(na::DVector<f64>);
impl_vector_project!(Vector3fVec);
// core:1 ends here

// test

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*test][test:1]]
#[test]
fn test_vector_projection() {
    use approx::*;

    let va = [1.0; 3].to_vector();
    let vb = [1.1, 1.2, 1.3].to_vector();

    let x = va.cosine_similarity(&vb);
    assert_relative_eq!(x, 0.9976931918526477, epsilon=1e-4);

    let x = va.scalar_projection(&vb);
    assert_relative_eq!(x, 1.728055, epsilon=1e-4);

    let x = va.vector_projection(&vb);
    let e = [0.9124424 , 0.99539171, 1.07834101];
    for i in 0..3 {
        assert_relative_eq!(x[i], e[i], epsilon=1e-4);
    }

    let x = va.vector_rejection(&vb);
    let e = [ 0.0875576 ,  0.00460829, -0.07834101];
    for i in 0..3 {
        assert_relative_eq!(x[i], e[i], epsilon=1e-4);
    }
}
// test:1 ends here
