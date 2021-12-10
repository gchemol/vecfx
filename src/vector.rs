// [[file:../vecfx.note::*imports][imports:1]]
#[cfg(feature = "nalgebra")]
use nalgebra as na;
// imports:1 ends here

// [[file:../vecfx.note::*types][types:1]]
#[cfg(feature = "nalgebra")]
/// 3xN matrix storing a list of 3D vectors
pub type Vector3fVec =
    na::Matrix<f64, na::U3, na::Dynamic, na::VecStorage<f64, na::U3, na::Dynamic>>;

#[cfg(feature = "nalgebra")]
/// A stack-allocated, 3-dimensional column vector.
pub type Vector3f = na::Vector3<f64>;

#[cfg(feature = "nalgebra")]
/// A stack-allocated, column-major, 3x3 square matrix
pub type Matrix3f = na::Matrix3<f64>;

// #[cfg(feature = "nalgebra")]
// /// A dynamically sized column vector
// pub type Vector = na::DVector<f64>;
// types:1 ends here

// [[file:../vecfx.note::*for Vec<f64>][for Vec<f64>:1]]
/// Abstracting simple vector based math operations
pub trait VecFloatExt {
    /// y += c*x
    fn vecadd(&mut self, x: &[f64], c: f64);

    /// vector dot product
    /// s = x.dot(y)
    fn vecdot(&self, other: &[f64]) -> f64;

    /// y = z
    fn veccpy(&mut self, x: &[f64]);

    /// y = -x
    fn vecncpy(&mut self, x: &[f64]);

    /// z = x - y
    fn vecdiff(&mut self, x: &[f64], y: &[f64]);

    /// y *= c
    fn vecscale(&mut self, c: f64);

    /// ||x||, L2 norm
    fn vec2norm(&self) -> f64;

    /// 1 / ||x||
    fn vec2norminv(&self) -> f64;

    /// d = ||a-b||
    fn vecdist(&self, other: &[f64]) -> f64 {
        self.vecdist_squared(other).sqrt()
    }

    /// d = ||a-b||^2
    fn vecdist_squared(&self, other: &[f64]) -> f64;

    #[cfg(feature = "nalgebra")]
    /// Create dynamically allocated column vector from self
    fn to_column_vector(&self) -> na::DVector<f64>;

    #[cfg(feature = "nalgebra")]
    /// Create dynamically allocated column vector from self
    fn to_vector(&self) -> na::DVector<f64> {
        self.to_column_vector()
    }
}

impl VecFloatExt for [f64] {
    /// y += c*x
    fn vecadd(&mut self, x: &[f64], c: f64) {
        for (y, x) in self.iter_mut().zip(x) {
            *y += c * x;
        }
    }

    /// s = y.dot(x)
    fn vecdot(&self, other: &[f64]) -> f64 {
        self.iter().zip(other).map(|(x, y)| x * y).sum()
    }

    /// y *= c
    fn vecscale(&mut self, c: f64) {
        for y in self.iter_mut() {
            *y *= c;
        }
    }

    /// y = x
    fn veccpy(&mut self, x: &[f64]) {
        for (v, x) in self.iter_mut().zip(x) {
            *v = *x;
        }
    }

    /// y = -x
    fn vecncpy(&mut self, x: &[f64]) {
        for (v, x) in self.iter_mut().zip(x) {
            *v = -x;
        }
    }

    /// z = x - y
    fn vecdiff(&mut self, x: &[f64], y: &[f64]) {
        for ((z, x), y) in self.iter_mut().zip(x).zip(y) {
            *z = x - y;
        }
    }

    /// ||x||, L2 norm
    fn vec2norm(&self) -> f64 {
        let n2 = self.vecdot(&self);
        n2.sqrt()
    }

    /// 1/||x||
    fn vec2norminv(&self) -> f64 {
        1.0 / self.vec2norm()
    }

    /// d = ||a-b||^2
    fn vecdist_squared(&self, other: &[f64]) -> f64 {
        self.iter().zip(other).map(|(a, b)| (a - b).powi(2)).sum::<f64>()
    }

    #[cfg(feature = "nalgebra")]
    fn to_column_vector(&self) -> na::DVector<f64> {
        na::DVector::from_column_slice(&self)
    }
}

