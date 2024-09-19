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
use crate::statsbase::{quantile, standard_deviation};
use num::{Float, ToPrimitive};
use statrs::distribution::Continuous;

pub struct KDE<R, J>
where
    R: Continuous<J, J> + Clone,
    J: Float,
{
    /// The prior proability of points
    /// TODO expand to different priors, for now jsut the same for every point
    priors: Vec<J>,
    /// The kernels used for the KDE
    kernels: Vec<R>,
}

impl<R, J> KDE<R, J>
where
    R: Continuous<J, J> + Clone,
    J: Float,
{
    pub fn new(priors: &[J], kernels: &[R]) -> Self {
        // Vec::from_iter(std::iter::repeat(1.0 / data.len()).take(data.len()));
        assert_eq!(
            priors
                .into_iter()
                .map(|x| ToPrimitive::to_f64(x).unwrap())
                .sum::<f64>(),
            1.0,
            "Priors do not sum to one"
        );
        KDE {
            priors: priors.to_vec(),
            kernels: kernels.to_vec(),
        }
    }

    // pub fn with_priors(data: &[J], kernel: R, bandwidth: J) -> Self {
    //     // Vec::from_iter(std::iter::repeat(1.0 / data.len()).take(data.len()));
    //     KDE {
    //         priors: 1.0 / (data.len() as f64),
    //         kernel,
    //         bandwidth,
    //         data: data.to_vec(),
    //     }
    // }

    pub fn pdf(&self, x: J) -> J {
        self.kernels
            .iter()
            .zip(self.priors.iter())
            .map(|(f, p)| *p * f.pdf(x))
            .reduce(|acc, f| acc + f)
            .unwrap()
    }
}

/// Silverman's rule of thumb for KDE bandwidth selection
pub fn silverman_bandwith<J: Float>(data: &[J]) -> J {
    let alpha: J = num::cast(0.9).unwrap();
    let n: J = num::cast(data.len()).unwrap();

    // Calculate width using variance and IQR
    let variance = standard_deviation(data, None);
    let iqr = quantile(data, num::cast(0.75).unwrap()) - quantile(data, num::cast(0.25).unwrap());

    alpha * variance.min(iqr / num::cast(1.34).unwrap()) * n.powf(num::cast(-1. / 5.).unwrap())
}

impl<R, J> ::rand::distributions::Distribution<f64> for KDE<R, J>
where
    R: Continuous<J, J> + Clone,
    J: Float,
{
    fn sample<T: rand::Rng + ?Sized>(&self, rng: &mut T) -> f64 {
        todo!()
        // self.data.map(|xi| {
        //     (self.priors / self.bandwidth) * rnd.sample(self.kernel)
        // })
    }
}
