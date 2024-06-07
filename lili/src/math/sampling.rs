//! This module provides functions and structures for various sampling techniques in computer graphics.

use super::{sqr, tuples::Point2f, Float, FloatExt};

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
            up.next_float_down()
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
                    u_remapped: ((up - sum) / *weight).min(Float::ONE_MINUS_EPSILON),
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
        2.0 * x.lerp(a, b) / (a + b)
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
        let x = u * (a + b) / (a + u.lerp(sqr(a), sqr(b)).sqrt());

        // Bound x to [0, 1) in the case of round off errors
        x.min(Float::ONE_MINUS_EPSILON)
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

/// Computes the probability density function (pdf) for the bilinear interpolation
///
/// # Arguments
///
/// * `p` - The point to compute the pdf for
/// * `w` - The weights for the four corners of the bilinear interpolation
///
/// # Returns
///
/// The value of the pdf at `p`
pub fn bilinear_pdf(p: Point2f, w: &[Float; 4]) -> Float {
    if p.x < 0.0 || p.x > 1.0 || p.y < 0.0 || p.y > 1.0 {
        0.0
    } else if w.iter().sum::<Float>() == 0.0 {
        1.0
    } else {
        4.0 * ((1.0 - p.x) * (1.0 - p.y) * w[0]
            + p.x * (1.0 - p.y) * w[1]
            + (1.0 - p.x) * p.y * w[2]
            + p.x * p.y * w[3])
            / (w.iter().sum::<Float>())
    }
}

pub fn sample_bilinear(u: Point2f, w: &[Float; 4]) -> Point2f {
    // Samples y from the bilinear marginal distribution
    let y = sample_linear(u.y, w[0] + w[1], w[2] + w[3]);
    Point2f::new(
        // Samples x from the bilinear conditional distribution
        sample_linear(u.x, y.lerp(w[0], w[2]), y.lerp(w[1], w[3])),
        y,
    )
}

pub fn invert_bilinear_sample(p: Point2f, w: &[Float; 4]) -> Point2f {
    Point2f::new(
        invert_linear_sample(p.x, p.y.lerp(w[0], w[2]), p.y.lerp(w[1], w[3])),
        invert_linear_sample(p.y, w[0] + w[1], w[2] + w[3]),
    )
}
