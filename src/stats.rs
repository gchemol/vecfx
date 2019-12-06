// stats
// abstracted from: https://github.com/rust-lang/libtest/blob/master/libtest/stats.rs

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*stats][stats:1]]
/// Trait that provides simple descriptive statistics on a univariate set of numeric samples.
pub trait StatsExt {
    /// Sum of the samples.
    ///
    /// Note: this method sacrifices performance at the altar of accuracy
    /// Depends on IEEE-754 arithmetic guarantees. See proof of correctness at:
    /// ["Adaptive Precision Floating-Point Arithmetic and Fast Robust Geometric
    /// Predicates"][paper]
    ///
    /// [paper]: http://www.cs.cmu.edu/~quake-papers/robust-arithmetic.ps
    fn sum(&self) -> f64;

    /// Minimum value of the samples.
    fn min(&self) -> f64;

    /// Maximum value of the samples.
    fn max(&self) -> f64;

    /// Arithmetic mean (average) of the samples: sum divided by sample-count.
    ///
    /// See: <https://en.wikipedia.org/wiki/Arithmetic_mean>
    fn mean(&self) -> f64;

    /// Variance of the samples: bias-corrected mean of the squares of the differences of each
    /// sample from the sample mean. Note that this calculates the _sample variance_ rather than the
    /// population variance, which is assumed to be unknown. It therefore corrects the `(n-1)/n`
    /// bias that would appear if we calculated a population variance, by dividing by `(n-1)` rather
    /// than `n`.
    ///
    /// See: <https://en.wikipedia.org/wiki/Variance>
    fn var(&self) -> f64;

    /// Standard deviation: the square root of the sample variance.
    ///
    /// Note: this is not a robust statistic for non-normal distributions. Prefer the
    /// `median_abs_dev` for unknown distributions.
    ///
    /// See: <https://en.wikipedia.org/wiki/Standard_deviation>
    fn std_dev(&self) -> f64;

    /// Index to the minimum value of the samples.
    fn imin(&self) -> usize;

    /// Index to the maximum value of the samples.
    fn imax(&self) -> usize;
}

impl StatsExt for [f64] {
    // FIXME #11059 handle NaN, inf and overflow
    fn sum(&self) -> f64 {
        let mut partials = vec![];

        for &x in self {
            let mut x = x;
            let mut j = 0;
            // This inner loop applies `hi`/`lo` summation to each
            // partial so that the list of partial sums remains exact.
            for i in 0..partials.len() {
                let mut y: f64 = partials[i];
                if x.abs() < y.abs() {
                    std::mem::swap(&mut x, &mut y);
                }
                // Rounded `x+y` is stored in `hi` with round-off stored in
                // `lo`. Together `hi+lo` are exactly equal to `x+y`.
                let hi = x + y;
                let lo = y - (hi - x);
                if lo != 0.0 {
                    partials[j] = lo;
                    j += 1;
                }
                x = hi;
            }
            if j >= partials.len() {
                partials.push(x);
            } else {
                partials[j] = x;
                partials.truncate(j + 1);
            }
        }
        let zero: f64 = 0.0;
        partials.iter().fold(zero, |p, q| p + *q)
    }

    fn min(&self) -> f64 {
        assert!(!self.is_empty());
        self.iter().fold(self[0], |p, q| p.min(*q))
    }

    fn max(&self) -> f64 {
        assert!(!self.is_empty());
        self.iter().fold(self[0], |p, q| p.max(*q))
    }

    fn mean(&self) -> f64 {
        assert!(!self.is_empty());
        self.sum() / (self.len() as f64)
    }

    fn var(&self) -> f64 {
        if self.len() < 2 {
            0.0
        } else {
            let mean = self.mean();
            let mut v: f64 = 0.0;
            for s in self {
                let x = *s - mean;
                v += x * x;
            }
            // N.B., this is _supposed to be_ len-1, not len. If you
            // change it back to len, you will be calculating a
            // population variance, not a sample variance.
            let denom = (self.len() - 1) as f64;
            v / denom
        }
    }

    fn std_dev(&self) -> f64 {
        self.var().sqrt()
    }

    fn imin(&self) -> usize {
        assert!(!self.is_empty());
        self.iter()
            .enumerate()
            .fold(0, |i, (j, q)| if self[i] > *q { j } else { i })
    }

    fn imax(&self) -> usize {
        assert!(!self.is_empty());
        self.iter()
            .enumerate()
            .fold(0, |i, (j, q)| if self[i] > *q { i } else { j })
    }
}
// stats:1 ends here

// test

// [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*test][test:1]]
#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_stats_nan() {
        let xs = &[1.0, 2.0, std::f64::NAN, 3.0, 4.0];
        assert_eq!(xs.min(), 1.0);
        assert_eq!(xs.max(), 4.0);
        assert_eq!(xs.imin(), 0);
        assert_eq!(xs.imax(), 4);
        assert!(xs.sum().is_nan());
        assert!(xs.mean().is_nan());
    }

    #[test]
    fn test_stats() {
        let val = &[958.0000000000, 924.0000000000];
        assert_relative_eq!(val.sum(), 1882.0);
        assert_relative_eq!(val.min(), 924.0);
        assert_relative_eq!(val.max(), 958.0);
        assert_relative_eq!(val.mean(), 941.0);
        assert_relative_eq!(val.var(), 578.0);
        assert_relative_eq!(val.std_dev(), 24.0416305603, epsilon = 1e-6);
        assert_eq!(val.imin(), 1);
        assert_eq!(val.imax(), 0);
    }
}
// test:1 ends here
