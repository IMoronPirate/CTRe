// Copyright (C) 2024 I, moron pirate
//
// This file is part of clinical_trial_risk.
//
// clinical_trial_risk is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// clinical_trial_risk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with clinical_trial_risk.  If not, see <https://www.gnu.org/licenses/>.

//! Basic statiscs for rust

use num::Float;

/// The mean is the sum of a collection of numbers divided by the number of numbers in the collection.
/// (reference)[http://en.wikipedia.org/wiki/Arithmetic_mean]
pub fn mean<T>(v: &[T]) -> T
where
    T: Float,
{
    let len = num::cast(v.len()).unwrap();
    v.iter().fold(T::zero(), |acc: T, elem| acc + *elem) / len
}

/// (Sample variance)[http://en.wikipedia.org/wiki/Variance#Sample_variance]
pub fn variance<T>(v: &[T], xbar: Option<T>) -> T
where
    T: Float,
{
    assert!(v.len() > 1, "variance requires at least two data points");
    let len: T = num::cast(v.len()).unwrap();
    let sum = sum_square_deviations(v, xbar);
    sum / (len - T::one())
}

///  Standard deviation is a measure that is used to quantify the amount of variation or
///  dispersion of a set of data values. (reference)[http://en.wikipedia.org/wiki/Standard_deviation]
pub fn standard_deviation<T>(v: &[T], xbar: Option<T>) -> T
where
    T: Float,
{
    let var = variance(v, xbar);
    var.sqrt()
}

fn sum_square_deviations<T>(v: &[T], c: Option<T>) -> T
where
    T: Float,
{
    let c = match c {
        Some(c) => c,
        None => mean(v),
    };

    let sum = v
        .iter()
        .map(|x| (*x - c) * (*x - c))
        .fold(T::zero(), |acc, elem| acc + elem);
    assert!(sum >= T::zero(), "negative sum of square root deviations");
    sum
}

/// Quantile function calculate as Definition 8 in
/// “Sample Quantile in Statistical Packages,” Current Biology, vol. 7, no. 3, p. R126, Mar. 1997, doi: 10.1016/S0960-9822(97)70976-X.
pub fn quantile<T>(v: &[T], p: T) -> T
where
    T: Float,
{
    let t0 = T::zero();
    let t1 = T::one();
    assert!(p >= t0, "p must be in [0, 1]");
    assert!(p <= t1, "p must be in [0, 1]");
    let n = num::cast(v.len()).unwrap();
		// Implementation similar to numpy.quantile
		//https://numpy.org/doc/stable/reference/generated/numpy.quantile.html
    let m = p / num::cast(3).unwrap() + num::cast(1.0 / 3.0).unwrap();
    let f = p * n + m - t1;
    let j = f.floor();
    let gamma = (p * n + m - t1) - j;

    // Get order statistics sorting the vec
		let mut ord_stat = vec![T::zero(); v.len()];
    ord_stat.copy_from_slice(v);
		ord_stat.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if j < t0 {
        v[0]
    } else if j >= (n - t1) {
        let j_max: usize = num::cast(n - t1).unwrap();
        v[j_max]
    } else {
        let ju: usize = num::cast(j).unwrap();
        (t1 - gamma) * v[ju] + gamma * v[ju + 1]
    }
}

#[cfg(test)]
mod tests {
    use statrs::assert_almost_eq;

    use super::*;

    #[test]
    fn check_quantile() {
        let mut v = vec![0.5, 0.6, 0.7, 0.8];
        // julia> quantile([0.5, 0.6, 0.7, 0.8], 0.45, alpha=1/3, beta=1/3) # = 0.6283333333333333
        assert_almost_eq!(quantile(&mut v, 0.45), 0.6283333333333333, 1e-10);
        assert_almost_eq!(quantile(&mut v, 0.2), 0.52, 1e-10);
        assert_almost_eq!(quantile(&mut v, 0.), 0.5, 1e-10);
				assert_almost_eq!(quantile(&mut v, 1.), 0.8, 1e-10);
    }
}