#[test]
fn test_vec_math() {
    use approx::*;

    // vector scaled add
    let x = [1.0, 1.0, 1.0];
    let c = 2.;

    let mut y = [1.0, 2.0, 3.0];
    y.vecadd(&x, c);

    assert_eq!(3.0, y[0]);
    assert_eq!(4.0, y[1]);
    assert_eq!(5.0, y[2]);

    // vector dot
    let v = y.vecdot(&x);
    assert_eq!(12.0, v);

    // vector scale
    y.vecscale(2.0);
    assert_eq!(6.0, y[0]);
    assert_eq!(8.0, y[1]);
    assert_eq!(10.0, y[2]);

    // vector diff
    let mut z = y.clone();
    z.vecdiff(&x, &y);
    assert_eq!(-5.0, z[0]);
    assert_eq!(-7.0, z[1]);
    assert_eq!(-9.0, z[2]);

    // vector copy
    y.veccpy(&x);

    // y = -x
    y.vecncpy(&x);
    assert_eq!(-1.0, y[0]);
    assert_eq!(-1.0, y[1]);
    assert_eq!(-1.0, y[2]);

    // vector distance
    let x = [0.0, 0.0, 0.0];
    let y = [1.0, 1.0, 1.0];
    let r = x.vecdist(&y);
    assert_relative_eq!(r, y.vec2norm(), epsilon = 1e-4);
}

#[cfg(feature = "nalgebra")]
#[test]
fn test_vec_math_na() {
    // nalgebra
    let y = [-1.0; 3];
    let v = y.to_column_vector();
    assert_eq!(v.norm_squared(), 3.0);
}

/// View a flat slice as nested 3D array
///
/// # Panics
/// if the slice size is incorrect.
pub trait VecFloatAs3D {
    /// View `&[f64]` as `&[[f64; 3]]` without copying.
    fn as_3d(&self) -> &[[f64; 3]];

    /// View `&mut [f64]` as `&mut [[f64; 3]]` without copying.
    fn as_mut_3d(&mut self) -> &mut [[f64; 3]];
}

impl VecFloatAs3D for [f64] {
    fn as_3d(&self) -> &[[f64; 3]] {
        assert_eq!(0, self.len() % 3, "cannot view slice of length {} as &[[_; 3]]", self.len());
        unsafe { ::std::slice::from_raw_parts(self.as_ptr() as *const _, self.len() / 3) }
    }

    fn as_mut_3d(&mut self) -> &mut [[f64; 3]] {
        assert_eq!(0, self.len() % 3, "cannot view slice of length {} as &[[_; 3]]", self.len());
        unsafe { ::std::slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() / 3) }
    }
}
// for Vec<f64>:1 ends here

// [[file:../vecfx.note::*for Vec<\[f64; 3\]>][for Vec<[f64; 3]>:1]]
pub trait VecFloat3Ext {
    /// Return a 1-D array, containing the elements of 3xN array
    fn ravel(&self) -> Vec<f64> {
        self.as_flat().to_vec()
    }

    /// View as a flat slice
    fn as_flat(&self) -> &[f64];

    /// View of mut flat slice
    fn as_mut_flat(&mut self) -> &mut [f64];

    #[cfg(feature = "nalgebra")]
    /// Create a 3xN matrix of nalgebra from self
    fn to_matrix(&self) -> Vector3fVec;

    #[cfg(feature = "nalgebra")]
    #[cfg(feature = "adhoc")]
    /// Return distance matrix
    fn distance_matrix(&self) -> na::DMatrix<f64>;
}

impl VecFloat3Ext for [[f64; 3]] {
    /// View as a flat slice
    fn as_flat(&self) -> &[f64] {
        unsafe { ::std::slice::from_raw_parts(self.as_ptr() as *const _, self.len() * 3) }
    }

    /// View of mut flat slice
    fn as_mut_flat(&mut self) -> &mut [f64] {
        unsafe { ::std::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() * 3) }
    }

    #[cfg(feature = "nalgebra")]
    /// Create a 3xN matrix of nalgebra from self
    fn to_matrix(&self) -> Vector3fVec {
        let r = self.as_flat();
        Vector3fVec::from_column_slice(r)
    }

    /// Return distance matrix
    #[cfg(feature = "nalgebra")]
    #[cfg(feature = "adhoc")]
    fn distance_matrix(&self) -> na::DMatrix<f64> {
        let n = self.len();

        let mut distances = na::DMatrix::zeros(n, n);
        for i in 0..n {
            for j in 0..i {
                let d = self[i].vecdist(&self[j]);
                distances[(i, j)] = d;
                distances[(j, i)] = d;
            }
        }

        distances
    }
}

