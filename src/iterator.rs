// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*na matrix][na matrix:1]]
#[cfg(feature = "nalgebra")]
pub(crate) mod na {
    use crate::nalgebra as na;
    use crate::vector::*;
    use crate::Matrix3f;

    pub trait FloatIterToVector {
        fn to_vector(self) -> na::DVector<f64>;
    }

    pub trait FloatIterToMatrix {
        fn to_matrix(self) -> Vector3fVec;
    }

    impl<T, F> FloatIterToVector for T
    where
        T: Iterator<Item = F>,
        F: std::borrow::Borrow<f64>,
    {
        /// Convert an iterator over floats as Nalgebra column vector
        fn to_vector(self) -> na::DVector<f64> {
            let d: Vec<f64> = self.map(|x| *x.borrow()).collect();
            d.to_column_vector()
        }
    }

    impl<F, T> FloatIterToMatrix for T
    where
        T: Iterator<Item = F>,
        F: std::borrow::Borrow<[f64; 3]>,
    {
        /// Convert an iterator over float array as Nalgebra 3xN matrix
        fn to_matrix(self) -> Vector3fVec {
            let d: Vec<[f64; 3]> = self.map(|x| *x.borrow()).collect();
            d.to_matrix()
        }
    }

    #[test]
    fn test_float_iter_trait() {
        let d = vec![1f64, 2.0, 3.0];
        let _ = d.iter().to_vector();
        let _ = d.into_iter().to_vector();

        let d = vec![[1f64; 3], [2.0; 3], [3.0; 3]];
        let _ = d.iter().to_matrix();
        let _ = d.into_iter().to_matrix();
    }
}
// na matrix:1 ends here

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*impl][impl:1]]
use std::borrow::Borrow;
use std::f64;

// https://stackoverflow.com/questions/43921436/extend-iterator-with-a-mean-method
/// max/min for iterator over floats.
///
/// # Example
/// ```
/// use vecfx::*;
/// 
/// let values = vec![1.0, 2.1, -1.1];
/// assert_eq!(values.iter().float_max(), 2.1);
/// assert_eq!(values.iter().float_min(), -1.1);
/// 
/// let values = vec![1.2, 0.0/0.0, 2.2];
/// assert_eq!(values.iter().float_max(), 2.2);
/// assert_eq!(values.iter().float_min(), 1.2);
/// ```
///
pub trait FloatIterExt<F>: Iterator<Item = F>
where
    F: std::borrow::Borrow<f64>,
{
    /// Returns the maximum element of an iterator.
    ///
    /// # Panic
    ///
    /// * panics if iterator is empty.
    fn float_max(&mut self) -> f64 {
        let value = *self.next().expect("float max: empty iterator").borrow();
        self.fold(value, |a, b| a.max(*b.borrow()))
    }

    /// Returns the minimum element of an iterator.
    ///
    /// # Panic
    ///
    /// * panics if iterator is empty.
    fn float_min(&mut self) -> f64 {
        let value = *self.next().expect("float min: empty iterator").borrow();
        self.fold(value, |a, b| a.min(*b.borrow()))
    }
}

impl<F, T> FloatIterExt<F> for T
where
    T: Iterator<Item = F>,
    F: std::borrow::Borrow<f64>,
{
}
// impl:1 ends here

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*test][test:1]]
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_float_ordering() {
        let values = vec![1.0, -1.0, 0.0 / 0.0, 0.5, 2.0];
        assert_eq!(values.iter().float_max(), values.max());
        assert_eq!(values.iter().float_min(), values.min());

        let values = vec![1.0, -1.0, 0.0 / 0.0, 0.5, 2.0];
        assert_eq!(values.iter().float_max(), values.max());
        assert_eq!(values.iter().float_min(), values.min());

        // test for compatibility
        let values = vec![1.0f64, 2.0, 0.0, -9.0, 0.0 / 0.0];
        assert_eq!(values.iter().cloned().float_max(), values.max());
        assert_eq!(values.iter().cloned().float_min(), values.min());
    }

    #[test]
    #[should_panic]
    fn test_float_min_max_empty() {
        let values: Vec<f64> = Vec::new();
        let _ = values.iter().float_max();
    }
}
// test:1 ends here
