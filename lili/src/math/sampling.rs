//! This module provides functions and structures for various sampling techniques in computer graphics.

use super::{float::lerp, next_float_down, sqr, Float, ONE_MINUS_EPSILON};

/// Computes the balance heuristic for two distributions
///
/// This is a heuristic to weight the importance of distributions for
/// multiple importance sampling.
///
/// # Arguments
///
/// * `nf` - The number of samples from the first distribution
/// * `f_pdf` - The probability density function of the first distribution
/// * `ng` - The number of samples from the second distribution
/// * `g_pdf` - The probability density function of the second distribution
///
/// # Returns
///
/// The balance heuristic value
pub fn balance_heuristic(nf: u32, f_pdf: f32, ng: u32, g_pdf: f32) -> f32 {
    ((nf as f32) * f_pdf) / ((nf as f32) * f_pdf + (ng as f32) * g_pdf)
}

/// Computes the power heuristic for two distributions
///
/// This is a heuristic to weight the importance of distributions for
/// multiple importance sampling.
///
/// # Arguments
///
/// * `nf` - The number of samples from the first distribution
/// * `f_pdf` - The probability density function of the first distribution
/// * `ng` - The number of samples from the second distribution
/// * `g_pdf` - The probability density function of the second distribution
///
/// # Returns
///
/// The power heuristic value
///
/// # Remarks
///
/// Beta is chosen as 2
pub fn power_heuristic(nf: u32, f_pdf: f32, ng: u32, g_pdf: f32) -> f32 {
    let f = (nf as f32) * f_pdf;
    let g = (ng as f32) * g_pdf;

    sqr(f) / (sqr(f) + sqr(g))
}

#[derive(Default)]
pub struct DiscreteSample {
    sample: i32,
    pmf: Float,
    u_remapped: Float,
}

impl DiscreteSample {
    /// Creates a `DiscreteSample` from a list of weights and a random value `u`
    ///
    /// # Arguments
    ///
    /// * `weights` - The list of weights for each sample
    /// * `u` - The random value used for sampling
    ///
    /// # Returns
    ///
    /// A `DiscreteSample` object containing the sampled index, the probability mass function (pmf),
    /// and the remapped random value `u`
    pub fn sample_from_weights(weights: &[Float], u: Float) -> Self {
        if weights.is_empty() {
            return Self {
                sample: -1,
                pmf: 0.0,
                u_remapped: 0.0,
            };
        }

        let sum_weights = weights.iter().sum::<Float>();
        let up: Float = u * sum_weights; // scale by weights sum

        // u * sumWeights could be == sumWeights from fp rounding error
        // When that happens, bump down up to the next lowest fp so that
        // the follow code can assume up < sumWeights
        let up = if up == sum_weights {
            next_float_down(up)
        } else {
            up
        };

        let mut i = 0;
        let mut sum: Float = 0.0;
        for weight in weights {
            sum += *weight;
            if sum > up {
                return Self {
                    sample: i,
                    pmf: *weight / sum_weights,
                    u_remapped: ((up - sum) / *weight).min(ONE_MINUS_EPSILON),
                };
            }
            i += 1;
        }

        // If we get here, something went wrong
        #[cfg(debug_assertions)]
        {
            panic!("sample_discrete() failed to sample an index");
        }
    }
}

/// Computes the probability density function (pdf) for a linear distribution
///
/// # Arguments
///
/// * `x` - The value to compute the pdf for
/// * `a` - The start of the linear distribution
/// * `b` - The end of the linear distribution
///
/// # Returns
///
/// The value of the pdf at `x`
pub fn linear_pdf(x: Float, a: Float, b: Float) -> Float {
    if x < 0.0 || x > 1.0 {
        0.0
    } else {
        2.0 * lerp(x, a, b) / (a + b)
    }
}

/// Samples a value from a linear distribution
///
/// # Arguments
///
/// * `u` - The random value used for sampling
/// * `a` - The start of the linear distribution
/// * `b` - The end of the linear distribution
///
/// # Returns
///
/// The sampled value
pub fn sample_linear(u: Float, a: Float, b: Float) -> Float {
    if u == 0.0 && a == 0.0 {
        0.0
    } else {
        let x = u * (a + b) / (a + lerp(u, sqr(a), sqr(b)).sqrt());

        // Bound x to [0, 1) in the case of round off errors
        x.min(ONE_MINUS_EPSILON)
    }
}

/// Inverts a sample from a linear distribution
///
/// # Arguments
///
/// * `x` - The sampled value
/// * `a` - The start of the linear distribution
/// * `b` - The end of the linear distribution
///
/// # Returns
///
/// The inverted sample value, which is the random sample `xi` that generates the value x
pub fn invert_linear_sample(x: Float, a: Float, b: Float) -> Float {
    x * (a * (2.0 - x) + b * x) / (a + b)
}