#[test]
fn test_vecf3() {
    use approx::*;

    #[cfg(feature = "nalgebra")]
    {
        let a = vec![1.0, 2.0, 3.0];
        let x = a.to_vector();
        assert_relative_eq!(x.norm(), a.vec2norm(), epsilon = 1e-3);
    }

    let positions = [
        [-0.131944, -0.282942, 0.315957],
        [0.40122, -1.210646, 0.315957],
        [-1.201944, -0.282942, 0.315957],
        [0.543331, 0.892036, 0.315957],
        [0.010167, 1.819741, 0.315957],
        [1.613331, 0.892036, 0.315957],
    ];

    #[cfg(feature = "nalgebra")]
    {
        let n = positions.as_flat().vec2norm();
        let m = positions.to_matrix();
        assert_relative_eq!(n, m.norm(), epsilon = 1e-4);
    }

    let x = positions.ravel();
    assert_eq!(positions.len() * 3, x.len());

    let flat = positions.as_flat();
    assert_eq!(18, flat.len());

    let mut positions = positions.clone();
    let mflat = positions.as_mut_flat();
    mflat[0] = 0.0;
    assert_eq!(0.0, positions[0][0]);
}
// for Vec<[f64; 3]>:1 ends here

// [[file:../vecfx.note::*for Vec<\[f64; 3\]>][for Vec<[f64; 3]>:2]]
#[cfg(feature = "nalgebra")]
impl VecFloatAs3D for Vector3fVec {
    fn as_3d(&self) -> &[[f64; 3]] {
        assert_eq!(
            0,
            self.len() % 3,
            "cannot view Matrix of length {} as &[[_; 3]]",
            self.len()
        );

        self.as_slice().as_3d()
    }

    fn as_mut_3d(&mut self) -> &mut [[f64; 3]] {
        assert_eq!(
            0,
            self.len() % 3,
            "cannot view Matrix of length {} as &[[_; 3]]",
            self.len()
        );

        self.as_mut_slice().as_mut_3d()
    }
}

#[test]
fn test_as_3d() {
    let v = [1., 2., 3.];
    let p = v.as_3d();
    assert_eq!(&[[1., 2., 3.]], p);

    let mut v = vec![1., 2., 3., 4., 5., 6.];
    let p = &mut v.as_mut_3d();
    assert_eq!(p, &mut [[1., 2., 3.], [4., 5., 6.],]);
}

#[test]
#[cfg(feature = "nalgebra")]
fn test_as_3d_na() {
    let p = [1., 2., 3.];
    let mut m = p.as_3d().to_matrix();
    let _ = m.norm();

    let mut v = vec![[1., 2., 3.], [4., 5., 6.]].to_matrix();
    let mp = v.as_mut_3d();
    assert_eq!(mp, &mut [[1., 2., 3.], [4., 5., 6.],]);

    mp[0][0] = 1.1;
    assert_eq!(1.1, v[(0, 0)]);
}
// for Vec<[f64; 3]>:2 ends here

// [[file:../vecfx.note::4edfda94][4edfda94]]
#[cfg(feature = "nalgebra")]
mod slice {
    use nalgebra::DVectorSlice;
    use nalgebra::DVectorSliceMut;

    pub trait SliceNaExt {
        /// View of mut flat slice
        fn as_vector_slice(&self) -> DVectorSlice<f64>;
    }

    impl SliceNaExt for [f64] {
        fn as_vector_slice(&self) -> DVectorSlice<f64> {
            // NOTE: DVectorSlice::from does not work (nalgebra v0.29)
            DVectorSlice::from_slice(&self, self.len())
        }
    }
}
// 4edfda94 ends here

// [[file:../vecfx.note::db30f038][db30f038]]
#[cfg(feature = "nalgebra")]
pub use self::slice::*;
// db30f038 ends here

// [[file:../vecfx.note::85f5310c][85f5310c]]
#[cfg(feature = "nalgebra")]
#[test]
fn test_vector3f() {
    use approx::*;

    let a = Vector3f::from([1.0, 2.0, 3.0]);
    assert_eq!(a.x, 1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, 3.0);
    assert_eq!(a[0], 1.0);
    assert_eq!(a[1], 2.0);
    assert_eq!(a[2], 3.0);

    let b = Vector3f::from([1.0, 2.0, 3.0]);
    let c = a + b;
    assert_relative_eq!(c.sum(), 12.0);
}

#[test]
fn test_na_slice() {
    let x = [1.0, 2.0];
    let a = x.as_vector_slice();
    let x = [1.0, 2.0];
    let b = x.as_vector_slice();
    assert_eq!((b - a).norm(), 0.0);
}
// 85f5310c ends here
