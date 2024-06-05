use super::{float::lerp, next_float_down, sqr, Float, ONE_MINUS_EPSILON};

/// Computes the balance heuristic for two distributions
///
/// This is a heuristic to weight the importance of distributions for
/// multiple importance sampling.
pub fn balance_heuristic(nf: u32, f_pdf: f32, ng: u32, g_pdf: f32) -> f32 {
    ((nf as f32) * f_pdf) / ((nf as f32) * f_pdf + (ng as f32) * g_pdf)
}

/// Computes the power heuristic for two distributions
///
/// This is a heuristic to weight the importance of distributions for
/// multiple importance sampling.
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

pub fn linear_pdf(x: Float, a: Float, b: Float) -> Float {
    if x < 0.0 || x > 1.0 {
        0.0
    } else {
        2.0 * lerp(x, a, b) / (a + b)
    }
}
