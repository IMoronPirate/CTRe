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

//! Generate a simple KDE estimation
use num::{Float, ToPrimitive};
use statrs::distribution::Continuous;

pub struct KDE<R, J>
where
    R: Continuous<f64, f64>,
    J: Float,
{
    /// The prior proability of points
    /// TODO expand to different priors, for now jsut the same for every point
    priors: f64,
    /// The kernel used for the KDE
    kernel: R,
    /// The bandwith of kernel
    bandwidth: f64,
    /// The data
    data: Vec<J>,
    // _marker: marker::PhantomData<T>,
}

impl<R, J> KDE<R, J>
where
    R: Continuous<f64, f64>,
    J: Float,
{
    pub fn _kde(data: &[J], kernel: R) -> Self {
        // Vec::from_iter(std::iter::repeat(1.0 / data.len()).take(data.len()));
        KDE {
            priors: 1.0 / (data.len() as f64),
            kernel,
            bandwidth: silverman_bandwith(data),
            data: data.to_vec(),
        }
    }

    pub fn kde(data: &[J], kernel: R, bandwidth: J) -> Self {
			// Vec::from_iter(std::iter::repeat(1.0 / data.len()).take(data.len()));
			KDE {
					priors: 1.0 / (data.len() as f64),
					kernel,
					bandwidth: ToPrimitive::to_f64(&bandwidth).unwrap(),
					data: data.to_vec(),
			}
	}

    pub fn pdf(&self, x: f64) -> f64 {
        self.data
            .iter()
            .map(|xi| {
                (self.priors / self.bandwidth)
                    * self
                        .kernel
                        .pdf((x - ToPrimitive::to_f64(xi).unwrap()) / self.bandwidth)
            })
            .reduce(|acc, f| acc + f)
            .unwrap()
    }
}


/// Silverman's rule of thumb for KDE bandwidth selection
pub(super) fn silverman_bandwith<J: Float>(data: &[J]) -> f64 {
	let alpha = 0.9;
	let n = data.len();

	// Calculate width using variance and IQR
	let variance = standard_deviation(data, None);
	todo!()
}

/// The mean is the sum of a collection of numbers divided by the number of numbers in the collection.
/// (reference)[http://en.wikipedia.org/wiki/Arithmetic_mean]
pub fn mean<T>(v: &[T]) -> T
    where T: Float
{
    let len = num::cast(v.len()).unwrap();
    v.iter().fold(T::zero(), |acc: T, elem| acc + *elem) / len
}

/// (Sample variance)[http://en.wikipedia.org/wiki/Variance#Sample_variance]
pub fn variance<T>(v: &[T], xbar: Option<T>) -> T
    where T: Float
{
    assert!(v.len() > 1, "variance requires at least two data points");
    let len: T = num::cast(v.len()).unwrap();
    let sum = sum_square_deviations(v, xbar);
    sum / (len - T::one())
}

///  Standard deviation is a measure that is used to quantify the amount of variation or
///  dispersion of a set of data values. (reference)[http://en.wikipedia.org/wiki/Standard_deviation]
pub fn standard_deviation<T>(v: &[T], xbar: Option<T>) -> T
    where T: Float
{
    let var = variance(v, xbar);
    var.sqrt()
}

fn sum_square_deviations<T>(v: &[T], c: Option<T>) -> T
    where T: Float
{
    let c = match c {
        Some(c) => c,
        None => mean(v),
    };

    let sum = v.iter().map( |x| (*x - c) * (*x - c) ).fold(T::zero(), |acc, elem| acc + elem);
    assert!(sum >= T::zero(), "negative sum of square root deviations");
    sum
}